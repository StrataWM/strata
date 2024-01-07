use crate::workspaces::{
	FocusTarget,
	Workspaces,
};
use gc_arena::{
	lock::RefLock,
	Rootable,
};
use piccolo::{
	meta_ops,
	Callback,
	CallbackReturn,
	Closure,
	Context,
	Executor,
	Lua,
	MetaMethod,
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
			generic::Generic,
			EventLoop,
			Interest,
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
	ffi::OsString,
	process::Command,
	rc::Rc,
	sync::Arc,
	time::Instant,
};

pub struct StrataState {
	pub lua: Lua,
	pub comp: Rc<RefCell<StrataComp>>,
	pub display_handle: DisplayHandle,
}

impl StrataState {
	pub fn process_input_event<I: InputBackend>(
		&mut self,
		event: InputEvent<I>,
	) -> anyhow::Result<()> {
		match &event {
			InputEvent::Keyboard { event, .. } => self.keyboard::<I>(event),
			InputEvent::PointerMotion { event, .. } => self.pointer_motion::<I>(event),
			InputEvent::PointerMotionAbsolute { event, .. } => {
				self.pointer_motion_absolute::<I>(event)
			}
			InputEvent::PointerButton { event, .. } => self.pointer_button::<I>(event),
			InputEvent::PointerAxis { event, .. } => self.pointer_axis::<I>(event),
			_ => Ok(()),
		}
	}

	pub fn keyboard<I: InputBackend>(&mut self, event: &I::KeyboardKeyEvent) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let time = Event::time_msec(event);

		let keyboard = self.comp.borrow_mut().seat.get_keyboard().unwrap();
		let ex = keyboard.input(
			&mut self.comp.borrow_mut(),
			event.key_code(),
			event.state(),
			serial,
			time,
			|_, mods, handle| {
				// for binding in &CONFIG.read().bindings {
				// 	let mut keysym: Keysym =
				// 		xkb::utf32_to_keysym(xkb::keysyms::KEY_NoSymbol);
				// 	let mut modifier_state = ModifiersState::default();
				//
				// 	for key in &binding.keys {
				// 		match key.as_str() {
				// 			"Super_L" => modifier_state.logo = true,
				// 			"Super_R" => modifier_state.logo = true,
				// 			"Shift_L" => modifier_state.shift = true,
				// 			"Shift_R" => modifier_state.shift = true,
				// 			"Alt_L" => modifier_state.alt = true,
				// 			"Alt_R" => modifier_state.alt = true,
				// 			"Ctrl_L" => modifier_state.ctrl = true,
				// 			"Ctrl_R" => modifier_state.ctrl = true,
				// 			"CapsLck" => modifier_state.caps_lock = true,
				// 			"Caps" => modifier_state.caps_lock = true,
				// 			&_ => {
				// 				let sym = xkb::keysym_from_name(
				// 					key.as_str(),
				// 					xkb::KEYSYM_NO_FLAGS,
				// 				);
				// 				keysym = sym;
				// 			}
				// 		}
				// 	}
				// 	if event.state() == KeyState::Released
				// 		&& modifier_state.alt == mods.alt && modifier_state.ctrl
				// 		== mods.ctrl && modifier_state.shift == mods.shift
				// 		&& modifier_state.logo == mods.logo && modifier_state.caps_lock
				// 		== mods.caps_lock && handle.raw_syms().contains(&keysym)
				// 	{
				// 	}
				// }

				if event.state() == KeyState::Released {
					let raw_syms = handle.raw_syms();

					if raw_syms.contains(&xkb::keysyms::KEY_Escape.into()) {
						let ex = self
							.lua
							.try_enter(|ctx| {
								let func = Closure::load(
									ctx,
									None,
									r#"
									strata:quit()
									"#
									.as_bytes(),
								)
								.unwrap();

								Ok(ctx.stash(Executor::start(ctx, func.into(), ())))
							})
							.unwrap();

						return FilterResult::Intercept(ex);
					}
				}

				FilterResult::Forward
			},
		);

		if let Some(ex) = ex {
			self.lua.execute::<()>(&ex)?;
		}

