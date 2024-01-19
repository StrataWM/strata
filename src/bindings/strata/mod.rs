use std::{
	cell::RefCell,
	rc::Rc,
};

use piccolo::{
	self as lua,
};

use crate::state::StrataComp;

use super::input;

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
			// let comp = ud.downcast_static::<Rc<RefCell<StrataComp>>>()?;

			let k = k.to_str()?;
			match k {
				"input" => {
					stack.push_front(input::module(ctx, comp.clone())?);
					Ok(lua::CallbackReturn::Return)
				}

				"workspaces" => Ok(lua::CallbackReturn::Return),

				"quit" => {
					stack.push_front(
						lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
							let comp = stack
								.consume::<lua::UserData>(ctx)?
								.downcast_static::<Rc<RefCell<StrataComp>>>()?;

							comp.borrow_mut().quit();

							Ok(lua::CallbackReturn::Return)
						})
						.into(),
					);
					Ok(lua::CallbackReturn::Return)
				}
				_ => Err(anyhow::anyhow!("invalid index key: {}", k).into()),
			}
		}),
	)?;

	meta.set(
		ctx,
		lua::MetaMethod::NewIndex,
		lua::Callback::from_fn(&ctx, |ctx, _, mut stack| {
			let (..) = stack.consume::<(lua::UserData, lua::String, lua::Value)>(ctx)?;

			Ok(lua::CallbackReturn::Return)
		}),
	)?;

	ud.set_metatable(&ctx, Some(meta));
	Ok(lua::Value::UserData(ud))
}
