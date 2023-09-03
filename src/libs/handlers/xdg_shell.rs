use crate::libs::structs::{
	state::StrataState,
	workspaces::{
		StrataWindow,
		Workspaces,
	},
};
use log::warn;
use smithay::{
	delegate_xdg_decoration,
	delegate_xdg_shell,
	desktop::{
		layer_map_for_output,
		PopupKind,
		PopupManager,
		Window,
		WindowSurfaceType,
	},
	reexports::{
		wayland_protocols::xdg::{
			decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
			shell::server::xdg_toplevel::State as ToplevelState,
		},
		wayland_server::protocol::{
			wl_seat::WlSeat,
			wl_surface::WlSurface,
		},
	},
	utils::Serial,
	wayland::{
		compositor::with_states,
		shell::{
			wlr_layer::LayerSurfaceData,
			xdg::{
				decoration::XdgDecorationHandler,
				PopupSurface,
				PositionerState,
				ToplevelSurface,
				XdgPopupSurfaceData,
				XdgShellHandler,
				XdgShellState,
				XdgToplevelSurfaceRoleAttributes,
			},
		},
	},
};
use std::{
	cell::RefCell,
	rc::Rc,
	sync::Mutex,
};

impl XdgShellHandler for StrataState {
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
		if let Err(err) = self.popup_manager.track_popup(PopupKind::from(surface)) {
			warn!("Failed to track popup: {}", err);
		}
	}

	fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {}
}

delegate_xdg_shell!(StrataState);

pub fn handle_commit(workspaces: &Workspaces, surface: &WlSurface, popup_manager: &PopupManager) {
	if let Some(window) = workspaces.all_windows().find(|w| w.toplevel().wl_surface() == surface) {
		let initial_configure_sent = with_states(surface, |states| {
			states
				.data_map
				.get::<Mutex<XdgToplevelSurfaceRoleAttributes>>()
				.unwrap()
				.lock()
				.unwrap()
				.initial_configure_sent
		});
		if !initial_configure_sent {
			let toplevel = window.toplevel();
			toplevel.with_pending_state(|state| {
				state.states.set(ToplevelState::TiledLeft);
				state.states.set(ToplevelState::TiledRight);
				state.states.set(ToplevelState::TiledTop);
				state.states.set(ToplevelState::TiledBottom);
			});
			toplevel.send_configure();
		}
	}

	if let Some(output) = workspaces.current().outputs().find(|o| {
		let map = layer_map_for_output(o);
		map.layer_for_surface(surface, WindowSurfaceType::TOPLEVEL).is_some()
	}) {
		let initial_configure_sent = with_states(surface, |states| {
			states
				.data_map
				.get::<LayerSurfaceData>()
				.unwrap()
				.lock()
				.unwrap()
				.initial_configure_sent
		});
		let mut map = layer_map_for_output(output);
		map.arrange();
		if !initial_configure_sent {
			let layer = map.layer_for_surface(surface, WindowSurfaceType::TOPLEVEL).unwrap();

			layer.layer_surface().send_configure();
		}
	};

	if let Some(popup) = popup_manager.find_popup(surface) {
		let PopupKind::Xdg(ref popup) = popup;
		let initial_configure_sent = with_states(surface, |states| {
			states
				.data_map
				.get::<XdgPopupSurfaceData>()
				.unwrap()
				.lock()
				.unwrap()
				.initial_configure_sent
		});
		if !initial_configure_sent {
			popup.send_configure().expect("initial configure failed");
		}
	};
}

impl XdgDecorationHandler for StrataState {
	fn new_decoration(&mut self, toplevel: ToplevelSurface) {
		toplevel.with_pending_state(|state| {
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
delegate_xdg_decoration!(StrataState);
