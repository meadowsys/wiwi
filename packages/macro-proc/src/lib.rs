use proc_macro::TokenStream;
use syn::parse_macro_input;

mod builder;
mod with_cloned;

#[proc_macro_attribute]
pub fn builder(attr: TokenStream, item: TokenStream) -> TokenStream {
	builder::builder(attr, parse_macro_input!(item)).into()
}

#[proc_macro]
pub fn with_cloned(input: TokenStream) -> TokenStream {
	match with_cloned::with_cloned(input.into()) {
		Ok(ts) | Err(ts) => { ts.into() }
	}
}

// #[cfg(feature = "memory-usage")]
// mod memory_usage;

// #[cfg(feature = "memory-usage")]
// #[proc_macro_derive(MemoryUsage, attributes(wiwi))]
// pub fn memory_usage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	memory_usage::macro_impl(input)
// }

// #[cfg(feature = "int")]
// mod int;

// /// Internal macro only, do not use
// #[cfg(feature = "int")]
// #[doc(hidden)]
// #[proc_macro]
// pub fn define_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	int::macro_impl(input)
// }

// #[cfg(feature = "serialiser-binary")]
// mod serialiser_binary;

// #[cfg(feature = "serialiser-binary")]
// #[proc_macro_derive(Serialise, attributes(wiwi))]
// pub fn serialise_binary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// 	serialiser_binary::macro_impl(input)
// }

// #[cfg(feature = "unicode")]
// mod unicode;
