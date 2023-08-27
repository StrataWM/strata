use mlua::{
	Function,
	IntoLua,
	RegistryKey,
};

#[derive(Debug, Default)]
pub struct Config {
	pub options: Options,
	pub bindings: Vec<Keybinding>,
	pub rules: Vec<Rule>,
}

#[derive(Debug, Default)]
pub struct Options {
	pub autostart: Vec<Cmd>,
	pub general: General,
	pub decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
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

#[derive(Debug, Default)]
pub struct General {
	pub workspaces: u8,
	pub gaps_in: i32,
	pub gaps_out: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Default)]
pub struct WindowDecorations {
	pub border: Border,
	pub window: Window,
	pub blur: Blur,
	pub shadow: Shadow,
}

#[derive(Debug, Default)]
pub struct Border {
	pub enable: bool,
	pub width: u32,
	pub active: String,
	pub inactive: String,
	pub radius: f64,
}

#[derive(Debug, Default)]
pub struct Window {
	pub opacity: f64,
}

#[derive(Debug, Default)]
pub struct Blur {
	pub enable: bool,
	pub size: u32,
	pub passes: u32,
	pub optimize: bool,
}

#[derive(Debug, Default)]
pub struct Shadow {
	pub enable: bool,
	pub size: u32,
	pub blur: u32,
	pub color: String,
}

#[derive(Debug, Default)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Default)]
pub struct Animations {
	pub enable: bool,
}

#[derive(Debug, strata_derive::FromLua)]
pub struct Keybinding {
	pub keys: Vec<String>,
	pub action: LuaFunction,
}

#[derive(Debug, strata_derive::FromLua)]
pub struct Rule {
	pub triggers: Vec<Trigger>,
	pub action: LuaFunction,
}

#[derive(Debug, strata_derive::FromLua)]
pub struct Trigger {
	pub event: String,
	pub class_name: Option<String>,
	pub workspace: Option<i32>,
}

impl Config {
	pub fn set(&mut self, config: Config) {
		*self = config;
	}

	pub fn update(&mut self, config: Config) {
		// TODO
		unimplemented!()
	}
}
