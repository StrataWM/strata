use mlua::{
	FromLua,
	Lua,
	Result,
	Value,
};

pub trait UpdateFromLua<'lua>: FromLua<'lua> {
	fn update_from_lua(&mut self, value: Value<'lua>, lua: &'lua Lua) -> Result<()>;
}

macro_rules! impl_update_from_lua {
	($($ty:ty),*) => {
		$(
			impl<'lua> UpdateFromLua<'lua> for $ty {
				fn update_from_lua(&mut self, value: Value<'lua>, _: &'lua Lua) -> Result<()> {
					*self = Self::from_lua(value, &Lua::new())?;
					Ok(())
				}
			}
		)*
	};
}

impl_update_from_lua!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, String);

impl<'lua, T: FromLua<'lua>> UpdateFromLua<'lua> for Vec<T> {
	fn update_from_lua(&mut self, value: Value<'lua>, lua: &'lua Lua) -> Result<()> {
		*self = FromLua::from_lua(value, lua)?;
		Ok(())
	}
}

impl<'lua, T: FromLua<'lua>> UpdateFromLua<'lua> for Option<T> {
	fn update_from_lua(&mut self, value: Value<'lua>, lua: &'lua Lua) -> Result<()> {
		*self = FromLua::from_lua(value, lua)?;
		Ok(())
	}
}
