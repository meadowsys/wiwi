use crate::prelude_internal::*;
use core::cmp::Ordering;
use core::mem::MaybeUninit;

impl_chain_conversions! { [T] Vec<T> }

chain_fns! {
	impl and_mut [T] { doc "Vec" } Vec<T>;

	doc ["[T]::align_to"]("slice::align_to")
	unsafe fn align_to[U](inner, cb: impl FnOnce((&[T], &[U], &[T]))) {
		// SAFETY: caller promises to uphold safety invariants
		unsafe { cb(inner.align_to()) }
	}

	doc ["[T]::align_to_mut"]("slice::align_to_mut")
	unsafe fn align_to_mut[U](
		inner,
		cb: impl FnOnce((&mut [T], &mut [U], &mut [T]))
	) {
		// SAFETY: caller promises to uphold safety invariants
		unsafe { cb(inner.align_to_mut()) }
	}

	doc [Self]
	fn append(inner, other: &mut impl ChainConversions<Inner = Vec<T>>) {
		inner.append(other.as_inner_mut())
	}

	doc ["[T]::binary_search"]("slice::binary_search")
	fn binary_search(
		inner,
		x: &T,
		out: impl Output<Result<usize, usize>>
	) where {
		T: Ord
	} {
		out.write(inner.binary_search(x))
	}

	doc ["[T]::binary_search_by"]("slice::binary_search_by")
	fn binary_search_by(
		inner,
		f: impl FnMut(&T) -> Ordering,
		out: impl Output<Result<usize, usize>>
	) {
		out.write(inner.binary_search_by(f))
	}

	doc ["[T]::binary_search_by_key"]("slice::binary_search_by_key")
	fn binary_search_by_key[B](
		inner,
		b: &B,
		f: impl FnMut(&T) -> B,
		out: impl Output<Result<usize, usize>>
	) where {
		B: Ord
	} {
		out.write(inner.binary_search_by_key(b, f))
	}

	doc [Self]
	fn capacity(inner, out: impl Output<usize>) {
		out.write(inner.capacity())
	}

	doc [Self]
	fn clear(inner) {
		inner.clear()
	}

	// todo concat trait is unstable
	// fn concat[Item](inner, out: impl Output<<[T] as std::slice::Concat<Item>>::Output>)
	// where {
	// 	[T]: std::slice::Concat<Item>,
	// 	Item: ?Sized
	// } {
	// 	out.write(inner.concat())
	// }

	doc [Self]
	fn dedup(inner)
	where {
		T: PartialEq
	} {
		inner.dedup()
	}

	doc [Self]
	fn dedup_by(
		inner,
		same_bucket: impl FnMut(&mut T, &mut T) -> bool
	) {
		inner.dedup_by(same_bucket)
	}

	doc [Self]
	fn dedup_by_key[K](inner, key: impl FnMut(&mut T) -> K)
	where {
		K: PartialEq
	} {
		inner.dedup_by_key(key)
	}

	doc ["[T]::fill"] ("slice::fill")
	fn fill(inner, value: T) where {
		T: Clone
	} {
		inner.fill(value)
	}

	doc ["[T]::fill_with"] ("slice::fill_with")
	fn fill_with(inner, f: impl FnMut() -> T) {
		inner.fill_with(f)
	}

	doc [Self]
	fn insert(inner, index: usize, element: T) {
		inner.insert(index, element)
	}

	doc [Self]
	fn is_empty(inner, out: impl Output<bool>) {
		out.write(inner.is_empty())
	}

	doc [Self]
	fn len(inner, out: impl Output<usize>) {
		out.write(inner.len())
	}

	doc [Self]
	fn pop(inner, out: impl Output<Option<T>>) {
		out.write(inner.pop())
	}

	// todo: msrv 1.86
	// fn pop_if(
	// 	inner,
	// 	predicate: impl FnOnce(&mut T) -> bool,
	// 	out: impl Output<Option<T>>
	// ) => out.write(inner.pop_if(predicate));

	doc [Self]
	fn push(inner, value: T) {
		inner.push(value)
	}

	// todo unstable
	// doc "Vec::push_within_capacity"
	// fn push_within_capacity(inner, value: T, out: impl Output<Result<(), T>>) {
	// 	out.write(inner.push_within_capacity(value))
	// }

	doc [Self]
	fn remove(inner, index: usize, out: impl Output<T>) {
		out.write(inner.remove(index))
	}

	doc [Self]
	fn reserve(inner, additional: usize) {
		inner.reserve(additional)
	}

	doc [Self]
	fn reserve_exact(inner, additional: usize) {
		inner.reserve_exact(additional)
	}

	doc [Self]
	fn resize(inner, new_len: usize, value: T)
	where {
		T: Clone
	} {
		inner.resize(new_len, value)
	}

	doc [Self]
	fn resize_with(inner, new_len: usize, f: impl FnMut() -> T) {
		inner.resize_with(new_len, f)
	}

	doc [Self]
	fn retain(inner, f: impl FnMut(&T) -> bool) {
		inner.retain(f)
	}

	doc [Self]
	fn retain_mut(inner, f: impl FnMut(&mut T) -> bool) {
		inner.retain_mut(f)
	}

	doc [Self]
	unsafe fn set_len(inner, new_len: usize) {
		// SAFETY: caller promises to uphold safety invariants
		unsafe { inner.set_len(new_len) }
	}

	doc [Self]
	fn shrink_to(inner, min_capacity: usize) {
		inner.shrink_to(min_capacity)
	}

	doc [Self]
	fn shrink_to_fit(inner) {
		inner.shrink_to_fit()
	}

	doc ["[T]::sort"]("slice::sort")
	fn sort(inner)
	where {
		T: Ord
	} {
		inner.sort()
	}

	doc ["[T]::sort_by"]("slice::sort_by")
	fn sort_by(inner, compare: impl FnMut(&T, &T) -> Ordering) {
		inner.sort_by(compare)
	}

	doc ["[T]::sort_by_key"]("slice::sort_by_key")
	fn sort_by_key[K](inner, f: impl FnMut(&T) -> K)
	where {
		K: Ord
	} {
		inner.sort_by_key(f)
	}

	doc ["[T]::sort_by_cached_key"]("slice::sort_by_cached_key")
	fn sort_by_cached_key[K](inner, f: impl FnMut(&T) -> K)
	where {
		K: Ord
	} {
		inner.sort_by_cached_key(f)
	}

	doc [Self]
	fn spare_capacity_mut(inner, cb: impl FnOnce(&mut [MaybeUninit<T>])) {
		cb(inner.spare_capacity_mut())
	}

	// allocator
	// array_chunks
	// array_chunks_mut
	// array_windows
	// as_array
	// as_array_mut
	// as_ptr_mut
	// as_slice_mut
	// as_non_null
	// as_ptr
	// as_slice
	// drain
	// extend_from_slice
	// extend_from_within
	// extract_if
	// from_parts
	// from_parts_in
	// from_raw_parts
	// from_raw_parts_in
	// into_boxed_slice
	// into_flattened
	// into_parts
	// into_parts_with_alloc
	// into_raw_parts
	// into_raw_parts_with_alloc
	// leak
	// splice
	// split_at_spare_mut
	// split_off
	// swap_remove
	// truncate
	// try_reserve
	// try_reserve_exact

	// as_ascii
	// as_ascii_unchecked
	// as_bytes
	// as_bytes
	// as_bytes_mut
	// as_chunks
	// as_chunks_mut
	// as_chunks_unchecked
	// as_chunks_unchecked_mut
	// as_flattened
	// as_flattened_mut
	// as_array_mut
	// as_ptr_mut
	// as_ptr_range_mut
	// as_ptr
	// as_ptr_range
	// as_rchunks
	// as_rchunks_mut
	// as_simd
	// as_simd_mut
	// as_str
	// assume_init_drop
	// assume_init_mut
	// assume_init_ref
	// binary_search
	// binary_search_by
	// binary_search_by_key
	// chunk_by
	// chunk_by_mut
	// chunks
	// chunks_exact
	// chunks_exact_mut
	// chunks_mut
	// clone_from_slice
	// concat
	// connect
	// contains
	// copy_from_slice
	// copy_within
	// element_offset
	// ends_with
	// eq_ignore_ascii_case
	// escape_ascii
	// first
	// first_chunk
	// first_chunk_mut
	// first_mut
	// get
	// get_disjoint_mut
	// get_disjoint_unchecked_mut
	// get_mut
	// get_unchecked
	// get_unchecked_mut
	// is_ascii
	// is_empty
	// is_sorted
	// is_sorted_by
	// is_sorted_by_key
	// iter
	// iter_mut
	// join
	// last
	// last_chunk
	// last_chunk_mut
	// last_mut
	// len
	// make_ascii_lowercase
	// make_ascii_uppercase
	// partition_dedup
	// partition_dedup_by
	// partition_dedup_by_key
	// partition_point
	// rchunks
	// rchunks_exact
	// rchunks_exact_mut
	// rchunks_mut
	// repeat
	// reverse
	// rotate_left
	// rotate_right
	// rsplit
	// rsplit_mut
	// rsplit_once
	// rsplitn
	// rsplitn_mut
	// select_nth_unstable
	// select_nth_unstable_by
	// select_nth_unstable_by_key
	// sort_floats
	// sort_floats
	// sort_unstable
	// sort_unstable_by
	// sort_unstable_by_key
	// split
	// split_at
	// split_at_checked
	// split_at_mut
	// split_at_checked_mut
	// split_at_unchecked_mut
	// split_at_unchecked
	// split_first
	// split_first_chunk
	// split_first_chunk_mut
	// split_first_mut
	// split_inclusive
	// split_inclusive_mut
	// split_last
	// split_last_chunk
	// split_last_chunk_mut
	// split_last_mut
	// split_mut
	// split_off
	// split_off_first
	// split_off_first_mut
	// split_off_last
	// split_off_last_mut
	// split_off_mut
	// split_once
	// splitn
	// splitn_mut
	// starts_with
	// strip_prefix
	// strip_suffix
	// subslice_range
	// swap
	// swap_unchecked
	// swap_with_slice
	// to_ascii_lowercase
	// to_ascii_uppercase
	// to_vec
	// to_vec_in
	// trim_ascii
	// trim_ascii_end
	// trim_ascii_start
	// utf8_chunks
	// windows
	// write_clone_of_slice
	// write_copy_of_slice
}

