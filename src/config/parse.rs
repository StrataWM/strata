use crate::{
	state::ConfigCommands,
	CHANNEL,
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
use strata_core::UpdateFromLua;

struct StrataApi;

impl StrataApi {
	pub async fn spawn<'lua>(lua: &'lua Lua, cmd: Value<'lua>) -> Result<()> {
		let cmd: Vec<String> = FromLua::from_lua(cmd, lua)?;

		// TODO: add log

		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::Spawn(cmd.join(" "))).unwrap();

		Ok(())
	}

	pub fn switch_to_ws<'lua>(lua: &'lua Lua, id: Value<'lua>) -> Result<()> {
		let id: u8 = FromLua::from_lua(id, lua)?;

		// TODO: add log

		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::SwitchWS(id)).unwrap();

		Ok(())
	}

	pub fn move_window<'lua>(lua: &'lua Lua, id: Value<'lua>) -> Result<()> {
		let id: u8 = FromLua::from_lua(id, lua)?;

		// TODO: add log

		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::MoveWindow(id)).unwrap();

		Ok(())
	}

	pub fn move_window_and_follow<'lua>(lua: &'lua Lua, id: Value<'lua>) -> Result<()> {
		let id: u8 = FromLua::from_lua(id, lua)?;

		// TODO: add log

		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::MoveWindowAndFollow(id)).unwrap();

		Ok(())
	}

	pub fn close_window<'lua>(_lua: &'lua Lua, _: Value<'lua>) -> Result<()> {
		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::CloseWindow).unwrap();

		Ok(())
	}

	pub fn quit<'lua>(_lua: &'lua Lua, _: Value<'lua>) -> Result<()> {
		let channel = CHANNEL.lock().unwrap();
		channel.sender.send(ConfigCommands::Quit).unwrap();

		Ok(())
	}

	pub fn set_config(lua: &Lua, config: Value) -> Result<()> {
		CONFIG.write().set(FromLua::from_lua(config, lua)?);

		Ok(())
	}

	pub fn get_config(_lua: &Lua, _args: Value) -> Result<()> {
		// TODO
		unimplemented!()
	}

	pub fn update_config(lua: &Lua, args: Value) -> Result<()> {
		CONFIG.write().update_from_lua(args, lua)
	}
}

pub fn parse_config(config_dir: PathBuf, lib_dir: PathBuf) -> Result<()> {
	let lua = LUA.lock();
	let api_submod = get_or_create_module(&lua, "strata.api").unwrap(); // TODO: remove unwrap

	api_submod.set("close_window", lua.create_function(StrataApi::close_window)?)?;
	api_submod.set("switch_to_ws", lua.create_function(StrataApi::switch_to_ws)?)?;
	api_submod.set("move_window", lua.create_function(StrataApi::move_window)?)?;
	api_submod
		.set("move_window_and_follow", lua.create_function(StrataApi::move_window_and_follow)?)?;
	api_submod.set("quit", lua.create_function(StrataApi::quit)?)?;
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
