use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };
use std::ops::Range;
use std::{ mem, slice };

chainer! {
	generics_decl: ['h, T]
	generics: ['h, T]
	chainer: SliceRefChain
	nonchain: &'h [T]
}

impl<'h, T> SliceRefChain<'h, T> {
	#[inline]
	pub fn from_ref(val: &'h T) -> Self {
		slice::from_ref(val).into()
	}

	#[inline]
	pub unsafe fn from_raw_parts(data: *const T, len: usize) -> Self {
		// SAFETY: caller promises to uphold invariants of `slice::from_raw_parts`
		let slice = unsafe { slice::from_raw_parts(data, len) };

		slice.into()
	}

	// TODO: from_ptr_range nightly
}

impl<'h, T> SliceRefChain<'h, T> {}

impl<'h, T, const N: usize> SliceRefChain<'h, [T; N]> {
	#[inline]
	pub fn flatten(self) -> SliceRefChain<'h, T> {
		// TODO: use SizedTypeProperties / slice `flatten` when stabilised
		let len = if mem::size_of::<T>() == 0 {
			self.as_nonchain()
				.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			// TODO: use unchecked mul
			self.as_nonchain().len() * N
		};

		// ptr returned by `self.nc_ptr()` is valid for `self.len()` reads of `[T; N]`,
		// so casting it to `T` will mean it is valid for `self.len() * N` reads of `T`.
		// `len` var above is this multiplication amount
		let ptr = self.as_nonchain().as_ptr().cast::<T>();

		// SAFETY: `ptr` is valid for `len` reads (see comment above)
		let slice = unsafe { slice::from_raw_parts(ptr, len) };

		slice.into()
	}
}

// TODO: align_to
// TODO: align_to_mut
// TODO: array_chunks
// TODO: array_chunks_mut
// TODO: array_windows
// TODO: as_ascii
// TODO: as_ascii_unchecked
// TODO: as_bytes
// TODO: as_chunks
// TODO: as_chunks_mut
// TODO: as_chunks_unchecked
// TODO: as_chunks_unchecked_mut
// TODO: as_mut_ptr_range
// TODO: as_ptr_range
// TODO: as_rchunks
// TODO: as_rchunks_mut
// TODO: as_simd
// TODO: as_simd_mut
// TODO: as_str
// TODO: assume_init
// TODO: binary_search
// TODO: binary_search_by
// TODO: binary_search_by_key
// TODO: chunk_by
// TODO: chunk_by_mut
// TODO: chunks
// TODO: chunks_exact
// TODO: chunks_exact_mut
// TODO: chunks_mut
// TODO: clone_from_slice
// TODO: concat
// TODO: connect
// TODO: contains
// TODO: copy_from_slice
// TODO: copy_within
// TODO: ends_with
// TODO: eq_ignore_ascii_case
// TODO: escape_ascii
// TODO: fill
// TODO: fill_with
// TODO: first
// TODO: first_chunk
// TODO: first_chunk_mut
// TODO: first_mut
// TODO: flatten
// TODO: flatten_mut
// TODO: get
// TODO: get_many_mut
// TODO: get_many_unchecked_mut
// TODO: get_mut
// TODO: get_unchecked
// TODO: get_unchecked_mut
// TODO: into_vec
// TODO: is_ascii
// TODO: is_empty
// TODO: is_sorted
// TODO: is_sorted_by
// TODO: is_sorted_by_key
// TODO: iter
// TODO: iter_mut
// TODO: join
// TODO: last
// TODO: last_chunk
// TODO: last_chunk_mut
// TODO: last_mut
// TODO: len
// TODO: make_ascii_lowercase
// TODO: make_ascii_uppercase
// TODO: new_uninit_slice
// TODO: new_uninit_slice_in
// TODO: new_zeroed_slice
// TODO: new_zeroed_slice_in
// TODO: partition_dedup
// TODO: partition_dedup_by
// TODO: partition_dedup_by_key
// TODO: partition_point
// TODO: rchunks
// TODO: rchunks_exact
// TODO: rchunks_exact_mut
// TODO: rchunks_mut
// TODO: repeat
// TODO: reverse
// TODO: rotate_left
// TODO: rotate_right
// TODO: rsplit
// TODO: rsplit_mut
// TODO: rsplit_once
// TODO: rsplitn
// TODO: rsplitn_mut
// TODO: select_nth_unstable
// TODO: select_nth_unstable_by
// TODO: select_nth_unstable_by_key
// TODO: sort
// TODO: sort_by
// TODO: sort_by_cached_key
// TODO: sort_by_key
// TODO: sort_floats
// TODO: sort_floats
// TODO: sort_unstable
// TODO: sort_unstable_by
// TODO: sort_unstable_by_key
// TODO: split
// TODO: split_at
// TODO: split_at_checked
// TODO: split_at_mut
// TODO: split_at_mut_checked
// TODO: split_at_mut_unchecked
// TODO: split_at_unchecked
// TODO: split_first
// TODO: split_first_chunk
// TODO: split_first_chunk_mut
// TODO: split_first_mut
// TODO: split_inclusive
// TODO: split_inclusive_mut
// TODO: split_last
// TODO: split_last_chunk
// TODO: split_last_chunk_mut
// TODO: split_last_mut
// TODO: split_mut
// TODO: split_once
// TODO: splitn
// TODO: splitn_mut
// TODO: starts_with
// TODO: strip_prefix
// TODO: strip_suffix
// TODO: swap
// TODO: swap_unchecked
// TODO: swap_with_slice
// TODO: take
// TODO: take_first
// TODO: take_first_mut
// TODO: take_last
// TODO: take_last_mut
// TODO: take_mut
// TODO: to_ascii_lowercase
// TODO: to_ascii_uppercase
// TODO: to_vec
// TODO: to_vec_in
// TODO: trim_ascii
// TODO: trim_ascii_end
// TODO: trim_ascii_start
// TODO: try_new_uninit_slice
// TODO: try_new_zeroed_slice
// TODO: utf8_chunks
// TODO: windows

// TODO: trait impls
// TODO: AsMut<[T]>
// TODO: AsMut<[T]>
// TODO: AsMut<[T]>
// TODO: AsMut<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[T]>
// TODO: AsRef<[u8]>
// TODO: AsRef<[u8]>
// TODO: AsRef<[u8]>
// TODO: AsciiExt
// TODO: Borrow<[T]>
// TODO: Borrow<[T]>
// TODO: BorrowMut<[T]>
// TODO: BorrowMut<[T]>
// TODO: BufRead
// TODO: Concat<T>
// TODO: Concat<str>
// TODO: ConstParamTy
// TODO: Eq
// TODO: From<&'a [T]>
// TODO: From<&'data mut [MaybeUninit<u8>]>
// TODO: From<&'data mut [u8]>
// TODO: From<&[T]>
// TODO: From<&[T]>
// TODO: From<&[T]>
// TODO: From<&[T]>
// TODO: From<&mut [T]>
// TODO: From<Box<str, A>>
// TODO: From<Cow<'_, [T]>>
// TODO: From<Vec<T, A>>
// TODO: From<[T; N]>
// TODO: FromIterator<I>
// TODO: Hash
// TODO: Index<I>
// TODO: IndexMut<I>
// TODO: IntoIterator
// TODO: IntoIterator
// TODO: Join<&OsStr>
// TODO: Join<&T>
// TODO: Join<&[T]>
// TODO: Join<&str>
// TODO: Ord
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U]>
// TODO: PartialEq<[U]>
// TODO: PartialEq<[U]>
// TODO: PartialOrd
// TODO: Pattern<'a>
// TODO: Read
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SliceIndex<[T]>
// TODO: SlicePattern
// TODO: StructuralPartialEq
// TODO: ToOwned
// TODO: ToSocketAddrs
// TODO: TryFrom<&'a [T]>
// TODO: TryFrom<&'a mut [T]>
// TODO: TryFrom<&[T]>
// TODO: TryFrom<&[T]>
// TODO: TryFrom<&mut [T]>
// TODO: TryFrom<&mut [T]>
// TODO: Write