/*
Trait Implementations
AsMut<Vec<T, A>>
AsMut<[T]>
AsRef<Vec<T, A>>
AsRef<[T]>
Borrow<[T]>
BorrowMut<[T]>
Clone
Debug
Default
Deref
DerefMut
DerefPure
Drop
Eq
Extend<&'a T>
Extend<T>
From<&'a Vec<T>>
From<&[T; N]>
From<&[T]>
From<&mut [T; N]>
From<&mut [T]>
From<&str>
From<BinaryHeap<T, A>>
From<Box<[T], A>>
From<ByteString>
From<CString>
From<Cow<'a, [T]>>
From<String>
From<Vec<NonZero<u8>>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T>>
From<VecDeque<T, A>>
From<[T; N]>
FromIterator<T>
Hash
Index<I>
IndexMut<I>
IntoIterator
IntoIterator
IntoIterator
Ord
PartialEq<&[U; N]>
PartialEq<&[U]>
PartialEq<&mut [U]>
PartialEq<ByteStr>
PartialEq<ByteString>
PartialEq<Vec<U, A2>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<u8>>
PartialEq<Vec<u8>>
PartialEq<[U; N]>
PartialEq<[U]>
PartialOrd<Vec<T, A2>>
TryFrom<Vec<T, A>>
TryFrom<Vec<T>>
TryFrom<Vec<u8>>
Write
*/
