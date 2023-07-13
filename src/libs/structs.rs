use std::ffi::OsString;

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

#[derive(Debug)]
pub struct Keybind {
	pub key: String,
	pub cmd: String,
}

pub enum Action {
	Spawn(String),
}
