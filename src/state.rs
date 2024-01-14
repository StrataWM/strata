use crate::{
	handlers::input::{
		KeyPattern,
		ModFlags,
		Mods,
	},
	workspaces::{
		FocusTarget,
		Workspaces,
	},
};
use gc_arena::{
	lock::RefLock,
	Rootable,
};
use piccolo::{
	closure::{
		UpValue,
		UpValueState,
	},
	meta_ops,
	Callback,
	CallbackReturn,
	Closure,
	Context,
	Executor,
	Lua,
	MetaMethod,
	StashedExecutor,
	StashedFunction,
	StashedUserData,
	Table,
	UserData,
	Value,
};
use smithay::{
	backend::{
		input::{
			AbsolutePositionEvent,
			Axis,
			AxisSource,
			ButtonState,
			Event,
			InputBackend,
			InputEvent,
			KeyState,
			KeyboardKeyEvent,
			PointerAxisEvent,
			PointerButtonEvent,
			PointerMotionEvent,
		},
		renderer::{
			damage::OutputDamageTracker,
			glow::GlowRenderer,
		},
		winit::WinitGraphicsBackend,
	},
	desktop::{
		layer_map_for_output,
		PopupManager,
		Window,
	},
	input::{
		keyboard::{
			xkb,
			FilterResult,
			Keysym,
			KeysymHandle,
			ModifiersState,
			XkbConfig,
		},
		pointer::{
			AxisFrame,
			ButtonEvent,
			MotionEvent,
			RelativeMotionEvent,
		},
		Seat,
		SeatState,
	},
	reexports::{
		calloop::{
			generic::{
				FdWrapper,
				Generic,
			},
			EventLoop,
			Interest,
			LoopHandle,
			LoopSignal,
			Mode,
			PostAction,
		},
		wayland_server::{
			backend::{
				ClientData,
				ClientId,
				DisconnectReason,
			},
			Display,
			DisplayHandle,
		},
	},
	utils::{
		Logical,
		Physical,
		Point,
		Size,
		SERIAL_COUNTER,
	},
	wayland::{
		compositor::{
			CompositorClientState,
			CompositorState,
		},
		output::OutputManagerState,
		selection::{
			data_device::DataDeviceState,
			primary_selection::PrimarySelectionState,
		},
		shell::{
			wlr_layer::{
				Layer,
				WlrLayerShellState,
			},
			xdg::{
				decoration::XdgDecorationState,
				XdgShellState,
			},
		},
		shm::ShmState,
		socket::ListeningSocketSource,
	},
};
use std::{
	cell::RefCell,
	collections::{
		HashMap,
		HashSet,
	},
	ffi::OsString,
	ops::Deref,
	os::fd::AsRawFd,
	process::Command,
	rc::Rc,
	sync::Arc,
	time::Instant,
};

pub enum Action {
	LuaExecute(StashedFunction),
	Return,
}

pub struct StrataState {
	pub lua: Lua,
	pub comp: Rc<RefCell<StrataComp>>,
	pub config: HashMap<KeyPattern, StashedFunction>,
	pub display: Display<StrataComp>,
}

impl StrataState {
	pub fn process_input_event<I: InputBackend>(
		&mut self,
		event: InputEvent<I>,
	) -> anyhow::Result<()> {
		match event {
			InputEvent::Keyboard { event, .. } => self.keyboard::<I>(event),
			InputEvent::PointerMotion { event, .. } => self.pointer_motion::<I>(event),
			InputEvent::PointerMotionAbsolute { event, .. } => {
				self.pointer_motion_absolute::<I>(event)
			}
			InputEvent::PointerButton { event, .. } => self.pointer_button::<I>(event),
			InputEvent::PointerAxis { event, .. } => self.pointer_axis::<I>(event),
			_ => anyhow::bail!("unhandled winit event"),
		}
	}

	pub fn keyboard<I: InputBackend>(&mut self, event: I::KeyboardKeyEvent) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let time = Event::time_msec(&event);

