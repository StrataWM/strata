mod libs;
use crate::libs::config::Config;
use chrono::Local;
use clap::Parser;
use lazy_static::lazy_static;
pub use libs::{
	backends::init_with_backend,
	structs::{
		args::Args,
		state::{
			CalloopData,
			StrataState,
		},
	},
};
use log::info;
use parking_lot::{
	ReentrantMutex,
	RwLock,
};
use std::{
	error::Error,
	io::stdout,
};
use tracing_subscriber::fmt::writer::MakeWriterExt;

lazy_static! {
	static ref LUA: ReentrantMutex<mlua::Lua> = ReentrantMutex::new(mlua::Lua::new());
	static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let xdg = xdg::BaseDirectories::with_prefix("strata")?;
	let log_dir = xdg.get_state_home();

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
	init_with_backend(args.backend).await?;

	info!("Initializing Strata WM");
	info!("Parsing config...");
	info!("Initializing socket interface...");

	Ok(())
}
