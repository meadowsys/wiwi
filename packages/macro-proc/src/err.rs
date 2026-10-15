use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
	ItemEnum,
	parse_macro_input
};

pub fn err(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr_stream;

	// todo figure this out (enforce just using it as ex. #[err] with no extra stuff)
	// let attr = parse_macro_input!(attr_stream as Attribute);
	// let attr_stream = TokenStream2::from(attr_stream);

	// if !matches!(attr.style, AttrStyle::Outer) {
	// 	return syn::Error::new_spanned(attr_stream, "use this macro only as an outer macro").into_compile_error()
	// }

	// if !matches!(attr.meta, Meta::Path(_)) {
	// 	return syn::Error::new_spanned(attr_stream, "macro should not contain any meta").into_compile_error()
	// }

	let item_enum = {
		let item = item.clone();
		parse_macro_input!(item as ItemEnum)
	};

	let item = TokenStream2::from(item);
	let item_name = item_enum.ident;
	quote! {
		#[derive(
			::wiwi_err::__proc_macro_support::Debug,
			::wiwi_err::__proc_macro_support::Display,
		)]
		#item

		const _: () = {
			impl ::wiwi_err::__proc_macro_support::Error for #item_name {}
		};
	}.into()
}
