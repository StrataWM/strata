mod libs;
use chrono::Local;
pub use libs::{
	backends::winit::init_winit,
	ctl::ctl,
	log::*,
	parse_config::parse_config,
	structs::{
		CalloopData,
		Strata,
	},
};
use log::{
	error,
	info,
};
use std::{
	env::var,
	error::Error,
	io::stdout,
};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let log_dir = format!("{}/.strata/", var("HOME").expect("This variable should be set!!!"));
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

	info!("Initializing Strata WM");
	info!("Parsing config...");
	let _ = parse_config();
	info!("Initializing socket interface...");
	let _ = ctl();

	Ok(())
}
