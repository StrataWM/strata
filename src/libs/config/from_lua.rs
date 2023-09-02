use mlua::{
	FromLua,
	Function,
	Lua,
	Table,
	Value,
};
use strata_core::UpdateFromLua;

use super::{
	LuaFunction,
	Rule,
};

impl<'lua> FromLua<'lua> for LuaFunction {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		Ok(Self { key: lua.create_registry_value(Function::from_lua(value, lua)?)? })
	}
}

impl<'lua> UpdateFromLua<'lua> for LuaFunction {
	fn update_from_lua(&mut self, value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<()> {
		self.key = lua.create_registry_value(Function::from_lua(value, lua)?)?;
		Ok(())
	}
}

#[derive(Debug, Default)]
pub(super) struct Rules {
	pub list: Vec<Rule>,
}

impl Rules {
	pub fn add_sequence(&mut self, rules: Table, lua: &Lua) -> mlua::Result<()> {
		for value in rules.sequence_values::<Table>() {
			let value = value?;
			if value.contains_key("triggers")? {
				self.list.push(Rule::from_lua(Value::Table(value), lua)?);
			} else {
				self.add_sequence(value, lua)?;
			}
		}

		Ok(())
	}
}

impl<'lua> FromLua<'lua> for Rules {
	fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
		let mut ret = Rules::default();

		ret.add_sequence(Table::from_lua(value, lua)?, lua)?;

		Ok(ret)
	}
}

impl Into<Vec<Rule>> for Rules {
	fn into(self) -> Vec<Rule> {
		self.list
	}
}