		Ok(())
	}

	pub fn pointer_motion<I: InputBackend>(
		&mut self,
		event: &I::PointerMotionEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let delta = (event.delta_x(), event.delta_y()).into();
		self.comp.borrow_mut().pointer_location += delta;
		self.comp.borrow_mut().pointer_location =
			self.comp.borrow().clamp_coords(self.comp.borrow().pointer_location);

		let under = self.comp.borrow().surface_under();

		self.comp.borrow_mut().set_input_focus_auto();

		let ptr = self.comp.borrow().seat.get_pointer();

		if let Some(ptr) = ptr {
			// let pointer_location = s.pointer_location;
			ptr.motion(
				&mut self.comp.borrow_mut(),
				under.clone(),
				&MotionEvent {
					location: self.comp.borrow().pointer_location,
					serial,
					time: event.time_msec(),
				},
			);

			ptr.relative_motion(
				&mut self.comp.borrow_mut(),
				under,
				&RelativeMotionEvent {
					delta,
					delta_unaccel: event.delta_unaccel(),
					utime: event.time(),
				},
			)
		}

		Ok(())
	}

	pub fn pointer_motion_absolute<I: InputBackend>(
		&mut self,
		event: &I::PointerMotionAbsoluteEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();

		let output = self.comp.borrow().workspaces.current().outputs().next().unwrap().clone();
		let output_geo = self.comp.borrow().workspaces.current().output_geometry(&output).unwrap();
		let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();
		let pointer = self.comp.borrow().seat.get_pointer().unwrap();

		let pointer_location = self.comp.borrow_mut().clamp_coords(pos);
		self.comp.borrow_mut().pointer_location = pointer_location;

		self.comp.borrow_mut().set_input_focus_auto();
		let under = self.comp.borrow().surface_under();
		pointer.motion(
			&mut self.comp.borrow_mut(),
			under,
			&MotionEvent { location: pos, serial, time: event.time_msec() },
		);

		Ok(())
	}

	pub fn pointer_button<I: InputBackend>(
		&mut self,
		event: &I::PointerButtonEvent,
	) -> anyhow::Result<()> {
		let pointer = self.comp.borrow().seat.get_pointer().unwrap();
		let serial = SERIAL_COUNTER.next_serial();
		let button = event.button_code();
		let button_state = event.state();
		self.comp.borrow_mut().set_input_focus_auto();
		pointer.button(
			&mut self.comp.borrow_mut(),
			&ButtonEvent { button, state: button_state, serial, time: event.time_msec() },
		);

		Ok(())
	}

	pub fn pointer_axis<I: InputBackend>(
		&mut self,
		event: &I::PointerAxisEvent,
	) -> anyhow::Result<()> {
		let horizontal_amount = event
			.amount(Axis::Horizontal)
			.unwrap_or_else(|| event.amount_discrete(Axis::Horizontal).unwrap_or(0.0) * 3.0);
		let vertical_amount = event
			.amount(Axis::Vertical)
			.unwrap_or_else(|| event.amount_discrete(Axis::Vertical).unwrap_or(0.0) * 3.0);
		let horizontal_amount_discrete = event.amount_discrete(Axis::Horizontal);
		let vertical_amount_discrete = event.amount_discrete(Axis::Vertical);

		{
			let mut frame = AxisFrame::new(event.time_msec()).source(event.source());
			if horizontal_amount != 0.0 {
				frame = frame.value(Axis::Horizontal, horizontal_amount);
				if let Some(discrete) = horizontal_amount_discrete {
					frame = frame.discrete(Axis::Horizontal, discrete as i32);
				}
			} else if event.source() == AxisSource::Finger {
				frame = frame.stop(Axis::Horizontal);
			}
			if vertical_amount != 0.0 {
				frame = frame.value(Axis::Vertical, vertical_amount);
				if let Some(discrete) = vertical_amount_discrete {
					frame = frame.discrete(Axis::Vertical, discrete as i32);
				}
			} else if event.source() == AxisSource::Finger {
				frame = frame.stop(Axis::Vertical);
			}
			let ptr = self.comp.borrow().seat.get_pointer();

			if let Some(ptr) = ptr {
				ptr.axis(&mut self.comp.borrow_mut(), frame);
			}
		}

		Ok(())
	}
}

