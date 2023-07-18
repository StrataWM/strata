use crate::libs::structs::Config;
use std::{
	env::var,
	fs::read_to_string,
};
use toml::from_str;

pub fn parse_config() -> Config {
	let config_path = format!(
		"{}/.config/strata/strata.toml",
		var("HOME").expect("This should always be set!!!")
	);
	let file_str =
		read_to_string(config_path).expect("Couldn't read config file. Check if it exists");
	let config: Config = from_str(&file_str).expect("Failed to parse TOML");
	return config;
}
