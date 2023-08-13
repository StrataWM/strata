mod libs;
use chrono::Local;
use clap::Parser;
pub use libs::{
	backends::init_with_backend,
	parse_config::parse_config,
	structs::{
		args::Args,
		state::{CalloopData, StrataState},
	},
};
use log::info;
use std::{env::var, error::Error, io::stdout};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let _ = tokio::spawn(async move { parse_config() });
	let log_dir =
		format!("{}/.strata/stratawm", var("HOME").expect("This variable should be set!!!"));
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

	init_with_backend(&args.backend);

	info!("Initializing Strata WM");
	info!("Parsing config...");
	info!("Initializing socket interface...");

	Ok(())
}
