use crate::libs::structs::workspaces::Workspaces;
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
