use std::{
	cell::RefCell,
	rc::{
		self,
		Rc,
	},
};

use crate::{
	handlers::input::{
		Key,
		ModFlags,
	},
	state::StrataComp,
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

mod modflags {
	use lua::IntoValue;

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
				let bits = UserData::new_static(&ctx, bits);

				let bits_meta = lua::Table::new(&ctx);
				bits_meta.set(
					ctx,
					MetaMethod::ToString,
					lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
						let this = stack.consume::<UserData>(ctx)?;
						let this = this.downcast_static::<ModFlags>()?;

						stack.push_front(format!("{:#?}", this).into_value(ctx));
						Ok(lua::CallbackReturn::Return)
					}),
				)?;
				bits.set_metatable(&ctx, Some(bits_meta));

				stack.push_front(Value::UserData(bits));

				Ok(CallbackReturn::Return)
			}),
		)?;

		Ok(meta)
	}

	pub fn module<'gc>(ctx: lua::Context<'gc>) -> anyhow::Result<lua::Value<'gc>> {
		let m = Table::new(&ctx);

		let _ = m.set_metatable(&ctx, Some(metatable(ctx)?));

		Ok(lua::Value::Table(m))
	}
}

mod keys {
	use crate::handlers::input::KeyPattern;

	use super::*;

	pub fn module<'gc>(
		ctx: lua::Context<'gc>,
		comp: Rc<RefCell<StrataComp>>,
	) -> anyhow::Result<Value<'gc>> {
		let key = lua::UserData::new_static(&ctx, comp);
		let meta = lua::Table::from_value(ctx, Key::metatable(ctx))?;

		// local k = Key({ Mod.XK_Control_L, Mod.XK_Super_L }, Key.XK_q, function(...) end)
		meta.set(
			ctx,
			MetaMethod::Call,
			Callback::from_fn(&ctx, |ctx, _, mut stack| {
				let (comp, mods, key, cb) =
					stack.consume::<(UserData, ModFlags, Key, Function)>(ctx)?;
				let comp = comp.downcast_static::<Rc<RefCell<StrataComp>>>()?;

				let keypat = KeyPattern { mods, key };

				comp.borrow_mut().config.keybinds.insert(keypat, ctx.stash(cb));

				println!("{:#?}: {:#?}", mods, key);

				Ok(CallbackReturn::Return)
			}),
		)?;

		key.set_metatable(&ctx, Some(meta));

		Ok(lua::Value::UserData(key))
	}
}

pub fn module<'gc>(
	ctx: lua::Context<'gc>,
	comp: Rc<RefCell<StrataComp>>,
) -> anyhow::Result<lua::Value<'gc>> {
	let input = lua::UserData::new_static(&ctx, comp.clone());
	let meta = lua::Table::new(&ctx);

	let index = lua::Table::new(&ctx);
	index.set(ctx, "Key", keys::module(ctx, comp)?)?;
	index.set(ctx, "Mod", modflags::module(ctx)?)?;

	meta.set(ctx, MetaMethod::Index, lua::Value::Table(index))?;
	input.set_metatable(&ctx, Some(meta));

	Ok(lua::Value::UserData(input))
}
