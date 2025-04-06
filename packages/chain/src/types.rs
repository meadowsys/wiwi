use crate::Chain;

pub type ArrayChain<T, const N: usize> = Chain<[T; N]>;
pub type ArrayMutChain<'h, T, const N: usize> = Chain<&'h mut [T; N]>;

pub type HashMapChain<K, V, S = std::hash::RandomState> = Chain<std::collections::HashMap<K, V, S>>;
pub type HashMapMutChain<'h, K, V, S = std::hash::RandomState> = Chain<&'h mut std::collections::HashMap<K, V, S>>;

pub type StringChain = Chain<String>;
pub type StringMutChain<'h> = Chain<&'h mut String>;

pub type VecChain<T> = Chain<Vec<T>>;
pub type VecMutChain<'h, T> = Chain<&'h mut Vec<T>>;

pub mod hashbrown {
	use crate::Chain;

	pub type HashMapChain<K, V, S = std::hash::RandomState> = Chain<hashbrown::HashMap<K, V, S>>;
	pub type HashMapMutChain<'h, K, V, S = std::hash::RandomState> = Chain<&'h mut hashbrown::HashMap<K, V, S>>;
}
