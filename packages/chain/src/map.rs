use crate::prelude_internal::*;

impl_chain_conversions!([K, V, S] std::collections::HashMap<K, V, S>);
impl_chain_conversions!(#[cfg(feature = "hashbrown")] [K, V, S] hashbrown::HashMap<K, V, S>);
