use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	Data,
	DataStruct,
	DeriveInput,
	Field,
	Fields,
	FieldsNamed,
	FieldsUnnamed,
	parse_macro_input
};

pub fn macro_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let DeriveInput {
		attrs,
		vis,
		ident,
		generics,
		data
	} = parse_macro_input!(input as DeriveInput);

	let result = match data {
		Data::Struct(DataStruct { struct_token: _, fields, semi_token }) => {
			match fields {
				Fields::Named(FieldsNamed { brace_token, named }) => {
					for field in named {
						// attrs, vis, mutability (RFC), ident, colon token, type
					}

					"".parse().unwrap()
				}
				Fields::Unnamed(FieldsUnnamed { paren_token, unnamed }) => {
					for field in unnamed {
						// attrs, vis, mutability (RFC), ident, colon token, type
					}
					"".parse().unwrap()
				}
				Fields::Unit => {
					quote! {
						#[automatically_derived]
						impl _wiwi_memory_usage::Static for #ident {
							const MEMORY_USAGE: usize = 0;
						}
					}
				}
			}
		}

		Data::Enum(e) => {
			quote! {
				compile_error!("enums not supported at this time");
			}
		}

		Data::Union(u) => {
			quote! {
				compile_error!("unions not supported at this time");
			}
		}
	};

	let result = quote! {
		const _: () = {
			use ::wiwi::memory_usage as _wiwi_memory_usage;
			#result
		};
	};

	result.into()
}
