use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	DeriveInput,
	Field,
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

	quote!{}.into()
}
