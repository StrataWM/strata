use crate::libs::structs::config::*;
use mlua::{
	chunk,
	Lua,
	LuaSerdeExt,
	Result,
	Table,
	Value,
};
use std::path::PathBuf;

struct StrataApi;

impl StrataApi {
	pub fn spawn(_: &Lua, cmd: String) -> Result<()> {
		println!("Spawning {}", cmd.to_string());
		Ok(())
	}

	pub fn set_config(lua: &Lua, configs: Value) -> Result<()> {
		println!("Called!");

		let mut options = CONFIG.options.write();
		*options = lua.from_value(configs)?;

		Ok(())
	}

	pub fn get_config(_lua: &Lua, _args: Value) -> Result<()> {
		unimplemented!()
	}
}

pub fn parse_config(config_dir: PathBuf, lib_dir: PathBuf) -> Result<()> {
	let lua = Lua::new();
	let api_submod = get_or_create_module(&lua, "strata.api").unwrap(); // TODO: remove unwrap

	api_submod.set("spawn", lua.create_function(StrataApi::spawn)?)?;
	api_submod.set("set_config", lua.create_function(StrataApi::set_config)?)?;
	api_submod.set("get_config", lua.create_function(StrataApi::get_config)?)?;

	let config_path = config_dir.to_string_lossy();
	let lib_path = lib_dir.to_string_lossy();

	lua.load(chunk!(
		local paths = {
			$config_path .. "?.lua",
			$config_path .. "?/init.lua",
			$lib_path .. "/strata/?.lua",
			$lib_path .. "/?/init.lua",
		}
		for _, path in ipairs(paths) do
			package.path = path .. ";" .. package.path
		end

		require("config")
	))
	.exec()?;

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
