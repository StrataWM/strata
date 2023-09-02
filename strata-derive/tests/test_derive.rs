use mlua::{
	FromLua,
	Lua,
	Value,
};
use strata_derive::Config;

#[derive(Debug, PartialEq, Default, Config)]
struct Foo {
	a: i32,
	#[config(flat)]
	b: String,
}

#[test]
fn test_config() {
	let lua = Lua::new();
	let table = lua.create_table().unwrap();
	table.set("b", "hello").unwrap();

	let foo: Foo = FromLua::from_lua(Value::Table(table), &lua).unwrap();

	assert_eq!(foo, Foo { a: 0, b: "hello".to_string() });
}
