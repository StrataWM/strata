use crate::{
	libs::structs::{
		Backend,
		CalloopData,
		CompWorkspaces,
		Strata,
	},
	CONFIG,
};
use smithay::{
	desktop::{
		Space,
		WindowSurfaceType,
	},
	input::{
		pointer::PointerHandle,
		Seat,
		SeatState,
	},
	reexports::{
		calloop::{
			generic::Generic,
			EventLoop,
			Interest,
			Mode,
			PostAction,
		},
		wayland_server::{
			backend::{
				ClientData,
				ClientId,
				DisconnectReason,
			},
			protocol::wl_surface::WlSurface,
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
		shell::xdg::XdgShellState,
		shm::ShmState,
		socket::ListeningSocketSource,
	},
};
use std::{
	ffi::OsString,
	os::unix::io::AsRawFd,
	sync::Arc,
};

impl<BackendData: Backend> Strata<BackendData> {
	pub fn new(
		event_loop: &mut EventLoop<CalloopData<BackendData>>,
		display: &mut Display<Self>,
		backend_data: BackendData,
	) -> Self {
		let config = CONFIG.lock().unwrap();
		let start_time = std::time::Instant::now();

		let dh = display.handle();

		let compositor_state = CompositorState::new::<Self>(&dh);
		let xdg_shell_state = XdgShellState::new::<Self>(&dh);
		let shm_state = ShmState::new::<Self>(&dh, vec![]);
		let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
		let mut seat_state = SeatState::new();
		let data_device_state = DataDeviceState::new::<Self>(&dh);
		let seat_name = backend_data.seat_name();
		let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, seat_name.clone());

		seat.add_keyboard(Default::default(), 0, 0).unwrap();
		seat.add_pointer();

		let space = Space::default();
		let socket_name = Self::init_wayland_listener(display, event_loop);
		let loop_signal = event_loop.get_signal();

		let workspaces = CompWorkspaces::new(config.general.workspaces);

		Self {
			start_time,
			workspaces,
			backend_data,
			seat_name,

			space,
			loop_signal,
			socket_name,

			compositor_state,
			xdg_shell_state,
			shm_state,
			output_manager_state,
			seat_state,
			data_device_state,
			seat,
		}
	}

	fn init_wayland_listener(
		display: &mut Display<Strata<BackendData>>,
		event_loop: &mut EventLoop<CalloopData<BackendData>>,
	) -> OsString {
		let listening_socket = ListeningSocketSource::new_auto().unwrap();
		let socket_name = listening_socket.socket_name().to_os_string();
		let handle = event_loop.handle();

		event_loop
			.handle()
			.insert_source(listening_socket, move |client_stream, _, state| {
				state
					.display
					.handle()
					.insert_client(client_stream, Arc::new(ClientState::default()))
					.unwrap();
			})
			.expect("Failed to init the Wayland event source.");

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

	pub fn surface_under_pointer(
		&self,
		pointer: &PointerHandle<Self>,
	) -> Option<(WlSurface, Point<i32, Logical>)> {
		let pos = pointer.current_location();
		self.space.element_under(pos).and_then(|(window, location)| {
			window
				.surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
				.map(|(s, p)| (s, p + location))
		})
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
