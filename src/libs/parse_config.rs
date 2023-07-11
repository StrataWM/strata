use crate::libs::structs::Keybind;
use colored::Colorize;
use dirs::config_dir;
use std::{
	fs::File,
	io::Read,
};
use toml::Value;

impl Keybind {
	fn new(key: &str, cmd: &str) -> Self {
		Self {
			key: key.to_string(),
			cmd: cmd.to_string(),
		}
	}
}

pub fn parse_config() -> Vec<Keybind> {
	let conf_path = match config_dir() {
		Some(mut path) => {
			path.push("strata");
			path
		}
		None => {
			println!(
				"{}",
				"Cannot access Strata config directory at ~/.config/strata\nCheck that your \
				 $XDG_CONFIG_DIR is set and the folder exists\nStrata will run normally but you \
				 are adviced to create the config directory"
					.red()
					.bold()
			);
			return vec![];
		}
	};

	let conf_file_path = format!("{}{}", conf_path.display(), "/strata.toml");
	println!("{}", conf_file_path);

	let mut conf_file =
		File::open(conf_file_path).expect("Couldn't open config file. Check if it exits");
	let mut conf_str = String::new();
	conf_file
		.read_to_string(&mut conf_str)
		.expect("Couldn't read config!");

	let config: Value = conf_str
		.parse()
		.expect("Couldn't parse TOML file. Check syntax");

	let keybinds = config["keybinds"]["bind"].as_array().unwrap();

	let mut return_keybinds = vec![];

	for bind in keybinds {
		let key = bind["key"].as_str().unwrap();
		let cmd = bind["command"].as_str().unwrap();
		return_keybinds.push(Keybind::new(key, cmd));
	}
	return return_keybinds;
}
