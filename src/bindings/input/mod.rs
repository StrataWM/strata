// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
	cell::RefCell,
	rc::Rc,
};

use piccolo as lua;

use crate::state::Compositor;

mod key;
mod modflags;

pub fn module<'gc>(
	ctx: lua::Context<'gc>,
	comp: Rc<RefCell<Compositor>>,
) -> anyhow::Result<lua::Value<'gc>> {
	let ud = lua::UserData::new_static(&ctx, comp.clone());
	let meta = lua::Table::new(&ctx);

	let index = lua::Table::new(&ctx);
	index.set(ctx, "Key", key::module(ctx, comp.clone())?)?;
	index.set(ctx, "Mod", modflags::module(ctx, comp.clone())?)?;

	meta.set(ctx, lua::MetaMethod::Index, index)?;
	ud.set_metatable(&ctx, Some(meta));

	Ok(lua::Value::UserData(ud))
}
