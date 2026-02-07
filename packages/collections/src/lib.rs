#![cfg_attr(feature = "nightly", feature(
	allocator_api,
	hasher_prefixfree_extras
))]

mod hash_builder;

pub mod placeholder_map;

pub use hash_builder::DefaultHashBuilder;
#[doc(inline)]
pub use placeholder_map::PlaceholderMap;