		let keyboard = self.comp.borrow().seat.get_keyboard().unwrap();
		let f = keyboard.input(
			&mut self.comp.borrow_mut(),
			event.key_code(),
			event.state(),
			serial,
			time,
			|comp, mods, keysym_h| {
				let handled_mods = comp.handle_mods::<I>(mods, keysym_h.modified_sym(), &event);

				match event.state() {
					KeyState::Pressed => {
						if !handled_mods {
							let k =
								KeyPattern { mods: comp.mods.flags, key: keysym_h.modified_sym() };

							println!("{:#?}", self.config);
							println!("{:#?}\n", k);

							if let Some(f) = self.config.get(&k) {
								return FilterResult::Intercept(f);
							}
						}

						FilterResult::Forward
					}
					KeyState::Released => {
						return FilterResult::Forward;
					}
				}
			},
		);

		if let Some(f) = f {
			let ex = self.lua.try_enter(|ctx| {
				let f = ctx.fetch(f);
				Ok(ctx.stash(Executor::start(ctx, f, ())))
			})?;
			let _ = self.lua.execute::<()>(&ex)?;
		}

		Ok(())
	}

	pub fn pointer_motion<I: InputBackend>(
		&mut self,
		event: I::PointerMotionEvent,
	) -> anyhow::Result<()> {
		self.comp.borrow_mut().pointer_motion::<I>(event)?;

		Ok(())
	}

	pub fn pointer_motion_absolute<I: InputBackend>(
		&mut self,
		event: I::PointerMotionAbsoluteEvent,
	) -> anyhow::Result<()> {
		self.comp.borrow_mut().pointer_motion_absolute::<I>(event)?;

		Ok(())
	}

	pub fn pointer_button<I: InputBackend>(
		&mut self,
		event: I::PointerButtonEvent,
	) -> anyhow::Result<()> {
		self.comp.borrow_mut().pointer_button::<I>(event)?;

		Ok(())
	}

	pub fn pointer_axis<I: InputBackend>(
		&mut self,
		event: I::PointerAxisEvent,
	) -> anyhow::Result<()> {
		self.comp.borrow_mut().pointer_axis::<I>(event)?;

		Ok(())
	}
}

pub struct StrataComp {
	pub dh: DisplayHandle,
	pub backend: WinitGraphicsBackend<GlowRenderer>,
	pub damage_tracker: OutputDamageTracker,
	pub start_time: Instant,
	pub loop_signal: LoopSignal,
	pub compositor_state: CompositorState,
	pub xdg_shell_state: XdgShellState,
	pub xdg_decoration_state: XdgDecorationState,
	pub shm_state: ShmState,
	pub output_manager_state: OutputManagerState,
	pub data_device_state: DataDeviceState,
	pub primary_selection_state: PrimarySelectionState,
	pub seat_state: SeatState<StrataComp>,
	pub layer_shell_state: WlrLayerShellState,
	pub popup_manager: PopupManager,
	pub seat: Seat<StrataComp>,
	pub socket_name: OsString,
	pub workspaces: Workspaces,
	pub pointer_location: Point<f64, Logical>,
	pub mods: Mods,
}

