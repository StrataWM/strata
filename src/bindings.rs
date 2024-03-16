// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
	cell::RefCell,
	process::Command,
	rc::Rc,
};

use piccolo as lua;

use crate::state::Compositor;

pub mod input;

pub fn register<'gc>(ctx: lua::Context<'gc>, comp: Rc<RefCell<Compositor>>) -> anyhow::Result<()> {
	let index = lua::Table::new(&ctx);
	index.set(ctx, "input", input::module(ctx, comp.clone())?)?;
	index.set(
		ctx,
		"spawn",
		lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
			let (cmd, _) = stack.consume::<(lua::String, lua::Value)>(ctx)?;
			let _ = Command::new(cmd.to_str()?).spawn()?;

			Ok(lua::CallbackReturn::Return)
		}),
	)?;
	index.set(
		ctx,
		"quit",
		lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
			let comp = stack
				.consume::<lua::UserData>(ctx)?
				.downcast_static::<Rc<RefCell<Compositor>>>()?;

			comp.borrow_mut().quit();

			Ok(lua::CallbackReturn::Return)
		}),
	)?;

	let strata = lua::UserData::new_static(&ctx, comp.clone());

	let meta = lua::Table::new(&ctx);
	meta.set(ctx, lua::MetaMethod::Index, index)?;

	strata.set_metatable(&ctx, Some(meta));
	ctx.globals().set(ctx, "strata", strata)?;

	Ok(())
}
