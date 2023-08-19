use crate::libs::structs::config::*;
use mlua::{
	Function,
	Lua,
	LuaSerdeExt,
	Result,
	Table,
	Value,
};
use std::{
	env::var,
	fs::read_to_string,
};

struct StrataApi;

impl StrataApi {
	pub fn spawn(_: &Lua, cmd: String) -> Result<()> {
		println!("Spawning {}", cmd.to_string());
		Ok(())
	}

	pub fn set_bindings(lua: &Lua, bindings: Table) -> Result<()> {
		for key in bindings.sequence_values::<Table>() {
			let table: Table = key?.clone();
			let keys: Vec<String> = table.get("keys")?;
			let cmd: Function = table.get("action")?;
			lua.globals()
				.get::<&str, Table>("package")?
				.get::<&str, Table>("loaded")?
				.get::<&str, Table>("strata")?
				.get::<&str, Table>("bindings")?
				.set(keys.clone().concat(), cmd)?;
			CONFIG
				.bindings
				.write()
				.push(Keybinding { keys: keys.clone(), action: keys.clone().concat() });
		}
		Ok(())
	}

	pub fn set_rules(lua: &Lua, rules: Table) -> Result<()> {
		for rule in rules.sequence_values::<Table>() {
			let table: Table = rule?.clone();
			let action: Function = table.get("action").ok().unwrap();
			let rules_triggers: Table = table.clone().get::<&str, Table>("triggers").ok().unwrap();
			for trigger in rules_triggers.sequence_values::<Value>() {
				let triggers: Triggers = lua.from_value(trigger?)?;
				let action_name: String = format!(
					"{}{}{}",
					triggers.clone().event,
					triggers.clone().class_name,
					triggers.workspace.unwrap_or(-1)
				);
				let _ = lua
					.globals()
					.get::<&str, Table>("package")?
					.get::<&str, Table>("loaded")?
					.get::<&str, Table>("strata")?
					.get::<&str, Table>("bindings")?
					.set(action_name.clone(), action.clone())?;
				CONFIG
					.rules
					.write()
					.push(Rules { triggers: triggers.clone(), action: action_name });
			}
		}

		Ok(())
	}

	pub fn set_config(lua: &Lua, configs: Table) -> Result<()> {
		println!("Called!");
		{
			let mut options = CONFIG.options.write();

			options.autostart = lua.from_value(configs.get("autostart")?)?;
			options.general = lua.from_value(configs.get("general")?)?;
			options.window_decorations = lua.from_value(configs.get("decorations")?)?;
			options.tiling = lua.from_value(configs.get("tiling")?)?;
			options.animations = lua.from_value(configs.get("animations")?)?;
		}
		{
			let mut rules = CONFIG.rules.write();
			rules.clear();
			rules.append(&mut lua.from_value(configs.get("rules")?)?);
		}
		{
			let mut bindings = CONFIG.bindings.write();
			bindings.clear();
			bindings.append(&mut lua.from_value(configs.get("bindings")?)?);
		}

		Ok(())
	}

	pub fn get_config(_lua: &Lua, _args: Value) -> Result<()> {
		unimplemented!()
	}
}

pub fn parse_config() -> Result<()> {
	let lua = Lua::new();
	let lib_path = "/usr/lib/stratawm/lua"; // TODO: properly handle install location somehow
	let config_path = format!(
		"{home}/.config/strata/strata.lua",
		home = var("HOME").expect("This should always be set!!!")
	);
	let config_str = read_to_string(config_path)?;

	// Create a new module
	let api_submod = get_or_create_sub_module(&lua, "api").unwrap(); // TODO: remove unwrap

	// Create "spawn api" for strata.api that can triggers Function as needed.
	api_submod.set("spawn", lua.create_function(StrataApi::spawn)?)?;
	api_submod.set("set_bindings", lua.create_function(StrataApi::set_bindings)?)?;
	api_submod.set("set_rules", lua.create_function(StrataApi::set_rules)?)?;
	api_submod.set("set_config", lua.create_function(StrataApi::set_config)?)?;
	api_submod.set("get_config", lua.create_function(StrataApi::get_config)?)?;

	lua.load(&format!(
		"package.path = '{lib_path}/strata/?.lua;{lib_path}/strata/init.lua;' .. package.path"
	))
	.exec()?;
	lua.load(&config_str).exec()?;

	Ok(())
}

fn get_or_create_module<'lua>(lua: &'lua Lua, name: &str) -> anyhow::Result<mlua::Table<'lua>> {
	let globals = lua.globals();
	let package: Table = globals.get("package")?;
	let loaded: Table = package.get("loaded")?;

	let module = loaded.get(name)?;
	match module {
		Value::Nil => {
			let module = lua.create_table()?;
			loaded.set(name, module.clone())?;
			Ok(module)
		}
		Value::Table(table) => Ok(table),
		wat => {
			anyhow::bail!(
				"cannot register module {name} as package.loaded.{name} is already set to a value \
				 of type {type_name}",
				type_name = wat.type_name()
			)
		}
	}
}

fn get_or_create_sub_module<'lua>(lua: &'lua Lua, name: &str) -> anyhow::Result<mlua::Table<'lua>> {
	let strata_mod = get_or_create_module(lua, "strata")?;
	let sub = strata_mod.get(name)?;
	match sub {
		Value::Nil => {
			let sub = lua.create_table()?;
			strata_mod.set(name, sub.clone())?;
			Ok(sub)
		}
		Value::Table(sub) => Ok(sub),
		wat => {
			anyhow::bail!(
				"cannot register module strata.{name} as it is already set to a value of type \
				 {type_name}",
				type_name = wat.type_name()
			)
		}
	}
}
