mod libs;
use clap::Parser;
pub use libs::{
	backends::winit::init_winit,
	parse_config::parse_config,
	structs::{
		CalloopData,
		Cli,
		Commands,
		Strata,
	},
};

use std::{
	error::Error,
	process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
	if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
		tracing_subscriber::fmt().with_env_filter(env_filter).init();
	} else {
		tracing_subscriber::fmt().init();
	}

	parse_config();
	let cli = Cli::parse();
	match &cli.command {
		Commands::Launch(backend) => {
			match backend.backend.as_str() {
				"winit" => {
					init_winit();
				}
				"udev" => {
					println!("TTY-Udev is not implement yet");
				}
				&_ => {
					println!(
						"No backend provided or unknown backend. Please choose one of these: \
						 \"winit\" / \"udev\""
					);
				}
			}
		}
		Commands::Quit(_) => {
			println!("Quitting");
			std::process::Command::new("sh")
				.arg("-c")
				.arg("killall strata")
				.output()
				.expect("failed to execute process");
		}
	}

	Ok(())
}
