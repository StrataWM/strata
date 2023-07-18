use serde_derive::Deserialize;
use smithay::{
	desktop::{
		Space,
		Window,
	},
	input::{
		Seat,
		SeatState,
	},
	reexports::{
		calloop::LoopSignal,
		wayland_server::Display,
	},
	wayland::{
		compositor::CompositorState,
		data_device::DataDeviceState,
		output::OutputManagerState,
		shell::xdg::XdgShellState,
		shm::ShmState,
	},
};
use std::ffi::OsString;

pub struct Strata {
	pub start_time: std::time::Instant,
	pub socket_name: OsString,

	pub space: Space<Window>,
	pub loop_signal: LoopSignal,

	// Smithay State
	pub compositor_state: CompositorState,
	pub xdg_shell_state: XdgShellState,
	pub shm_state: ShmState,
	pub output_manager_state: OutputManagerState,
	pub seat_state: SeatState<Strata>,
	pub data_device_state: DataDeviceState,

	pub seat: Seat<Self>,
}

pub struct CalloopData {
	pub state: Strata,
	pub display: Display<Strata>,
}

// Config structs
#[derive(Debug, Deserialize)]
pub struct AutostartCmd {
	pub cmd: String,
}

#[derive(Debug, Deserialize)]
pub struct Autostart {
	pub cmd: Vec<AutostartCmd>,
}

#[derive(Debug, Deserialize)]
pub struct General {
	pub win_gaps: u32,
	pub out_gaps: u32,
}

#[derive(Debug, Deserialize)]
pub struct WindowDecorations {
	pub border_width: u32,
	pub border_active: String,
	pub border_inactive: String,
	pub border_radius: u32,
	pub window_opacity: f64,
	pub blur_enable: bool,
	pub blur_size: u32,
	pub blur_passes: u32,
	pub blur_optimization: bool,
	pub shadows_enabled: bool,
	pub shadow_size: u32,
	pub shadow_blur: u32,
	pub shadow_color: String,
}

#[derive(Debug, Deserialize)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Deserialize)]
pub struct Animations {
	pub anim_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
	pub workspace: i32,
	pub class_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
	pub workspace: Vec<Workspace>,
	pub floating: Vec<Floating>,
}

#[derive(Debug, Deserialize)]
pub struct Floating {
	pub class_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	pub autostart: Autostart,
	pub general: General,
	pub window_decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
	pub rules: Rules,
}
