use crate::libs::structs::workspaces::Workspaces;
use once_cell::sync::OnceCell;
use smithay::{
	backend::{
		renderer::{
			damage::OutputDamageTracker,
			glow::GlowRenderer,
		},
		winit::WinitGraphicsBackend,
	},
	desktop::PopupManager,
	input::{
		Seat,
		SeatState,
	},
	reexports::{
		calloop::{
			LoopHandle,
			LoopSignal,
		},
		wayland_server::{
			Display,
			DisplayHandle,
		},
	},
	utils::{
		Logical,
		Point,
	},
	wayland::{
		compositor::CompositorState,
		data_device::DataDeviceState,
		output::OutputManagerState,
		primary_selection::PrimarySelectionState,
		shell::{
			wlr_layer::WlrLayerShellState,
			xdg::{
				decoration::XdgDecorationState,
				XdgShellState,
			},
		},
		shm::ShmState,
	},
};
use std::{
	ffi::OsString,
	ops::Deref,
	time::Instant,
};

pub struct CalloopData {
	pub state: StrataState,
	pub display: Display<StrataState>,
}

pub struct StrataState {
	pub dh: DisplayHandle,
	pub backend: WinitGraphicsBackend<GlowRenderer>,
	pub damage_tracker: OutputDamageTracker,
	pub start_time: Instant,
	pub loop_handle: LoopHandle<'static, CalloopData>,
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

pub struct GlobalState {
	inner: OnceCell<StrataState>,
}

impl GlobalState {
	pub fn set(
		&self,
		loop_handle: LoopHandle<'static, CalloopData>,
		loop_signal: LoopSignal,
		display: &mut Display<StrataState>,
		seat_name: String,
		backend: WinitGraphicsBackend<GlowRenderer>,
		damage_tracker: OutputDamageTracker,
	) -> Result<(), StrataState> {
		self.inner.set(StrataState::new(
			loop_handle,
			loop_signal,
			display,
			seat_name,
			backend,
			damage_tracker,
		))
	}
	pub fn get(&self) -> &StrataState {
		self.inner.get().expect("State not initialized")
	}
}

impl Deref for GlobalState {
	type Target = StrataState;

	fn deref(&self) -> &Self::Target {
		&self.get()
	}
}