impl StrataComp {
	pub fn new(
		event_loop: &EventLoop<StrataState>,
		display: &Display<StrataComp>,
		socket_name: OsString,
		seat_name: String,
		backend: WinitGraphicsBackend<GlowRenderer>,
		damage_tracker: OutputDamageTracker,
	) -> Self {
		let start_time = Instant::now();
		let dh = display.handle();
		let loop_signal = event_loop.get_signal();
		let compositor_state = CompositorState::new::<Self>(&dh);
		let xdg_shell_state = XdgShellState::new::<Self>(&dh);
		let xdg_decoration_state = XdgDecorationState::new::<Self>(&dh);
		let shm_state = ShmState::new::<Self>(&dh, vec![]);
		let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
		let mut seat_state = SeatState::new();
		let data_device_state = DataDeviceState::new::<Self>(&dh);
		let primary_selection_state = PrimarySelectionState::new::<Self>(&dh);
		let layer_shell_state = WlrLayerShellState::new::<Self>(&dh);

		let mut seat = seat_state.new_wl_seat(&dh, seat_name);
		seat.add_keyboard(
			XkbConfig {
				layout: "it",
				options: Some("caps:swapescape".to_string()),
				..Default::default()
			},
			500,
			250,
		)
		.expect("Couldn't parse XKB config");
		seat.add_pointer();

		let config_workspace: u8 = 5;
		let workspaces = Workspaces::new(config_workspace);

		StrataComp {
			dh,
			backend,
			damage_tracker,
			start_time,
			socket_name,
			compositor_state,
			xdg_shell_state,
			xdg_decoration_state,
			loop_signal,
			shm_state,
			output_manager_state,
			popup_manager: PopupManager::default(),
			seat_state,
			data_device_state,
			primary_selection_state,
			layer_shell_state,
			seat,
			workspaces,
			pointer_location: Point::from((0.0, 0.0)),
			mods: Mods { flags: ModFlags::empty(), state: None },
		}
	}

