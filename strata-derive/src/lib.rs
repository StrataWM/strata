use proc_macro::TokenStream;
use quote::{
	quote,
	quote_spanned,
	spanned::Spanned,
};

#[proc_macro_derive(FromLua)]
pub fn from_lua_derive(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match input.data {
		syn::Data::Struct(ref struct_data) => {
			let struct_name = input.ident.clone();
			let struct_fields = struct_data.fields.iter().filter_map(|field| field.ident.as_ref());

			quote! {
				impl<'lua> mlua::FromLua<'lua> for #struct_name {
					fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
						let table = mlua::Table::from_lua(value, lua)?;
						Ok(Self {
							#(
								#struct_fields: table.get(stringify!(#struct_fields))?
							),*
						})
					}
				}
			}
		}
		_ => {
			quote_spanned! {
				input.__span() => compile_error!("FromLua can only be derived for structs");
			}
		}
	}
	.into()
}
