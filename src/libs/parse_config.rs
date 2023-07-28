use crate::libs::structs::config::Config;
use mlua::{
	prelude::*,
	serde::Deserializer,
	Lua,
};
use std::{
	env::var,
	fs::read_to_string,
};

pub fn parse_config() -> Config {
	let config_path =
		format!("{}/.config/strata/strata.lua", var("HOME").expect("This should always be set!!!"));

	let raw_lua = read_to_string(config_path).expect("Couldn't read the config.");

	let lua_parser = Lua::new();

	lua_parser.load(&raw_lua).exec().expect("");

	let globals = lua_parser.globals();
	let returns: LuaTable = globals.get("return").expect("Couldn't fetch `return` values");
	let config: Config =
		returns.deserialize(returns, &lua_parser).expect("Error deserializing Lua table");

	return config;
}
