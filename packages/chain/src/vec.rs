use super::{ ChainConversions, Output };
use core::cmp;

crate::decl_chain! {
	struct VecChain[T];
	impl[T] VecChain<T>;
	inner Vec<T>;
}

crate::decl_chain! {
	struct VecMutChain['h, T];
	impl['h, T] VecMutChain<'h, T>;
	inner &'h mut Vec<T>;
}

crate::impl_chain_conversions! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecMutChain<'h, T>;
	impl inner [T] Vec<T>;
	type inner Vec<T>;
	type mut_chain VecMutChain<'mut_chain, T>;
}

crate::chain_fns! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecMutChain<'h, T>;

	// align_to
	// align_to_mut
	// allocator
	underlying_fn "Vec::append"
	fn append(
		inner,
		other: &mut impl ChainConversions<Inner = Vec<T>>
	) {
		inner.append(other.as_inner_mut())
	}

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

	underlying_fn "[T]::binary_search"
	link_to "slice::binary_search"
	fn binary_search(
		inner,
		x: &T,
		out: impl Output<Result<usize, usize>>
	) where {
		T: Ord
	} {
		out.write(inner.binary_search(x))
	}

	underlying_fn "[T]::binary_search_by"
	link_to "slice::binary_search_by"
	fn binary_search_by(
		inner,
		f: impl FnMut(&T) -> cmp::Ordering,
		out: impl Output<Result<usize, usize>>
	) {
		out.write(inner.binary_search_by(f))
	}

	underlying_fn "[T]::binary_search_by_key"
	link_to "slice::binary_search_by_key"
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

	underlying_fn "Vec::capacity"
	fn capacity(inner, out: impl Output<usize>) {
		out.write(inner.capacity())
	}

	underlying_fn "Vec::clear"
	fn clear(inner) {
		inner.clear()
	}

	underlying_fn "Vec::dedup"
	fn dedup(inner)
	where {
		T: PartialEq
	} {
		inner.dedup()
	}

	underlying_fn "Vec::dedup_by"
	fn dedup_by(
		inner,
		same_bucket: impl FnMut(&mut T, &mut T) -> bool
	) {
		inner.dedup_by(same_bucket)
	}

	underlying_fn "Vec::dedup_by_key"
	fn dedup_by_key[K](inner, key: impl FnMut(&mut T) -> K)
	where {
		K: PartialEq
	} {
		inner.dedup_by_key(key)
	}

	// drain
	// extend_from_slice
	// extend_from_within
	// extract_if
	// from_parts
	// from_parts_in
	// from_raw_parts
	// from_raw_parts_in
	// insert
	// into_boxed_slice
	// into_flattened
	// into_parts
	// into_parts_with_alloc
	// into_raw_parts
	// into_raw_parts_with_alloc
	// is_empty
	// leak

	underlying_fn "Vec::len"
	fn len(inner, out: impl Output<usize>) {
		out.write(inner.len())
	}

	underlying_fn "Vec::pop"
	fn pop(inner, out: impl Output<Option<T>>) {
		out.write(inner.pop())
	}

	// todo: msrv 1.86
	// fn pop_if(
	// 	inner,
	// 	predicate: impl FnOnce(&mut T) -> bool,
	// 	out: impl Output<Option<T>>
	// ) => out.write(inner.pop_if(predicate));

	underlying_fn "Vec::push"
	fn push(inner, value: T) {
		inner.push(value)
	}

	// push_within_capacity
	// remove
	// reserve
	// reserve_exact
	// resize
	// resize_with
	// retain
	// retain_mut

	underlying_fn "Vec::set_len"
	unsafe fn set_len(inner, new_len: usize) {
		// SAFETY: caller promises to uphold safety invariants
		unsafe { inner.set_len(new_len) }
	}

	// shrink_to
	// shrink_to_fit
	// spare_capacity_mut
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
	// fill
	// fill_with
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
	// sort
	// sort_by
	// sort_by_cached_key
	// sort_by_key
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
