use crate::libs::structs::Keybind;
use colored::Colorize;
use std::{
	env::var,
	fs::File,
	io::Read,
};
use toml::Value;

impl Keybind {
	fn new(key: &str, cmd: &str) -> Self {
		Self { key: key.to_string(), cmd: cmd.to_string() }
	}
}

pub fn parse_config() -> Vec<Keybind> {
	let conf_file_path = format!(
		"{}/.config/strata/strata.toml",
		var("HOME").expect("This variable should be set!!!")
	);
	println!("{}", conf_file_path);

	let mut conf_file =
		File::open(conf_file_path).expect("Couldn't open config file. Check if it exits");
	let mut conf_str = String::new();
	conf_file.read_to_string(&mut conf_str).expect("Couldn't read config!");

	let config: Value = conf_str.parse().expect("Couldn't parse TOML file. Check syntax");

	let keybinds = config["keybinds"]["bind"].as_array().unwrap();

	let mut return_keybinds = vec![];

	for bind in keybinds {
		let key = bind["key"].as_str().unwrap();
		let cmd = bind["command"].as_str().unwrap();
		return_keybinds.push(Keybind::new(key, cmd));
	}
	println!("{:?}", return_keybinds);
	return return_keybinds;
}
