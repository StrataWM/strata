use crate::libs::structs::config::Config;
use mlua::{
	Function,
	Lua,
	Result,
};
use std::{
	collections::HashMap,
	env::var,
	fs::read_to_string,
};

pub fn parse_config() -> Result<(Config, HashMap<Vec<String>, Function>)> {
	let lua = Lua::new();
	let config_path =
		format!("{}/.config/strata/strata.lua", var("HOME").expect("This should always be set!!!"));
	let config_str = read_to_string(config_path).unwrap();
	let globals = lua.globals();
	let strata: mlua::Table = lua.load("return require('strata')").eval()?;
	globals.set("strata", strata)?;
	let config: Config = lua.load(&config_str).eval()?;
	let bindings: mlua::Table = lua.load("return bindings").eval()?;
	let mut keybindings = HashMap::new();
	for pair in bindings.pairs::<mlua::Table, Function>() {
		let (key, func) = pair?;
		let key_combo: Vec<String> = key.sequence_values().collect::<Result<Vec<String>>>()?;
		keybindings.insert(key_combo, func);
	}
	Ok((config, keybindings))
}
