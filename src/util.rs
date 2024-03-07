// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_export]
macro_rules! enum_table {
    (

        $(#[$outer:meta])*
        $vis:vis struct $Name:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        $(#[$outer])*
        $vis struct $Name(pub $T);

        #[allow(dead_code,deprecated,unused_doc_comments,unused_attributes,unused_mut,unused_imports,non_upper_case_globals,clippy::assign_op_pattern,clippy::indexing_slicing,clippy::same_name_method,clippy::iter_without_into_iter,)]
        const _: () = {
		impl<'gc> piccolo::FromValue<'gc> for $Name {
			fn from_value(_: piccolo::Context<'gc>, value: piccolo::Value<'gc>) -> Result<Self, piccolo::TypeError> {
				match value {
					piccolo::Value::UserData(ud) => {
						let k = *ud.downcast_static::<$Name>().map_err(|_| {
							piccolo::TypeError { expected: stringify!($Name), found: value.type_name() }
						})?;

						Ok(k)
					}
					_ => Err(piccolo::TypeError { expected: stringify!($Name), found: value.type_name() }),
				}
			}
		}

		impl From<$T> for $Name {
			fn from(value: $T) -> $Name {
				$Name(value)
			}
		}

        impl $Name {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $vis const $Flag:$T = $value;
            )*

			pub fn from_name(name: &str) -> Option<$T> {
                match name {
                    $(
                        stringify!($Flag) => Some($value),
                    )*
                    _ => None,
                }
            }

            pub fn metatable<'gc>(ctx: piccolo::Context<'gc>) -> anyhow::Result<piccolo::Value<'gc>> {
				use piccolo::IntoValue;

                let meta = piccolo::Table::new(&ctx);
				let index = piccolo::Table::new(&ctx);

				let v_meta = piccolo::Table::new(&ctx);
				v_meta.set(
					ctx,
					piccolo::MetaMethod::ToString,
					piccolo::Callback::from_fn(&ctx, |ctx, _, mut stack| {
						let ud = stack.consume::<piccolo::UserData>(ctx)?;
						let this = ud.downcast_static::<$Name>()?;

						stack.push_front(format!("{:#?}", this.0).into_value(ctx));

						Ok(piccolo::CallbackReturn::Return)
					}),
				)?;

				$(
				let ud = piccolo::UserData::new_static(&ctx, $Name($value));
				ud.set_metatable(&ctx, Some(v_meta));
				index.set(ctx, stringify!($Flag), ud)?;
				)*

				meta.set(ctx, piccolo::MetaMethod::Index, index)?;
                meta.set(
                    ctx,
                    piccolo::MetaMethod::NewIndex,
                    piccolo::Callback::from_fn(&ctx, |_, _, _| {
                        Ok(piccolo::CallbackReturn::Return)
                    })
                )?;

                Ok(piccolo::Value::Table(meta))
            }
        }
        };
    }
}
