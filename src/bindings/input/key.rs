// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
	cell::RefCell,
	rc::Rc,
};

use lua::FromValue;
use piccolo as lua;

use crate::{
	handlers::input::{
		Key,
		KeyPattern,
		ModFlags,
	},
	state::Compositor,
};

pub fn module<'gc>(
	ctx: lua::Context<'gc>,
	comp: Rc<RefCell<Compositor>>,
) -> anyhow::Result<lua::Value<'gc>> {
	let meta = lua::Table::from_value(ctx, Key::metatable(ctx)?)?;

	// local k = Key({ Mod.Control_L, Mod.Super_L }, Key.Escape, function(...) end)
	meta.set(
		ctx,
		lua::MetaMethod::Call,
		lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
			let (comp, mods, key, cb) =
				stack.consume::<(lua::UserData, ModFlags, Key, lua::Function)>(ctx)?;
			let comp = comp.downcast_static::<Rc<RefCell<Compositor>>>()?;

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
