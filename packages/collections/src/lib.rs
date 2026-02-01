#![cfg_attr(feature = "nightly", feature(allocator_api))]

pub mod placeholder_map;

#[doc(inline)]
pub use placeholder_map::PlaceholderMap;

pub type DefaultHashBuilder = ahash::RandomState;