	pub fn ud_from_rc_refcell<'gc>(
		ctx: Context<'gc>,
		state: Rc<RefCell<StrataComp>>,
	) -> anyhow::Result<UserData<'gc>> {
		let ud = UserData::new_static(&ctx, state);
		ud.set_metatable(&ctx, Some(Self::metatable(ctx)?));
		Ok(ud)
	}

	pub fn metatable<'gc>(ctx: Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		m.set(
			ctx,
			MetaMethod::Index,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (ud, k) = stack.consume::<(UserData, piccolo::String)>(ctx)?;
				// let comp = ud.downcast_static::<Rc<RefCell<StrataComp>>>()?;

				match k.as_bytes() {
					b"quit" => {
						stack.push_front(
							Callback::from_fn(&ctx, |ctx, _, mut stack| {
								let comp = stack
									.consume::<UserData>(ctx)?
									.downcast_static::<Rc<RefCell<StrataComp>>>()?;

								comp.borrow_mut().quit();

								Ok(CallbackReturn::Return)
							})
							.into(),
						);
					}
					_ => {
						panic!("invalid key: {}", k);
					}
				};

				Ok(CallbackReturn::Return)
			}),
		)?;

		m.set(
			ctx,
			MetaMethod::NewIndex,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (ud, k, v) =
					stack.consume::<(UserData, piccolo::String, piccolo::Value)>(ctx)?;

				match k.as_bytes() {
					b"" => {}
					_ => {
						panic!("invalid key: {}", k);
					}
				};
				// todo
				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(m)
	}

	pub fn window_under(&mut self) -> Option<(Window, Point<i32, Logical>)> {
		let pos = self.pointer_location;
		self.workspaces.current().window_under(pos).map(|(w, p)| (w.clone(), p))
	}
	pub fn surface_under(&self) -> Option<(FocusTarget, Point<i32, Logical>)> {
		let pos = self.pointer_location;
		let output = self.workspaces.current().outputs().find(|o| {
			let geometry = self.workspaces.current().output_geometry(o).unwrap();
			geometry.contains(pos.to_i32_round())
		})?;
		let output_geo = self.workspaces.current().output_geometry(output).unwrap();
		let layers = layer_map_for_output(output);

		let mut under = None;
		if let Some(layer) =
			layers.layer_under(Layer::Overlay, pos).or_else(|| layers.layer_under(Layer::Top, pos))
		{
			let layer_loc = layers.layer_geometry(layer).unwrap().loc;
			under = Some((layer.clone().into(), output_geo.loc + layer_loc))
		} else if let Some((window, location)) = self.workspaces.current().window_under(pos) {
			under = Some((window.clone().into(), location));
		} else if let Some(layer) = layers
			.layer_under(Layer::Bottom, pos)
			.or_else(|| layers.layer_under(Layer::Background, pos))
		{
			let layer_loc = layers.layer_geometry(layer).unwrap().loc;
			under = Some((layer.clone().into(), output_geo.loc + layer_loc));
		};
		under
	}

	pub fn close_window(&mut self) {
		if let Some((window, _)) = self.workspaces.current().window_under(self.pointer_location) {
			window.toplevel().send_close()
		}
	}

	pub fn switch_to_workspace(&mut self, id: u8) {
		self.workspaces.activate(id);
		self.set_input_focus_auto();
	}

	pub fn move_window_to_workspace(&mut self, id: u8) {
		let window =
			self.workspaces.current().window_under(self.pointer_location).map(|d| d.0.clone());

		if let Some(window) = window {
			self.workspaces.move_window_to_workspace(&window, id);
		}
	}

	pub fn follow_window_move(&mut self, id: u8) {
		self.move_window_to_workspace(id);
		self.switch_to_workspace(id);
	}

	pub fn quit(&mut self) {
		self.loop_signal.stop();
	}

	pub fn spawn(&mut self, command: &str) {
		Command::new("/bin/sh").arg("-c").arg(command).spawn().expect("Failed to spawn command");
	}

	pub fn handle_mods<I: InputBackend>(
		&mut self,
		new_modstate: &ModifiersState,
		keysym: Keysym,
		event: &I::KeyboardKeyEvent,
	) -> bool {
		let mut r = false;
		let old_modstate = self.mods.state.take().unwrap_or(new_modstate.clone());

		let modflag = match keysym {
			// equivalent to "Control_* + Shift_* + Alt_*"
			Keysym::Meta_L => ModFlags::Alt_L,
			Keysym::Meta_R => ModFlags::Alt_R,

			Keysym::Shift_L => ModFlags::Shift_L,
			Keysym::Shift_R => ModFlags::Shift_R,

			Keysym::Control_L => ModFlags::Control_L,
			Keysym::Control_R => ModFlags::Control_R,

			Keysym::Alt_L => ModFlags::Alt_L,
			Keysym::Alt_R => ModFlags::Alt_R,

			Keysym::Super_L => ModFlags::Super_L,
			Keysym::Super_R => ModFlags::Super_R,

			Keysym::ISO_Level3_Shift => ModFlags::ISO_Level3_Shift,
			Keysym::ISO_Level5_Shift => ModFlags::ISO_Level5_Shift,

			_ => ModFlags::empty(),
		};

		match event.state() {
			KeyState::Pressed => {
				let depressed = if new_modstate == &old_modstate {
					self.mods.flags.is_empty()
				} else {
					new_modstate.serialized.depressed > old_modstate.serialized.depressed
				};

				// valid mod
				if new_modstate.serialized.depressed - new_modstate.serialized.locked > 0
					&& depressed
				{
					self.mods.flags ^= modflag;
					r = true;
				}
			}
			KeyState::Released => {
				self.mods.flags ^= modflag;
			}
		};

		self.mods.state = Some(new_modstate.clone());

		r
	}
}

pub fn init_wayland_listener(
	event_loop: &EventLoop<StrataState>,
) -> (Display<StrataComp>, OsString) {
	let loop_handle = event_loop.handle();
	let mut display: Display<StrataComp> = Display::new().unwrap();
	let listening_socket = ListeningSocketSource::new_auto().unwrap();
	let socket_name = listening_socket.socket_name().to_os_string();

	loop_handle
		.insert_source(listening_socket, move |client_stream, _, state| {
			// You may also associate some data with the client when inserting the client.
			state
				.display
				.handle()
				.insert_client(client_stream, Arc::new(ClientState::default()))
				.unwrap();
		})
		.expect("Failed to init the wayland event source.");

	loop_handle
		.insert_source(
			Generic::new(
				unsafe { FdWrapper::new(display.backend().poll_fd().as_raw_fd()) },
				Interest::READ,
				Mode::Level,
			),
			|_, _, state| {
				state.display.dispatch_clients(&mut state.comp.borrow_mut())?;

				Ok(PostAction::Continue)
			},
		)
		.unwrap();

	(display, socket_name)
}

#[derive(Default)]
pub struct ClientState {
	pub compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
	fn initialized(&self, _client_id: ClientId) {}
	fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
