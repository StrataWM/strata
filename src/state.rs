// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
	cell::RefCell,
	collections::HashMap,
	ffi::OsString,
	os::fd::AsRawFd,
	process::Command,
	rc::Rc,
	sync::Arc,
	time::{
		Duration,
		Instant,
	},
};

use piccolo as lua;
use smithay::{
	backend::{
		allocator::{
			dmabuf::{
				AnyError,
				Dmabuf,
			},
			Allocator,
		},
		drm::DrmNode,
		input::{
			Event,
			InputBackend,
			InputEvent,
			KeyState,
			KeyboardKeyEvent,
		},
		renderer::{
			damage::OutputDamageTracker,
			element::texture::TextureBuffer,
			gles::GlesRenderer,
			glow::GlowRenderer,
			multigpu::{
				gbm::GbmGlesBackend,
				GpuManager,
				MultiTexture,
			},
		},
		session::libseat::LibSeatSession,
		winit::WinitGraphicsBackend,
	},
	desktop::{
		layer_map_for_output,
		space::SpaceElement,
		PopupManager,
	},
	input::{
		keyboard::{
			FilterResult,
			Keysym,
			ModifiersState,
			XkbConfig,
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
		Point,
		Rectangle,
		SERIAL_COUNTER,
	},
	wayland::{
		compositor::{
			CompositorClientState,
			CompositorState,
		},
		dmabuf::{
			DmabufGlobal,
			DmabufState,
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

use crate::{
	backends::Backend,
	decorations::BorderShader,
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

pub struct StrataState {
	pub lua: lua::Lua,
	pub comp: Rc<RefCell<StrataComp>>,
	pub display: Display<StrataComp>,
}

impl StrataState {
	pub fn process_input_event<I: InputBackend>(
		&mut self,
		event: InputEvent<I>,
	) -> anyhow::Result<()> {
		match event {
			InputEvent::Keyboard { event, .. } => self.keyboard::<I>(event)?,
			InputEvent::PointerMotion { event, .. } => self.pointer_motion::<I>(event)?,
			InputEvent::PointerMotionAbsolute { event, .. } => {
				self.pointer_motion_absolute::<I>(event)?
			}
			InputEvent::PointerButton { event, .. } => self.pointer_button::<I>(event)?,
			InputEvent::PointerAxis { event, .. } => self.pointer_axis::<I>(event)?,
			InputEvent::DeviceAdded { device: _ } => {
				// todo
				println!("device added");
			}
			InputEvent::DeviceRemoved { device: _ } => todo!(),
			InputEvent::GestureSwipeBegin { event: _ } => todo!(),
			InputEvent::GestureSwipeUpdate { event: _ } => todo!(),
			InputEvent::GestureSwipeEnd { event: _ } => todo!(),
			InputEvent::GesturePinchBegin { event: _ } => todo!(),
			InputEvent::GesturePinchUpdate { event: _ } => todo!(),
			InputEvent::GesturePinchEnd { event: _ } => todo!(),
			InputEvent::GestureHoldBegin { event: _ } => todo!(),
			InputEvent::GestureHoldEnd { event: _ } => todo!(),
			InputEvent::TouchDown { event: _ } => todo!(),
			InputEvent::TouchMotion { event: _ } => todo!(),
			InputEvent::TouchUp { event: _ } => todo!(),
			InputEvent::TouchCancel { event: _ } => todo!(),
			InputEvent::TouchFrame { event: _ } => todo!(),
			InputEvent::TabletToolAxis { event: _ } => todo!(),
			InputEvent::TabletToolProximity { event: _ } => todo!(),
			InputEvent::TabletToolTip { event: _ } => todo!(),
			InputEvent::TabletToolButton { event: _ } => todo!(),
			InputEvent::Special(_) => todo!(),
			// _ => anyhow::bail!("unhandled winit event: {:#?}", &event),
		};

		Ok(())
	}

	pub fn keyboard<I: InputBackend>(&mut self, event: I::KeyboardKeyEvent) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let time = Event::time_msec(&event);

		// println!("key: {:#?}, {:#?}", Key::from_name("b"), Keysym::b);

		let keyboard = self.comp.borrow().seat.get_keyboard().unwrap();
		let f = keyboard.input(
			&mut self.comp.borrow_mut(),
			event.key_code(),
			event.state(),
			serial,
			time,
			|comp, mods, keysym_h| {
				comp.handle_mods::<I>(mods, keysym_h.modified_sym(), &event);

				// println!("{:#?}", comp.mods);
				// println!("{:#?}({:#?})", event.state(), keysym_h.modified_sym());
				match event.state() {
					KeyState::Pressed => {
						let k = KeyPattern {
							mods: comp.mods.flags,
							key: keysym_h.modified_sym().into(),
						};

						if let Some(f) = comp.config.keybinds.get(&k) {
							return FilterResult::Intercept(f.clone());
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
				let f = ctx.fetch(&f);
				Ok(ctx.stash(lua::Executor::start(ctx, f, ())))
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

pub struct WinitData {
	backend: WinitGraphicsBackend<GlowRenderer>,
	damage_tracker: OutputDamageTracker,
}

pub struct UdevData {
	pub session: LibSeatSession,
	dh: DisplayHandle,
	dmabuf_state: Option<(DmabufState, DmabufGlobal)>,
	primary_gpu: DrmNode,
	allocator: Option<Box<dyn Allocator<Buffer = Dmabuf, Error = AnyError>>>,
	gpus: GpuManager<GbmGlesBackend<GlesRenderer>>,
	backends: HashMap<DrmNode, BackendData>,
	pointer_images: Vec<(xcursor::parser::Image, TextureBuffer<MultiTexture>)>,
	pointer_element: PointerElement<MultiTexture>,
	pointer_image: crate::cursor::Cursor,
}

pub enum Backend {
	Winit(WinitData),
	Udev(UdevData),
}

pub struct StrataComp {
	pub dh: DisplayHandle,
	pub backend: Backend,
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
	pub mods: Mods,
	pub config: StrataConfig,
}

impl StrataComp {
	pub fn new(
		event_loop: &EventLoop<StrataState>,
		display: &Display<StrataComp>,
		socket_name: OsString,
		seat_name: String,
		backend: Backend,
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
		let keyboard = seat
			.add_keyboard(
				XkbConfig {
					layout: "it",
					options: Some("caps:swapescape".to_string()),
					..Default::default()
				},
				160,
				40,
			)
			.expect("Couldn't parse XKB config");
		seat.add_pointer();

		let config_workspace: u8 = 5;
		let workspaces = Workspaces::new(config_workspace);
		let mods_state = keyboard.modifier_state();

		StrataComp {
			dh,
			backend,
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
			mods: Mods { flags: ModFlags::empty(), state: mods_state },
			config: StrataConfig { keybinds: HashMap::new() },
		}
	}

	fn winit_render(&mut self) {
		let render_elements = self.workspaces.current().render_elements(self.backend.renderer());
		self.backend
			.render_output(self.backend.renderer(), 0, &render_elements, [0.1, 0.1, 0.1, 1.0])
			.unwrap();
	}
	pub fn surface_under(&self) -> Option<(FocusTarget, Point<i32, Logical>)> {
		let pos = self.seat.get_pointer().unwrap().current_location();
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

	pub fn winit_update(&mut self) {
		self.winit_render();

		self.set_input_focus_auto();

		// damage tracking
		let size = self.backend.window_size();
		let damage = Rectangle::from_loc_and_size((0, 0), size);
		self.backend.bind().unwrap();
		self.backend.submit(Some(&[damage])).unwrap();

		// sync and cleanups
		let output = self.workspaces.current().outputs().next().unwrap();
		self.workspaces.current().windows().for_each(|window| {
			window.send_frame(output, self.start_time.elapsed(), Some(Duration::ZERO), |_, _| {
				Some(output.clone())
			});

			window.refresh();
		});
		self.dh.flush_clients().unwrap();
		self.popup_manager.cleanup();
		BorderShader::cleanup(self.backend.renderer());
	}

	pub fn close_window(&mut self) {
		let ptr = self.seat.get_pointer().unwrap();
		if let Some((window, _)) = self.workspaces.current().window_under(ptr.current_location()) {
			window.toplevel().send_close()
		}
	}

	pub fn switch_to_workspace(&mut self, id: u8) {
		self.workspaces.activate(id);
		self.set_input_focus_auto();
	}

	pub fn move_window_to_workspace(&mut self, id: u8) {
		let pos = self.seat.get_pointer().unwrap().current_location();
		let window = self.workspaces.current().window_under(pos).map(|d| d.0.clone());

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
	) {
		let old_modstate = self.mods.state;

		let modflag = match keysym {
			// equivalent to "Control_* + Shift_* + Alt_*" (on my keyboard *smile*)
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
					// ignore previous modstate
					true
				} else {
					// "lock" key modifier or "normal" key modifier
					new_modstate.serialized.depressed > old_modstate.serialized.depressed
				};

				// "lock" key modifiers (Caps Lock, Num Lock, etc...) => `depressed` == `locked`
				// "normal" key modifiers (Control_*, Shift_*, etc...) => `depressed` > 0
				// "normal" keys (a, s, d, f) => `depressed` == 0
				let is_modifier = new_modstate.serialized.depressed
					> new_modstate.serialized.locked - old_modstate.serialized.locked;

				if is_modifier && depressed {
					self.mods.flags ^= modflag;
				}
			}
			KeyState::Released => {
				self.mods.flags ^= modflag;
			}
		};

		self.mods.state = new_modstate.clone();
	}
}

pub struct StrataConfig {
	pub keybinds: HashMap<KeyPattern, lua::StashedFunction>,
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
