// use strata_core::UpdateFromLua;
//
// use super::{
// 	LuaFunction,
// 	Rule,
// };
//
// #[derive(Debug, Default)]
// pub(super) struct Rules {
// 	pub list: Vec<Rule>,
// }
//
// impl Rules {
// 	pub fn add_sequence(&mut self, rules: Table, lua: &Lua) -> mlua::Result<()> {
// 		for value in rules.sequence_values::<Table>() {
// 			let value = value?;
// 			if value.contains_key("triggers")? {
// 				self.list.push(Rule::from_lua(Value::Table(value), lua)?);
// 			} else {
// 				self.add_sequence(value, lua)?;
// 			}
// 		}
//
// 		Ok(())
// 	}
// }
//
// impl Into<Vec<Rule>> for Rules {
// 	fn into(self) -> Vec<Rule> {
// 		self.list
// 	}
// }
