mod libs;

use crate::libs::{
	backends::init_with_backend,
	config::{
		parse_config,
		Config,
	},
	structs::{
		args::Args,
		comms::{
			CommsChannel,
			ConfigCommands,
		},
	},
};
use chrono::Local;
use clap::Parser;
use crossbeam_channel::unbounded;
use lazy_static::lazy_static;
use log::info;
use parking_lot::{
	ReentrantMutex,
	RwLock,
};
use std::{
	error::Error,
	io::stdout,
	sync::{
		Arc,
		Mutex,
	},
};
use tracing_subscriber::fmt::writer::MakeWriterExt;

lazy_static! {
	static ref LUA: ReentrantMutex<mlua::Lua> = ReentrantMutex::new(mlua::Lua::new());
	static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
	static ref CHANNEL: Arc<Mutex<CommsChannel<ConfigCommands>>> = {
		let (sender, receiver) = unbounded();
		Arc::new(Mutex::new(CommsChannel { sender, receiver }))
	};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let xdg = xdg::BaseDirectories::with_prefix("strata")?;
	let config_dir = xdg.find_config_file("");
	let lib_dir = xdg.find_data_file("lua");
	let log_dir = xdg.get_state_home();

	if let (Some(config_path), Some(data_path)) = (config_dir, lib_dir) {
		tokio::spawn(async { parse_config(config_path, data_path) }).await??;
	}

	let file_appender = tracing_appender::rolling::never(
		&log_dir,
		format!("strata_{}.log", Local::now().format("%Y-%m-%d_%H:%M:%S")),
	);

	let latest_file_appender = tracing_appender::rolling::never(&log_dir, "latest.log");
	let log_appender = stdout.and(file_appender).and(latest_file_appender);

	if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
		tracing_subscriber::fmt().with_writer(log_appender).with_env_filter(env_filter).init();
	} else {
		tracing_subscriber::fmt().with_writer(log_appender).init();
	}

	let args = Args::parse();

	let channel = CHANNEL.lock().unwrap();
	channel.sender.send(ConfigCommands::CloseWindow);
	match channel.receiver.recv().unwrap() {
		ConfigCommands::CloseWindow => {
			println!("close")
		}
		ConfigCommands::Spawn(cmd) => {
			println!("{}", cmd)
		}
		ConfigCommands::SwitchWS(id) => {
			println!("Switching to workspace {}", id)
		}
	}
	init_with_backend(&args.backend);

	info!("Initializing Strata WM");
	info!("Parsing config...");
	info!("Initializing socket interface...");

	Ok(())
}
