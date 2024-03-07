// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

mod key;
mod modflags;

use std::{
	cell::RefCell,
	rc::Rc,
};

use crate::state::StrataComp;
use piccolo::{
	self as lua,
};

pub fn module<'gc>(
	ctx: lua::Context<'gc>,
	comp: Rc<RefCell<StrataComp>>,
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
