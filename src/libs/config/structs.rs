use mlua::{
	FromLua,
	Function,
	IntoLua,
	RegistryKey,
};
use smart_default::SmartDefault;
use strata_derive::Config;
use strum::EnumString;

use super::from_lua;

#[derive(Debug, Default, Config)]
pub struct Config {
	pub autostart: Vec<Cmd>,
	pub general: General,
	pub decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
	pub bindings: Vec<Keybinding>,
	#[config(from = from_lua::Rules)]
	pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct LuaFunction {
	pub(crate) key: RegistryKey,
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

pub type Cmd = Vec<String>;

#[derive(Debug, SmartDefault, Config)]
pub struct General {
	#[default(10)]
	pub workspaces: u8,
	#[default(5)]
	pub gaps_in: i32,
	#[default(10)]
	pub gaps_out: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Default, Config)]
pub struct WindowDecorations {
	pub border: Border,
	pub window: Window,
	pub blur: Blur,
	pub shadow: Shadow,
}

#[derive(Debug, SmartDefault, Config)]
pub struct Border {
	pub enable: bool,
	#[default(2)]
	pub width: u32,
	#[default("#ffffff")]
	pub active: String,
	#[default("#888888")]
	pub inactive: String,
	#[default(5.0)]
	pub radius: f64,
}

#[derive(Debug, SmartDefault, Config)]
pub struct Window {
	#[default(1.0)]
	pub opacity: f64,
}

#[derive(Debug, SmartDefault, Config)]
pub struct Blur {
	pub enable: bool,
	#[default(5)]
	pub size: u32,
	#[default(1)]
	pub passes: u32,
	#[default(true)]
	pub optimize: bool,
}

#[derive(Debug, SmartDefault, Config)]
pub struct Shadow {
	pub enable: bool,
	#[default(5)]
	pub size: u32,
	#[default(5)]
	pub blur: u32,
	#[default("#000000")]
	pub color: String,
}

#[derive(Debug, Default, Config)]
pub struct Tiling {
	pub layout: Layout,
}

#[derive(Debug, Default, EnumString, Config)]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
	#[default]
	Dwindle,
}

#[derive(Debug, SmartDefault, Config)]
pub struct Animations {
	#[default(true)]
	pub enable: bool,
}

#[derive(Debug, Config)]
pub struct Keybinding {
	#[config(flat)]
	pub keys: Vec<String>,
	#[config(flat)]
	pub action: LuaFunction,
}

#[derive(Debug, Config)]
pub struct Rule {
	#[config(flat)]
	pub triggers: Vec<Trigger>,
	#[config(flat)]
	pub action: LuaFunction,
}

#[derive(Debug, Config)]
pub struct Trigger {
	#[config(flat)]
	pub event: String,
	#[config(flat)]
	pub class_name: Option<String>,
	#[config(flat)]
	pub workspace: Option<i32>,
}

impl Config {
	pub fn set(&mut self, config: Config) {
		*self = config;
	}
}
