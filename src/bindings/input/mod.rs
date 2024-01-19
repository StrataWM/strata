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

	meta.set(
		ctx,
		lua::MetaMethod::Index,
		lua::Callback::from_fn(&ctx, move |ctx, _, mut stack| {
			let (_, k) = stack.consume::<(lua::UserData, lua::String)>(ctx)?;
			match k.as_bytes() {
				b"Key" => {
					let m = key::module(ctx, comp.clone())?;
					stack.push_front(m);
				}
				b"Mod" => {
					let m = modflags::module(ctx, comp.clone())?;
					stack.push_front(m);
				}
				_ => todo!(),
			};

			Ok(lua::CallbackReturn::Return)
		}),
	)?;

	ud.set_metatable(&ctx, Some(meta));

	Ok(lua::Value::UserData(ud))
}


