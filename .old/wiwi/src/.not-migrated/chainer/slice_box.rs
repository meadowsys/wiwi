use super::{ chainer, chain_fn, ChainHalf, NonChainHalf, VecChain };
use std::mem::MaybeUninit;

chainer! {
	generics_decl: [T]
	generics: [T]
	chainer: SliceBoxChain
	nonchain: Box<[T]>
}

impl<T> SliceBoxChain<T> {
	#[inline]
	pub fn new_uninit(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		let uninit = VecChain::with_capacity(len);

		// SAFETY:
		// - the call to `VecChain::with_capacity` allocates enough capacity
		// - `MaybeUninit<T>` has no initialisation requirement, uninit is okay
		let uninit = unsafe { uninit.set_len(len) };

		uninit
			.into_nonchain()
			.into_boxed_slice()
			.into()
	}

	#[inline]
	pub fn new_zeroed(len: usize) -> SliceBoxChain<MaybeUninit<T>> {
		let mut this = Self::new_uninit(len);

		// SAFETY:
		// - `Self::new_uninit` allocates capacity for at least `len` elements,
		// so it is safe to write `len * size_of::<T>()` bytes
		// - `MaybeUninit<T>` has no initialisation requirement
		unsafe { this.as_nonchain_mut().as_mut_ptr().write_bytes(0, len) }

		this
	}
}

impl SliceBoxChain<f32> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f32::total_cmp)
	}
}

impl SliceBoxChain<f64> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f64::total_cmp)
	}
}

impl<T> SliceBoxChain<MaybeUninit<T>> {
	#[inline]
	pub unsafe fn assume_init(self) -> SliceBoxChain<T> {
		let ptr = Box::into_raw(self.into_nonchain());

		// SAFETY:
		// - ptr was obtained from `Box::into_raw`
		// - caller promises the all elements in slice are initialised `T`
		// - [MaybeUninit<T>] and [T] have same layout
		let this = unsafe { Box::from_raw(ptr as *mut [T]) };

		this.into()
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
