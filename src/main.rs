mod libs;
use chrono::Local;
use lazy_static::lazy_static;
pub use libs::{
	backends::winit::init_winit,
	ctl::ctl,
	parse_config::parse_config,
	structs::{
		CalloopData,
		Config,
		Strata,
	},
};
use log::info;
use notify::{
	recommended_watcher,
	RecommendedWatcher,
	RecursiveMode::NonRecursive,
	Watcher,
};
use std::{
	env::var,
	error::Error,
	io::stdout,
	path::Path,
	sync::mpsc::channel,
	time::Duration,
};
use tracing_subscriber::fmt::writer::MakeWriterExt;
lazy_static! {
	static ref CONFIG: Config = parse_config();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config_path = format!(
		"{}/.config/strata/strata.toml",
		var("HOME").expect("This should always be set!!!")
	);
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

	let mut watcher = recommended_watcher(|res| {
		match res {
			Ok(event) => println!("event: {:?}", event),
			Err(e) => println!("watch error: {:?}", e),
		}
	})?;

	watcher.watch(Path::new(&config_path), NonRecursive);
	info!("Initializing Strata WM");
	info!("Parsing config...");
	info!("Initializing socket interface...");
	let _ = ctl();

	Ok(())
}
