use crate::libs::structs::state::{
	Backend,
	StrataState,
};
use anyhow::Context;
use log::info;
use std::{
	fs::{
		metadata,
		remove_file,
	},
	io::Read,
	os::unix::net::{
		UnixListener,
		UnixStream,
	},
};

pub fn ctl<BackendData: Backend>(state: &mut StrataState<BackendData>) -> anyhow::Result<()> {
	let socket_path: &str = "/tmp/strata_socket";

	if metadata(socket_path).is_ok() {
		info!("A socket is already present. Deleting it ...");
		remove_file(socket_path)
			.with_context(|| format!("Could not delete previous socket at {:?}", socket_path))?;
	}

	let unix_listener =
		UnixListener::bind(socket_path).context("Could not create the unix socket")?;

	loop {
		let (unix_stream, _socket_address) = unix_listener
			.accept()
			.context("Failed at accepting a connection on the unix listener")?;
		handle_stream(unix_stream, state)?;
	}
}

fn handle_stream<BackendData: Backend>(
	mut unix_stream: UnixStream,
	state: &mut StrataState<BackendData>,
) -> anyhow::Result<()> {
	info!("Connection established to Strata CTL!");
	let mut command = String::new();
	unix_stream.read_to_string(&mut command).context("Failed at reading the unix stream")?;

	match command.as_str() {
		ws_id if ws_id.starts_with("window move ") => {
			let id = ws_id.trim_start_matches("window move ").trim().parse::<u8>().unwrap();
			info!("Moving to workspace: {}", id);
			state.move_window_to_workspace(id);
		}
		ws_id if ws_id.starts_with("window move_and_follow ") => {
			let id =
				ws_id.trim_start_matches("window move_and_follow ").trim().parse::<u8>().unwrap();
			info!("Moving to workspace: {}", id);
			state.follow_window_move(id);
		}
		ws_id if ws_id.starts_with("workspace switch ") => {
			let id = ws_id.trim_start_matches("workspace switch ").trim().parse::<u8>().unwrap();
			info!("Switching to workspace: {}", id);
			state.switch_to_workspace(id);
		}
		"window close" => {
			info!("Closing current window.");
			state.close_window();
		}
		"quit" => {
			info!("Quitting Strata.");
			state.quit();
		}
		spawn_cmd if spawn_cmd.starts_with("spawn ") => {
			let program = spawn_cmd.trim_start_matches("spawn ").trim();
			info!("Spawning program: {}", program);
			state.spawn(program);
		}
		&_ => {}
	}

	Ok(())
}
