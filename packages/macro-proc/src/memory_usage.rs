use proc_macro::TokenStream;

#[cfg(all(
	feature = "memory-usage",
	not(feature = "memory-usage-nightly")
))]
pub fn memory_usage_cfg_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr_stream;
	let _ = item;

	TokenStream::new()
}

#[cfg(all(
	feature = "memory-usage",
	feature = "memory-usage-nightly"
))]
pub fn memory_usage_cfg_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr_stream;

	item
}

#[cfg(all(
	feature = "memory-usage",
	not(feature = "memory-usage-nightly")
))]
pub fn memory_usage_cfg_not_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr_stream;

	item
}

#[cfg(all(
	feature = "memory-usage",
	feature = "memory-usage-nightly"
))]
pub fn memory_usage_cfg_not_nightly(attr_stream: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr_stream;
	let _ = item;

	TokenStream::new()
}
