use proc_macro::TokenStream;
use quote::{
	quote,
	quote_spanned,
	spanned::Spanned,
};
use syn::Attribute;

struct ConfigAttrs {
	is_flat: bool,
	from_type: Option<syn::Type>,
}

impl<'a, T> From<T> for ConfigAttrs
where
	T: IntoIterator<Item = &'a Attribute>,
{
	fn from(iter: T) -> Self {
		let mut ret = Self { is_flat: false, from_type: None };

		for attr in iter {
			attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("from") {
					ret.from_type = Some(meta.value()?.parse()?);
				} else if meta.path.is_ident("flat") {
					ret.is_flat = true;
				} else {
					panic!("unknown attribute");
				}
				Ok(())
			})
			.expect("failed to parse attribute");
		}

		ret
	}
}

/// Procedural macro to derive a wrapper with optional fields around a struct.
#[proc_macro_derive(Config, attributes(config))]
pub fn config_derive(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match input.data {
		syn::Data::Struct(ref struct_data) => {
			let struct_name = input.ident;
			let fields = struct_data.fields.iter().map(|field| {
				let attrs = ConfigAttrs::from(
					field.attrs.iter().filter(|attr| attr.path().is_ident("config")),
				);
				(field.ident.as_ref().unwrap(), &field.ty, attrs)
			});

			let from_lua_fields = fields.clone().map(|(field_name, field_type, attrs)| {
				let field_type = attrs.from_type.as_ref().unwrap_or(field_type);
				let field_value = if attrs.is_flat {
					quote! {
						table.get::<_, #field_type>(stringify!(#field_name))?.into()
					}
				} else {
					quote! {
						table
							.get::<_, Option<#field_type>>(stringify!(#field_name))?
							.map(Into::into)
							.unwrap_or_default()
					}
				};

				quote! {
					#field_name: #field_value
				}
			});

			let update_from_lua_fields = fields.map(|(field_name, _field_type, attrs)| {
				match (attrs.is_flat, attrs.from_type) {
					(true, None) => {
						quote! {
							self.#field_name.update_from_lua(table.get(stringify!(#field_name))?, lua)?;
						}
					}
					(true, Some(from_type)) => {
						quote! {
							self.#field_name = table.get::<_, #from_type>(stringify!(#field_name))?.into();
						}
					}
					(false, None) => {
						quote! {
							if let Some(value) = table.get::<_, Option<mlua::Value>>(stringify!(#field_name))? {
								self.#field_name.update_from_lua(value, lua)?;
							}
						}
					}
					(false, Some(from_type)) => {
						quote! {
							if let Some(value) = table.get::<_, Option<#from_type>>(stringify!(#field_name))? {
								self.#field_name = value.into();
							}
						}
					}
				}
			});

			quote! {
				impl<'lua> mlua::FromLua<'lua> for #struct_name {
					fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
						let table = mlua::Table::from_lua(value, lua)?;
						Ok(Self {
							#(#from_lua_fields),*
						})
					}
				}

				impl<'lua> strata_core::UpdateFromLua<'lua> for #struct_name {
					fn update_from_lua(&mut self, value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<()> {
						let table = mlua::Table::from_lua(value, lua)?;
						#(#update_from_lua_fields)*
						Ok(())
					}
				}
			}
		}
		syn::Data::Enum(_) => {
			let enum_name = input.ident;
			quote! {
				impl<'lua> mlua::FromLua<'lua> for #enum_name {
					fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
						let str = String::from_lua(value, lua)?;
						 std::str::FromStr::from_str(&str).map_err(|_| mlua::Error::FromLuaConversionError {
							from: "string",
							to: stringify!(#enum_name),
							message: Some("invalid variant".to_owned()),
						})
					}
				}

				impl<'lua> strata_core::UpdateFromLua<'lua> for #enum_name {
					fn update_from_lua(&mut self, value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<()> {
						*self = Self::from_lua(value, lua)?;
						Ok(())
					}
				}
			}
		}
		_ => {
			quote_spanned! {
				input.ident.__span() => compile_error!("Config can only be derived for structs and enums");
			}
		}
	}
	.into()
}
