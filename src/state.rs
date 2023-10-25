use crate::{
	workspaces::{
		FocusTarget,
		Workspaces,
	},
	CONFIG,
};
use smithay::{
	backend::{
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
		keyboard::XkbConfig,
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
		Point,
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
	ffi::OsString,
	process::Command,
	sync::Arc,
	time::Instant,
};

pub struct CalloopData {
	pub state: StrataState,
	pub display_handle: DisplayHandle,
}

pub struct StrataState {
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
	pub seat_state: SeatState<StrataState>,
	pub layer_shell_state: WlrLayerShellState,
	pub popup_manager: PopupManager,
	pub seat: Seat<Self>,
	pub seat_name: String,
	pub socket_name: OsString,
	pub workspaces: Workspaces,
	pub pointer_location: Point<f64, Logical>,
}

impl StrataState {
	pub fn new(
		event_loop: &mut EventLoop<CalloopData>,
		display: Display<StrataState>,
		seat_name: String,
		backend: WinitGraphicsBackend<GlowRenderer>,
		damage_tracker: OutputDamageTracker,
	) -> Self {
		let config = &CONFIG.read();

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
		if !config.general.kb_repeat.is_empty() {
			let key_delay: i32 = config.general.kb_repeat[0];
			let key_repeat: i32 = config.general.kb_repeat[1];
			seat.add_keyboard(XkbConfig::default(), key_delay, key_repeat)
				.expect("Couldn't parse XKB config");
		} else {
			seat.add_keyboard(XkbConfig::default(), 500, 250).expect("Couldn't parse XKB config");
		}
		let config_workspace: u8 = config.general.workspaces.clone();
		let workspaces = Workspaces::new(config_workspace);
		seat.add_pointer();
		let socket_name = Self::init_wayland_listener(display, event_loop);
		let loop_signal = event_loop.get_signal();

		Self {
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

	fn init_wayland_listener(
		display: Display<StrataState>,
		event_loop: &mut EventLoop<CalloopData>,
	) -> OsString {
		let listening_socket = ListeningSocketSource::new_auto().unwrap();
		let socket_name = listening_socket.socket_name().to_os_string();

		let handle = event_loop.handle();

		event_loop
			.handle()
			.insert_source(listening_socket, move |client_stream, _, state| {
				// You may also associate some data with the client when inserting the client.
				state
					.display_handle
					.insert_client(client_stream, Arc::new(ClientState::default()))
					.unwrap();
			})
			.expect("Failed to init the wayland event source.");

		handle
			.insert_source(
				Generic::new(display, Interest::READ, Mode::Level),
				|_, display, state| {
					unsafe {
						display.get_mut().dispatch_clients(&mut state.state).unwrap();
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
