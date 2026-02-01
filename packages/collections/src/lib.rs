#![cfg_attr(feature = "nightly", feature(allocator_api))]

pub mod placeholder_map;

pub use placeholder_map::PlaceholderMap;

pub type DefaultHashBuilder = ahash::RandomState;
