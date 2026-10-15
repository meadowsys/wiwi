use proc_macro::TokenStream;

#[cfg(feature = "chain")]
mod chain;

#[cfg(feature = "chain")]
#[proc_macro_attribute]
pub fn chain_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
	chain::chain_fn(attr, item)
}

#[cfg(feature = "err")]
mod err;

#[cfg(feature = "err")]
#[proc_macro_attribute]
pub fn err(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	err::err(attr_stream, item)
}

#[cfg(feature = "memory-usage")]
mod memory_usage;

#[cfg(feature = "memory-usage")]
#[proc_macro_attribute]
pub fn memory_usage_cfg_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	memory_usage::memory_usage_cfg_nightly(attr_stream, item)
}

#[cfg(feature = "memory-usage")]
#[proc_macro_attribute]
pub fn memory_usage_cfg_not_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	memory_usage::memory_usage_cfg_not_nightly(attr_stream, item)
}
