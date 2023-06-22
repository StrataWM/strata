use colored::Colorize;
use dirs::config_dir;
use std::{
	fs::File,
	io::Read,
};
use toml::Value;

pub fn parse_config() {
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
			return;
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

	for bind in keybinds {
		let key = bind["key"].as_str().unwrap();
		let cmd = bind["command"].as_str().unwrap();
		println!("Key: {}\nCommand: {}\n", key, cmd);
	}
}
