use crate::{
	CONFIG,
	LUA,
};
use mlua::{
	chunk,
	FromLua,
	Lua,
	Result,
	Table,
	Value,
};
use std::path::PathBuf;

use super::from_lua::{self,};

struct StrataApi;

impl StrataApi {
	pub async fn spawn<'lua>(lua: &'lua Lua, cmd: Value<'lua>) -> Result<()> {
		let cmd: Vec<String> = FromLua::from_lua(cmd, lua)?;

		tokio::spawn(async move {
			let mut child = tokio::process::Command::new(&cmd[0])
				.args(&cmd[1..])
				.spawn()
				.expect("failed to execute child");

			let ecode = child.wait().await.expect("failed to wait on child");
			println!("child process exited with: {}", ecode);
		})
		.await
		.map_err(|_| mlua::Error::RuntimeError("Failed to spawn process".to_owned()))?;

		// TODO: add log

		Ok(())
	}

	pub fn set_config(lua: &Lua, config: Value) -> Result<()> {
		let config: from_lua::Config = FromLua::from_lua(config, lua)?;
		CONFIG.write().set(config.into());

		Ok(())
	}

	pub fn get_config(_lua: &Lua, _args: Value) -> Result<()> {
		// TODO
		unimplemented!()
	}

	pub fn update_config(_lua: &Lua, _args: Value) -> Result<()> {
		// TODO
		unimplemented!()
	}
}

pub fn parse_config(config_dir: PathBuf, lib_dir: PathBuf) -> Result<()> {
	let lua = LUA.lock();
	let api_submod = get_or_create_module(&lua, "strata.api").unwrap(); // TODO: remove unwrap

	api_submod.set("spawn", lua.create_async_function(StrataApi::spawn)?)?;
	api_submod.set("set_config", lua.create_function(StrataApi::set_config)?)?;
	api_submod.set("get_config", lua.create_function(StrataApi::get_config)?)?;
	api_submod.set("update_config", lua.create_function(StrataApi::update_config)?)?;

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
	let loaded: Table = lua.globals().get::<_, Table>("package")?.get("loaded")?;
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
