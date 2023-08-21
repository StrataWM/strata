use mlua::{
	FromLua,
	Function,
	IntoLua,
	Lua,
	RegistryKey,
	Table,
	Value,
};
use parking_lot::RwLock;
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct General {
	pub workspaces: u8,
	pub gaps_in: i32,
	pub gaps_out: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct WindowDecorations {
	pub border: Border,
	pub window: Window,
	pub blur: Blur,
	pub shadow: Shadow,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Border {
	pub width: u32,
	pub active: String,
	pub inactive: String,
	pub radius: f64,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Window {
	pub opacity: f64,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Blur {
	pub enable: bool,
	pub size: u32,
	pub passes: u32,
	pub optimize: bool,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Shadow {
	pub enable: bool,
	pub size: u32,
	pub blur: u32,
	pub color: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct Animations {
	pub enable: bool,
}

#[derive(Debug, Clone)]
pub struct Trigger {
	pub event: String,
	pub class_name: Option<String>,
	pub workspace: Option<i32>,
}

#[derive(Debug, Default)]
pub struct Rules {
	pub list: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
	pub triggers: Vec<Trigger>,
	pub action: LuaFunction,
}

#[derive(Debug)]
pub struct Keybinding {
	pub keys: Vec<String>,
	pub action: LuaFunction,
}

#[derive(Debug)]
pub struct LuaFunction {
	key: RegistryKey,
}

pub type Cmd = Vec<String>;

#[derive(Debug, Default, Clone)]
pub struct Options {
	pub autostart: Vec<Cmd>,
	pub general: General,
	pub decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
}

#[derive(Debug, Default)]
pub struct Config {
	pub options: RwLock<Options>,
	pub bindings: RwLock<Vec<Keybinding>>,
	pub rules: RwLock<Rules>,
}

impl LuaFunction {
	pub fn call<'lua, T: IntoLua<'lua>>(
		&'lua self,
		lua: &'lua mlua::Lua,
		args: T,
	) -> anyhow::Result<()> {
		lua.registry_value::<Function>(&self.key)?.call(args)?;
		Ok(())
	}
}

impl<'lua> FromLua<'lua> for LuaFunction {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		Ok(Self { key: lua.create_registry_value(Function::from_lua(value, lua)?)? })
	}
}

// TODO: make our own FromLua proc macro without a Clone requirement, then move everything that
// used serde to this
impl<'lua> FromLua<'lua> for Keybinding {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		let table = Table::from_lua(value, lua)?;

		Ok(Self { keys: table.get("keys")?, action: table.get("action")? })
	}
}

impl<'lua> FromLua<'lua> for Rules {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		let mut ret = Rules::default();

		ret.add_sequence(Table::from_lua(value, lua)?)?;

		Ok(ret)
	}
}

impl<'lua> FromLua<'lua> for Trigger {
	fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
		let table = Table::from_lua(value, _lua)?;

		Ok(Self {
			event: table.get("event")?,
			class_name: table.get("class_name")?,
			workspace: table.get("workspace")?,
		})
	}
}

impl Rules {
	pub fn add_sequence(&mut self, rules: Table) -> mlua::Result<()> {
		for value in rules.sequence_values::<Table>() {
			let value = value?;
			if value.contains_key("triggers")? {
				self.list
					.push(Rule { triggers: value.get("triggers")?, action: value.get("action")? });
			} else {
				self.add_sequence(value)?;
			}
		}

		Ok(())
	}
}
