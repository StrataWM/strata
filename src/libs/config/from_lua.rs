use mlua::{
	FromLua,
	Function,
	Lua,
	Table,
	Value,
};

use super::{
	Cmd,
	Keybinding,
	LuaFunction,
	Rule,
};

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct General {
	pub workspaces: Option<u8>,
	pub gaps_in: Option<i32>,
	pub gaps_out: Option<i32>,
	pub kb_repeat: Option<Vec<i32>>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct WindowDecorations {
	pub border: Option<Border>,
	pub window: Option<Window>,
	pub blur: Option<Blur>,
	pub shadow: Option<Shadow>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Border {
	pub enable: Option<bool>,
	pub width: Option<u32>,
	pub active: Option<String>,
	pub inactive: Option<String>,
	pub radius: Option<f64>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Window {
	pub opacity: Option<f64>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Blur {
	pub enable: Option<bool>,
	pub size: Option<u32>,
	pub passes: Option<u32>,
	pub optimize: Option<bool>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Shadow {
	pub enable: Option<bool>,
	pub size: Option<u32>,
	pub blur: Option<u32>,
	pub color: Option<String>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Tiling {
	pub layout: Option<String>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Animations {
	pub enable: Option<bool>,
}

#[derive(Debug, Default)]
pub(super) struct Rules {
	pub list: Vec<Rule>,
}

#[derive(Debug, Default, strata_derive::FromLua)]
pub(super) struct Config {
	pub autostart: Vec<Cmd>,
	pub general: General,
	pub decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
	pub bindings: Vec<Keybinding>,
	pub rules: Rules,
}

impl<'lua> FromLua<'lua> for LuaFunction {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		Ok(Self { key: lua.create_registry_value(Function::from_lua(value, lua)?)? })
	}
}

impl<'lua> FromLua<'lua> for Rules {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		let mut ret = Rules::default();

		ret.add_sequence(Table::from_lua(value, lua)?)?;

		Ok(ret)
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

// TODO: Find an elegant way to remove or generate this boilerplate, and define the defaults elsewhere

impl Into<super::Config> for Config {
	fn into(self) -> super::Config {
		super::Config {
			options: super::Options {
				autostart: self.autostart,
				general: self.general.into(),
				decorations: self.decorations.into(),
				tiling: self.tiling.into(),
				animations: self.animations.into(),
			},
			bindings: self.bindings.into_iter().map(Into::into).collect(),
			rules: self.rules.list,
		}
	}
}

impl Into<super::General> for General {
	fn into(self) -> super::General {
		super::General {
			workspaces: self.workspaces.unwrap_or(10),
			gaps_in: self.gaps_in.unwrap_or(0),
			gaps_out: self.gaps_out.unwrap_or(0),
			kb_repeat: self.kb_repeat.unwrap_or(vec![]),
		}
	}
}

impl Into<super::WindowDecorations> for WindowDecorations {
	fn into(self) -> super::WindowDecorations {
		super::WindowDecorations {
			border: self.border.unwrap_or_default().into(),
			window: self.window.unwrap_or_default().into(),
			blur: self.blur.unwrap_or_default().into(),
			shadow: self.shadow.unwrap_or_default().into(),
		}
	}
}

impl Into<super::Border> for Border {
	fn into(self) -> super::Border {
		super::Border {
			enable: self.enable.unwrap_or(true),
			width: self.width.unwrap_or(1),
			active: self.active.unwrap_or("#ffffff".to_string()),
			inactive: self.inactive.unwrap_or("#ffffff".to_string()),
			radius: self.radius.unwrap_or(0.0),
		}
	}
}

impl Into<super::Window> for Window {
	fn into(self) -> super::Window {
		super::Window { opacity: self.opacity.unwrap_or(1.0) }
	}
}

impl Into<super::Blur> for Blur {
	fn into(self) -> super::Blur {
		super::Blur {
			enable: self.enable.unwrap_or(true),
			size: self.size.unwrap_or(10),
			passes: self.passes.unwrap_or(1),
			optimize: self.optimize.unwrap_or(true),
		}
	}
}

impl Into<super::Shadow> for Shadow {
	fn into(self) -> super::Shadow {
		super::Shadow {
			enable: self.enable.unwrap_or(true),
			size: self.size.unwrap_or(10),
			blur: self.blur.unwrap_or(10),
			color: self.color.unwrap_or("#000000".to_string()),
		}
	}
}

impl Into<super::Tiling> for Tiling {
	fn into(self) -> super::Tiling {
		super::Tiling { layout: self.layout.unwrap_or("bsp".to_string()) }
	}
}

impl Into<super::Animations> for Animations {
	fn into(self) -> super::Animations {
		super::Animations { enable: self.enable.unwrap_or(true) }
	}
}
