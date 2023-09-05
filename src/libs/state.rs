use crate::{
	libs::structs::{
		state::{
			CalloopData,
			StrataState,
		},
		workspaces::{
			FocusTarget,
			Workspaces,
		},
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
		SeatState,
	},
	reexports::{
		calloop::{
			generic::Generic,
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
		data_device::DataDeviceState,
		output::OutputManagerState,
		primary_selection::PrimarySelectionState,
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
	os::unix::io::AsRawFd,
	process::Command,
	sync::Arc,
	time::Instant,
};

impl StrataState {
	pub fn new(
		mut loop_handle: LoopHandle<'static, CalloopData>,
		loop_signal: LoopSignal,
		display: &mut Display<StrataState>,
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
		let socket_name = Self::init_wayland_listener(&mut loop_handle, display);

		Self {
			loop_handle,
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
		handle: &mut LoopHandle<'static, CalloopData>,
		display: &mut Display<StrataState>,
	) -> OsString {
		let listening_socket = ListeningSocketSource::new_auto().unwrap();
		let socket_name = listening_socket.socket_name().to_os_string();

		handle
			.insert_source(listening_socket, move |client_stream, _, state| {
				state
					.display
					.handle()
					.insert_client(client_stream, Arc::new(ClientState::default()))
					.unwrap();
			})
			.expect("Failed to init the wayland event source.");

		handle
			.insert_source(
				Generic::new(display.backend().poll_fd().as_raw_fd(), Interest::READ, Mode::Level),
				|_, _, state| {
					state.display.dispatch_clients(&mut state.state).unwrap();
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
}

#[derive(Default)]
pub struct ClientState {
	pub compositor_state: CompositorClientState,
}
impl ClientData for ClientState {
	fn initialized(&self, _client_id: ClientId) {}
	fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
