use crate::libs::{
	state::ClientState,
	structs::Strata,
	xdg_shell,
};
use smithay::{
	backend::renderer::utils::on_commit_buffer_handler,
	delegate_compositor,
	delegate_shm,
	reexports::wayland_server::{
		protocol::{
			wl_buffer,
			wl_surface::WlSurface,
		},
		Client,
	},
	wayland::{
		buffer::BufferHandler,
		compositor::{
			get_parent,
			is_sync_subsurface,
			CompositorClientState,
			CompositorHandler,
			CompositorState,
		},
		shm::{
			ShmHandler,
			ShmState,
		},
	},
};

impl<BackendData> CompositorHandler for Strata<BackendData> {
	fn compositor_state(&mut self) -> &mut CompositorState {
		&mut self.compositor_state
	}

	fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
		&client.get_data::<ClientState>().unwrap().compositor_state
	}

	fn commit(&mut self, surface: &WlSurface) {
		on_commit_buffer_handler::<Self>(surface);
		if !is_sync_subsurface(surface) {
			let mut root = surface.clone();
			while let Some(parent) = get_parent(&root) {
				root = parent;
			}
			if let Some(window) = self.space.elements().find(|w| w.toplevel().wl_surface() == &root)
			{
				window.on_commit();
			}
		};

		xdg_shell::handle_commit(&self.space, surface);
	}
}

impl<BackendData> BufferHandler for Strata<BackendData> {
	fn buffer_destroyed(&mut self, _buffer: &wl_buffer::WlBuffer) {}
}

impl<BackendData> ShmHandler for Strata<BackendData> {
	fn shm_state(&self) -> &ShmState {
		&self.shm_state
	}
}

delegate_compositor!(Strata<BackendData>);
delegate_shm!(Strata<BackendData>);
