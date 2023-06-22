use crate::libs::structs::Strata;
use smithay::{
	delegate_xdg_shell,
	desktop::{
		Space,
		Window,
	},
	reexports::wayland_server::protocol::{
		wl_seat,
		wl_surface::WlSurface,
	},
	utils::Serial,
	wayland::{
		compositor::with_states,
		shell::xdg::{
			PopupSurface,
			PositionerState,
			ToplevelSurface,
			XdgShellHandler,
			XdgShellState,
			XdgToplevelSurfaceData,
		},
	},
};

impl XdgShellHandler for Strata {
	fn xdg_shell_state(&mut self) -> &mut XdgShellState {
		&mut self.xdg_shell_state
	}

	fn new_toplevel(&mut self, surface: ToplevelSurface) {
		let window = Window::new(surface);
		self.space.map_element(window, (0, 0), false);
		self.refresh_geometry();
	}

	fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
		self.refresh_geometry();
	}

	fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}

	fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}
}
delegate_xdg_shell!(Strata);

pub fn handle_commit(space: &Space<Window>, surface: &WlSurface) -> Option<()> {
	let window = space
		.elements()
		.find(|w| w.toplevel().wl_surface() == surface)
		.cloned()?;

	let initial_configure_sent = with_states(surface, |states| {
		states
			.data_map
			.get::<XdgToplevelSurfaceData>()
			.unwrap()
			.lock()
			.unwrap()
			.initial_configure_sent
	});

	if !initial_configure_sent {
		window.toplevel().send_configure();
	}

	Some(())
}
