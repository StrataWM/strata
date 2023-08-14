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
}

pub struct Stratacmd;

impl Stratacmd {
	pub fn spawn(lua: &Lua, cmd: String) -> Result<Function> {
		let func = lua
			.load(format!(
				r#"
            local strata = require("strata")
            strata.api.spawn("{}")"#,
				cmd
			))
			.into_function()?;
		Ok(func)
	}

	pub fn set_bindings(lua: &Lua, bindings: Table) -> Result<()> {
		for key in bindings.sequence_values::<Table>() {
			let table: Table = key?.clone();
			let keys: Vec<String> = table.get("keys")?;
			let cmd: Function = table.get("cmd")?;
			let _ = lua
				.globals()
				.get::<&str, Table>("package")?
				.get::<&str, Table>("loaded")?
				.get::<&str, Table>("strata")?
				.get::<&str, Table>("bindings")?
				.set(keys.clone().concat(), cmd)?;
			CONFIG
				.lock()
				.unwrap()
				.bindings
				.push(Keybinding { keys: keys.clone(), func: keys.clone().concat() });
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
					.lock()
					.unwrap()
					.rules
					.push(Rules { triggers: triggers.clone(), action: action_name });
			}
		}

		Ok(())
	}

	pub fn set_config(lua: &Lua, configs: Table) -> Result<()> {
		for autostart in configs.clone().get::<&str, Table>("autostart")?.sequence_values::<Table>()
		{
			for value in autostart?.clone().sequence_values::<Value>() {
				let cmd: String = lua.from_value(value?.clone())?;
				CONFIG.lock().unwrap().autostart.cmd.push(cmd)
			}
		}
		let general: General = lua.from_value(configs.clone().get::<&str, Value>("general")?)?;
		println!("General: {:#?}", general);
		CONFIG.lock().unwrap().general = general;
		let decorations: WindowDecorations = lua.from_value(configs.clone().get("decorations")?)?;
		CONFIG.lock().unwrap().window_decorations = decorations;
		let tiling: Tiling = lua.from_value(configs.clone().get("tiling")?)?;
		CONFIG.lock().unwrap().tiling = tiling;
		let animation: Animations = lua.from_value(configs.clone().get("animations")?)?;
		CONFIG.lock().unwrap().animations = animation;
		let bindings = configs.clone().get::<&str, Table>("bindings")?;
		let _ = Stratacmd::set_bindings(&lua, bindings);
		let rules = configs.clone().get::<&str, Table>("rules")?;
		let _ = Stratacmd::set_rules(&lua, rules);

		let configs = CONFIG.lock().unwrap();
		println!("{:#?}", configs);
		Ok(())
	}
}

pub fn parse_config() -> Result<()> {
	let lua = Lua::new();
	let config_path =
		format!("{}/.config/strata/strata.lua", var("HOME").expect("This should always be set!!!"));
	let config_str = read_to_string(config_path).unwrap();
	println!("{}", config_str);

	// Create a new module
	let strata_mod = get_or_create_module(&lua, "strata").ok().unwrap();
	let cmd_submod = get_or_create_sub_module(&lua, "cmd").ok().unwrap();
	let api_submod = get_or_create_sub_module(&lua, "api").ok().unwrap();
	let _submod = get_or_create_sub_module(&lua, "bindings").ok().unwrap();

	// Create "spawn config" for strata.cmd to construct Function and use it later.
	cmd_submod.set("spawn", lua.create_function(Stratacmd::spawn).ok().unwrap())?;
	// Create "spawn api" for strata.api that can triggers Function as needed.
	api_submod.set("spawn", lua.create_function(StrataApi::spawn).ok().unwrap())?;

	strata_mod.set("set_bindings", lua.create_function(Stratacmd::set_bindings).ok().unwrap())?;
	strata_mod.set("set_rules", lua.create_function(Stratacmd::set_rules).ok().unwrap())?;
	strata_mod.set("set_config", lua.create_function(Stratacmd::set_config).ok().unwrap())?;

	lua.load(&config_str).exec().ok();

	Ok(())
}

pub fn get_or_create_module<'lua>(lua: &'lua Lua, name: &str) -> anyhow::Result<mlua::Table<'lua>> {
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
				"cannot register module {} as package.loaded.{} is already set to a value of type \
				 {}",
				name,
				name,
				wat.type_name()
			)
		}
	}
}

pub fn get_or_create_sub_module<'lua>(
	lua: &'lua Lua,
	name: &str,
) -> anyhow::Result<mlua::Table<'lua>> {
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
				"cannot register module strata.{name} as it is already set to a value of type {}",
				wat.type_name()
			)
		}
	}
}