#[derive(gc_arena::Collect)]
#[collect(require_static)]
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
	pub seat_name: String,
	pub socket_name: OsString,
	pub workspaces: Workspaces,
	pub pointer_location: Point<f64, Logical>,
}

impl StrataComp {
	pub fn new(
		event_loop: &mut EventLoop<StrataState>,
		display: Display<StrataComp>,
		seat_name: String,
		backend: WinitGraphicsBackend<GlowRenderer>,
		damage_tracker: OutputDamageTracker,
	) -> Self {
		let start_time = Instant::now();
		let dh = display.handle();
		let compositor_state = CompositorState::new::<Self>(&dh);
		let xdg_shell_state = XdgShellState::new::<Self>(&dh);
		let xdg_decoration_state = XdgDecorationState::new::<Self>(&dh);
		let shm_state = ShmState::new::<Self>(&dh, vec![]);
		let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
		let mut seat_state = SeatState::new();
		let data_device_state = DataDeviceState::new::<Self>(&dh);
		let primary_selection_state = PrimarySelectionState::new::<Self>(&dh);
		let mut seat = seat_state.new_wl_seat(&dh, seat_name.clone());
		let layer_shell_state = WlrLayerShellState::new::<Self>(&dh);

		seat.add_keyboard(XkbConfig::default(), 500, 250).expect("Couldn't parse XKB config");
		let config_workspace: u8 = 5;
		let workspaces = Workspaces::new(config_workspace);
		seat.add_pointer();
		let socket_name = Self::init_wayland_listener(display, event_loop);
		let loop_signal = event_loop.get_signal();

		StrataComp {
			dh,
			backend,
			damage_tracker,
			start_time,
			seat_name,
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
		}
	}

	pub fn ud_from_rc_refcell<'gc>(
		ctx: Context<'gc>,
		state: Rc<RefCell<StrataComp>>,
	) -> anyhow::Result<UserData<'gc>> {
		let ud = UserData::new_static(&ctx, state);
		ud.set_metatable(&ctx, Some(StrataComp::metatable(ctx)?));
		Ok(ud)
	}

	pub fn metatable<'gc>(ctx: Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		m.set(
			ctx,
			MetaMethod::Index,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (ud, k) = stack.consume::<(UserData, piccolo::String)>(ctx)?;
				// let this = ud.downcast_static::<Rc<RefCell<StrataComp>>>()?;

				match k.as_bytes() {
					b"quit" => {
						stack.push_front(
							Callback::from_fn(&ctx, |ctx, _, mut stack| {
								let this = stack
									.consume::<UserData>(ctx)?
									.downcast_static::<Rc<RefCell<StrataComp>>>()?;

								this.borrow_mut().quit();

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

	fn init_wayland_listener(
		display: Display<StrataComp>,
		event_loop: &mut EventLoop<StrataState>,
	) -> OsString {
		let listening_socket = ListeningSocketSource::new_auto().unwrap();
		let socket_name = listening_socket.socket_name().to_os_string();

		let evlh = event_loop.handle();

		evlh.insert_source(listening_socket, move |client_stream, _, state| {
			// You may also associate some data with the client when inserting the client.
			state
				.display_handle
				.insert_client(client_stream, Arc::new(ClientState::default()))
				.unwrap();
		})
		.expect("Failed to init the wayland event source.");

		evlh.insert_source(
			Generic::new(display, Interest::READ, Mode::Level),
			|_, display, state| {
				unsafe {
					display.get_mut().dispatch_clients(&mut state.comp.borrow_mut()).unwrap();
				}

				Ok(PostAction::Continue)
			},
		)
		.unwrap();

		socket_name
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
}

pub struct CommsChannel<T> {
	pub sender: crossbeam_channel::Sender<T>,
	pub receiver: crossbeam_channel::Receiver<T>,
}

pub enum ConfigCommands {
	Spawn(String),
	CloseWindow,
	SwitchWS(u8),
	MoveWindow(u8),
	MoveWindowAndFollow(u8),
	Quit,
}

#[derive(Default)]
pub struct ClientState {
	pub compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
	fn initialized(&self, _client_id: ClientId) {}
	fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
