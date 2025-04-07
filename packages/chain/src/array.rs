use crate::prelude_internal::*;

impl_chain_conversions! { [T, const N: usize] [T; N] }

chain_fns! {
	impl and_mut [T, const N: usize] [T; N];

	doc ["[T; N]::as_slice"]("array::as_slice")
	fn as_slice(inner, cb: impl FnOnce(&[T])) {
		cb(inner.as_slice())
	}

	doc ["[T; N]::each_ref"]("array::each_ref")
	fn each_ref(inner, cb: impl FnOnce([&T; N])) {
		cb(inner.each_ref())
	}

	doc ["[T; N]::each_mut"]("array::each_mut")
	fn each_mut(inner, cb: impl FnOnce([&mut T; N])) {
		cb(inner.each_mut())
	}
}

chain_fns! {
	impl [T, const N: usize] [T; N];

	doc ["[T; N]::map"]("array::map")
	fn map[T2](inner, f: impl FnMut(T) -> T2) -> [T2; N] {
		inner.map(f)
	}
}

/*
Methods
as_ascii
as_ascii_unchecked
as_mut_slice
map
rsplit_array_mut
rsplit_array_ref
split_array_mut
split_array_ref
transpose
transpose
try_map
Trait Implementations
AsMut<[T; N]>
AsMut<[T]>
AsRef<[T; N]>
AsRef<[T]>
Borrow<[T]>
BorrowMut<[T]>
Clone
ConstParamTy_
Copy
Debug
Default
Eq
From<&'a [T; N]>
From<&[T; N]>
From<&mut [T; N]>
From<(T,)>
From<Mask<T, N>>
From<Simd<T, N>>
From<[(K, V); N]>
From<[(K, V); N]>
From<[T; 1]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[T; N]>
From<[bool; N]>
From<[u8; 4]>
From<[u8; 4]>
From<[u8; 16]>
From<[u8; 16]>
From<[u16; 8]>
From<[u16; 8]>
Hash
Index<I>
IndexMut<I>
IntoIterator
IntoIterator
IntoIterator
Ord
PartialEq<&[U; N]>
PartialEq<&[U; N]>
PartialEq<&[U]>
PartialEq<&[u8; N]>
PartialEq<&[u8; N]>
PartialEq<&mut [U; N]>
PartialEq<&mut [U]>
PartialEq<ByteStr>
PartialEq<ByteStr>
PartialEq<ByteString>
PartialEq<ByteString>
PartialEq<[U; N]>
PartialEq<[U; N]>
PartialEq<[U; N]>
PartialEq<[U; N]>
PartialEq<[U; N]>
PartialEq<[U; N]>
PartialEq<[U]>
PartialEq<[u8; N]>
PartialEq<[u8; N]>
PartialOrd
Pattern
Pattern
SlicePattern
StructuralPartialEq
TryFrom<&'a [T]>
TryFrom<&'a mut [T]>
TryFrom<&[T]>
TryFrom<&mut [T]>
TryFrom<Box<[T]>>
TryFrom<Vec<T, A>>
TryFrom<Vec<T>>
UnsizedConstParamTy
*/
