use mlua::{
	FromLua,
	Lua,
	Value,
};
use strata_derive::FromLua;

#[derive(Debug, PartialEq, FromLua)]
struct MyStruct {
	a: i32,
	b: String,
}

#[test]
fn test_from_lua() {
	let lua = Lua::new();
	let table = lua.create_table().unwrap();
	table.set("a", 1).unwrap();
	table.set("b", "hello").unwrap();

	let my_struct: MyStruct = FromLua::from_lua(Value::Table(table), &lua).unwrap();

	assert_eq!(my_struct, MyStruct { a: 1, b: "hello".to_string() });
}
