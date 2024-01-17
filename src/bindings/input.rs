use crate::handlers::input::ModFlags;
use piccolo::{
	Callback,
	CallbackReturn,
	Context,
	FromValue,
	Function,
	MetaMethod,
	String as PiccoloString,
	Table,
	Value,
};

mod modflags {
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
								Value::Integer(bits) => {
									let Some(n) = Self::from_bits(bits as u8) else {
										return Err(piccolo::TypeError {
											found: v.type_name(),
											expected: "ModFlags(0..9)",
										});
									};

									r |= n;
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
				let (_, k) = stack.consume::<(Table, PiccoloString)>(ctx)?;

				match k.as_bytes() {
					b"Shift_L" => stack.push_front(Value::Integer(ModFlags::Shift_L.bits() as i64)),
					b"Shift_R" => stack.push_front(Value::Integer(ModFlags::Shift_R.bits() as i64)),
					b"Control_L" => {
						stack.push_front(Value::Integer(ModFlags::Control_L.bits() as i64))
					}
					b"Control_R" => {
						stack.push_front(Value::Integer(ModFlags::Control_R.bits() as i64))
					}
					b"Alt_L" => stack.push_front(Value::Integer(ModFlags::Alt_L.bits() as i64)),
					b"Alt_R" => stack.push_front(Value::Integer(ModFlags::Alt_R.bits() as i64)),
					b"Super_L" => stack.push_front(Value::Integer(ModFlags::Super_L.bits() as i64)),
					b"Super_R" => stack.push_front(Value::Integer(ModFlags::Super_R.bits() as i64)),
					b"ISO_Level3_Shift" => {
						stack.push_front(Value::Integer(ModFlags::ISO_Level3_Shift.bits() as i64))
					}
					b"ISO_Level5_Shift" => {
						stack.push_front(Value::Integer(ModFlags::ISO_Level5_Shift.bits() as i64))
					}
					_ => stack.push_front(Value::Integer(ModFlags::empty().bits() as i64)),
				};
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

mod key {
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
