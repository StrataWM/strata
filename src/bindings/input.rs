use crate::handlers::input::ModFlags;
use piccolo::{
	Callback,
	CallbackReturn,
	Context,
	FromValue,
	Function,
	IntoValue,
	MetaMethod,
	Table,
	Value,
};

pub mod modflags {
	use super::*;

	impl<'gc> FromValue<'gc> for ModFlags {
		fn from_value(
			_: piccolo::Context<'gc>,
			value: piccolo::Value<'gc>,
		) -> Result<Self, piccolo::TypeError> {
			match value {
				Value::Table(mods) => {
					let mut r = Self::empty();

					if mods.length() == 0 {
						return Ok(r);
					} else {
						for (_, v) in mods {
							match v {
								Value::Integer(n) => {
									let Some(bits) = Self::from_bits(n as u8) else {
										return Err(piccolo::TypeError {
											found: v.type_name(),
											expected: "ModFlags(0..9)",
										});
									};

									r |= bits;
								}
								_ => {
									return Err(piccolo::TypeError {
										found: v.type_name(),
										expected: "ModFlags(0..9)",
									});
								}
							};
						}

						return Ok(r);
					}
				}
				_ => Err(piccolo::TypeError { found: value.type_name(), expected: "Table" }),
			}
		}
	}

	pub fn metatable<'gc>(ctx: Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let meta = Table::new(&ctx);

		meta.set(
			ctx,
			MetaMethod::Index,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (_, k) = stack.consume::<(Table, piccolo::String)>(ctx)?;

				let k = k.to_str()?;
				let bits = ModFlags::from_name(k).ok_or(Into::<piccolo::Error>::into(
					piccolo::String::from_slice(&ctx, format!("invalid index key: {}", k))
						.into_value(ctx),
				))?;
				stack.push_front(Value::Integer(bits.bits().into()));

				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(meta)
	}

	pub fn module<'gc>(ctx: piccolo::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		let _ = m.set_metatable(&ctx, Some(metatable(ctx)?));

		Ok(m)
	}
}

pub mod keys {
	use super::*;

	pub fn metatable<'gc>(ctx: piccolo::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let meta = Table::new(&ctx);

		// local k = Key({ Mod.Control_L, Mod.Super_L }, Key.Q, function(...) end)
		meta.set(
			ctx,
			MetaMethod::Call,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (_, mods, key, cb) = stack.consume::<(Table, ModFlags, u8, Function)>(ctx)?;
				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(meta)
	}

	pub fn module<'gc>(ctx: piccolo::Context<'gc>) -> anyhow::Result<Table<'gc>> {
		let m = Table::new(&ctx);

		let _ = m.set_metatable(&ctx, Some(metatable(ctx)?));

		Ok(m)
	}
}
