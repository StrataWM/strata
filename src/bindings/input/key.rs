use std::{
	cell::RefCell,
	rc::Rc,
};

use crate::{
	handlers::input::{
		Key,
		KeyPattern,
		ModFlags,
	},
	state::StrataComp,
};
use lua::FromValue;
use piccolo::{
	self as lua,
};

pub fn module<'gc>(
	ctx: lua::Context<'gc>,
	comp: Rc<RefCell<StrataComp>>,
) -> anyhow::Result<lua::Value<'gc>> {
	let meta = lua::Table::from_value(ctx, Key::metatable(ctx)?)?;

	// local k = Key({ Mod.Control_L, Mod.Super_L }, Key.Escape, function(...) end)
	meta.set(
		ctx,
		lua::MetaMethod::Call,
		lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
			let (comp, mods, key, cb) =
				stack.consume::<(lua::UserData, ModFlags, Key, lua::Function)>(ctx)?;
			let comp = comp.downcast_static::<Rc<RefCell<StrataComp>>>()?;

			let keypat = KeyPattern { mods, key };

			comp.borrow_mut().config.keybinds.insert(keypat, ctx.stash(cb));

			println!("{:#?}: {:#?}", mods, key);

			Ok(lua::CallbackReturn::Return)
		}),
	)?;

	let ud = lua::UserData::new_static(&ctx, comp);
	ud.set_metatable(&ctx, Some(meta));

	Ok(lua::Value::UserData(ud))
}
