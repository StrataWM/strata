use crate::libs::structs::{
	CompWorkspaces,
	Strata,
	StrataWindow,
};
use log::warn;
use smithay::{
	delegate_xdg_decoration,
	delegate_xdg_shell,
	desktop::{
		Space,
		Window,
	},
	reexports::{
		wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
		wayland_server::protocol::{
			wl_seat,
			wl_surface::WlSurface,
		},
	},
	utils::Serial,
	wayland::{
		compositor::with_states,
		shell::xdg::{
			decoration::XdgDecorationHandler,
			PopupSurface,
			PositionerState,
			ToplevelSurface,
			XdgShellHandler,
			XdgShellState,
			XdgToplevelSurfaceData,
		},
	},
};
use std::{
	cell::RefCell,
	rc::Rc,
	sync::Mutex,
};

impl XdgShellHandler for Strata {
	fn xdg_shell_state(&mut self) -> &mut XdgShellState {
		&mut self.xdg_shell_state
	}

	fn new_toplevel(&mut self, surface: ToplevelSurface) {
		let window = Window::new(surface);
		self.workspaces.current_mut().add_window(Rc::new(RefCell::new(StrataWindow {
			window: window.clone(),
			rec: window.geometry(),
		})));
	}

	fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
		let window =
			self.workspaces.all_windows().find(|w| w.toplevel() == &surface).unwrap().clone();

		self.workspaces.workspace_from_window(&window).unwrap().remove_window(&window);
	}

	fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState) {
		surface.with_pending_state(|state| {
			state.geometry = positioner.get_geometry();
		});
	}

	fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}
}
delegate_xdg_shell!(Strata);

pub fn handle_commit(space: &Space<Window>, surface: &WlSurface) -> Option<()> {
	let window = space.elements().find(|w| w.toplevel().wl_surface() == surface).cloned()?;

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

impl XdgDecorationHandler for Strata {
	fn new_decoration(&mut self, toplevel: ToplevelSurface) {
		toplevel.with_pending_state(|state| {
			// Advertise server side decoration
			state.decoration_mode = Some(Mode::ServerSide);
		});
		toplevel.send_configure();
	}

	fn request_mode(
		&mut self,
		_toplevel: ToplevelSurface,
		_mode: smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
	) {
	}

	fn unset_mode(&mut self, _toplevel: ToplevelSurface) {}
}

delegate_xdg_decoration!(Strata);
