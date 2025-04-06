use crate::Chain;

pub type ArrayChain<T, const N: usize> = Chain<[T; N]>;
pub type ArrayMutChain<'h, T, const N: usize> = Chain<&'h mut [T; N]>;

pub type BTreeMapChain<K, V> = Chain<std::collections::BTreeMap<K, V>>;
pub type BTreeMapMutChain<'h, K, V> = Chain<&'h mut std::collections::BTreeMap<K, V>>;

pub type BTreeSetChain<T> = Chain<std::collections::BTreeSet<T>>;
pub type BTreeSetMutChain<'h, T> = Chain<&'h mut std::collections::BTreeSet<T>>;

pub type HashMapChain<K, V, S = std::hash::RandomState> = Chain<std::collections::HashMap<K, V, S>>;
pub type HashMapMutChain<'h, K, V, S = std::hash::RandomState> = Chain<&'h mut std::collections::HashMap<K, V, S>>;

pub type HashSetChain<T, S = std::hash::RandomState> = Chain<std::collections::HashSet<T, S>>;
pub type HashSetMutChain<'h, T, S = std::hash::RandomState> = Chain<&'h mut std::collections::HashSet<T, S>>;

pub type StringChain = Chain<String>;
pub type StringMutChain<'h> = Chain<&'h mut String>;

pub type VecChain<T> = Chain<Vec<T>>;
pub type VecMutChain<'h, T> = Chain<&'h mut Vec<T>>;

#[cfg(feature = "hashbrown")]
pub mod hashbrown {
	use crate::Chain;

	pub type HashMapChain<K, V, S = hashbrown::DefaultHashBuilder> = Chain<hashbrown::HashMap<K, V, S>>;
	pub type HashMapMutChain<'h, K, V, S = hashbrown::DefaultHashBuilder> = Chain<&'h mut hashbrown::HashMap<K, V, S>>;

	pub type HashSetChain<T, S = hashbrown::DefaultHashBuilder> = Chain<hashbrown::HashSet<T, S>>;
	pub type HashSetMutChain<'h, T, S = hashbrown::DefaultHashBuilder> = Chain<&'h mut hashbrown::HashSet<T, S>>;
}
