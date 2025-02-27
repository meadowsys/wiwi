use std::collections::{
	BTreeMap,
	BTreeSet,
	BinaryHeap,
	HashMap,
	HashSet,
	LinkedList,
	VecDeque
};
use super::*;

/// Trait providing [`into_chainer`], to convert any
/// supported type into one with a chaining API.
///
/// Every type that implements this trait has a "preferred" chain type, declared
/// by the [`IntoChainer::Chain`] associated type, and calling [`into_chainer`]
/// will convert it to that chain type.
///
/// [`into_chainer`]: IntoChainer::into_chainer
pub trait IntoChainer: Sized {
	/// The preferred chainer of this type
	type Chain: From<Self>;

	/// Converts `self` into its associated chain type
	///
	/// # Examples
	///
	/// ```
	/// # // bruh
	/// # use wiwi::chainer::{ IntoChainer, VecChain };
	/// let vec: Vec<String> = Vec::new();
	/// let chain: VecChain<String> = vec.into_chainer();
	/// // ...
	/// // do funny things with chaining API c:
	/// ```
	#[inline]
	fn into_chainer(self) -> Self::Chain {
		self.into()
	}
}

impl<T> IntoChainer for Box<[T]> {
	type Chain = SliceBoxChain<T>;
}

impl<'h, T> IntoChainer for &'h [T] {
	type Chain = SliceRefChain<'h, T>;
}

impl<'h, T> IntoChainer for &'h mut [T] {
	type Chain = SliceMutChain<'h, T>;
}

impl<T, const N: usize> IntoChainer for [T; N] {
	type Chain = ArrayChain<T, N>;
}

impl<'h, T, const N: usize> IntoChainer for &'h [T; N] {
	type Chain = ArrayRefChain<'h, T, N>;
}

impl<'h, T, const N: usize> IntoChainer for &'h mut [T; N] {
	type Chain = ArrayMutChain<'h, T, N>;
}

impl<T> IntoChainer for Vec<T> {
	type Chain = VecChain<T>;
}

impl<'h, T> IntoChainer for &'h Vec<T> {
	type Chain = VecRefChain<'h, T>;
}

impl<'h, T> IntoChainer for &'h mut Vec<T> {
	type Chain = VecMutChain<'h, T>;
}

impl IntoChainer for String {
	type Chain = StringChain;
}

impl<'h> IntoChainer for &'h String {
	type Chain = StringRefChain<'h>;
}

impl<'h> IntoChainer for &'h mut String {
	type Chain = StringMutChain<'h>;
}

impl IntoChainer for Box<str> {
	type Chain = StrBoxChain;
}

impl<'h> IntoChainer for &'h str {
	type Chain = StrRefChain<'h>;
}

impl<'h> IntoChainer for &'h mut str {
	type Chain = StrMutChain<'h>;
}

impl<T> IntoChainer for VecDeque<T> {
	type Chain = VecDequeChain<T>;
}

impl<T> IntoChainer for LinkedList<T> {
	type Chain = LinkedListChain<T>;
}

impl<K, V, S> IntoChainer for HashMap<K, V, S> {
	type Chain = HashMapChain<K, V, S>;
}

impl<T, S> IntoChainer for HashSet<T, S> {
	type Chain = HashSetChain<T, S>;
}

impl<K, V> IntoChainer for BTreeMap<K, V> {
	type Chain = BTreeMapChain<K, V>;
}

impl<T> IntoChainer for BTreeSet<T> {
	type Chain = BTreeSetChain<T>;
}

impl<T> IntoChainer for BinaryHeap<T> {
	type Chain = BinaryHeapChain<T>;
}

#[cfg(feature = "bitstream-unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
impl IntoChainer for crate::bitstream::Encoder {
	type Chain = super::BitstreamEncoderChain;
}
