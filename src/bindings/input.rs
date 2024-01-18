use crate::handlers::input::{
	Key,
	ModFlags,
};
use piccolo::{
	self as lua,
	Callback,
	CallbackReturn,
	Context,
	FromValue,
	Function,
	MetaMethod,
	Table,
	UserData,
	Value,
};

pub mod modflags {
	use super::*;

	impl<'gc> FromValue<'gc> for ModFlags {
		fn from_value(
			_: lua::Context<'gc>,
			value: lua::Value<'gc>,
		) -> Result<Self, lua::TypeError> {
			match value {
				Value::Table(mods) => {
					let mut r = Self::empty();

					for (_, v) in mods {
						match v {
							Value::UserData(ud) => {
								let bits = *ud.downcast_static::<Self>().map_err(|_| {
									lua::TypeError { expected: "Mod", found: value.type_name() }
								})?;

								r |= bits;
							}
							_ => {
								return Err(lua::TypeError {
									expected: "Mod",
									found: v.type_name(),
								});
							}
						};
					}

					return Ok(r);
				}
				_ => Err(lua::TypeError { found: value.type_name(), expected: "table" }),
			}
		}
	}

	pub fn metatable<'gc>(ctx: Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let meta = Table::new(&ctx);

		meta.set(
			ctx,
			MetaMethod::Index,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (_, k) = stack.consume::<(Table, lua::String)>(ctx)?;

				let k = k.to_str()?;
				let bits =
					ModFlags::from_name(k).ok_or_else(|| anyhow::anyhow!("invalid key: {}", k))?;
				stack.push_front(Value::UserData(UserData::new_static(&ctx, bits)));

				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(meta)
	}

	pub fn module<'gc>(ctx: lua::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		let _ = m.set_metatable(&ctx, Some(metatable(ctx)?));

		Ok(m)
	}
}

pub mod keys {
	use super::*;

	impl<'gc> FromValue<'gc> for Key {
		fn from_value(_: Context<'gc>, value: Value<'gc>) -> Result<Self, lua::TypeError> {
			match value {
				Value::UserData(ud) => {
					let k = *ud.downcast_static::<Self>().map_err(|_| {
						lua::TypeError { expected: "Key", found: value.type_name() }
					})?;

					Ok(k)
				}
				_ => Err(lua::TypeError { expected: "Key", found: value.type_name() }),
			}
		}
	}

	pub fn metatable<'gc>(ctx: lua::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let meta = Table::new(&ctx);

		meta.set(
			ctx,
			MetaMethod::Index,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (_, k) = stack.consume::<(Table, lua::String)>(ctx)?;

				let k = k.to_str()?;
				let bits =
					Key::from_name(k).ok_or_else(|| anyhow::anyhow!("invalid key: {}", k))?;
				stack.push_front(Value::UserData(UserData::new_static(&ctx, bits)));

				Ok(CallbackReturn::Return)
			}),
		)?;

		// local k = Key({ Mod.Control_L, Mod.Super_L }, Key.Q, function(...) end)
		meta.set(
			ctx,
			MetaMethod::Call,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (_, mods, key, cb) = stack.consume::<(Table, ModFlags, Key, Function)>(ctx)?;
				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(meta)
	}

	pub fn module<'gc>(ctx: lua::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		let _ = m.set_metatable(&ctx, Some(metatable(ctx)?));

		Ok(m)
	}
}
