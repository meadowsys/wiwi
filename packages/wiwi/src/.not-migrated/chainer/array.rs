use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };
use std::mem::{ ManuallyDrop, MaybeUninit };
use std::ptr;

chainer! {
	generics_decl: [T, const N: usize]
	generics: [T, N]
	chainer: ArrayChain
	nonchain: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	#[inline]
	pub fn new_uninit() -> ArrayChain<MaybeUninit<T>, N> {
		let this = MaybeUninit::<[MaybeUninit<T>; N]>::uninit();

		// SAFETY: `MaybeUninit<T>` has no initialisation requirement, so
		// uninitialised `[MaybeUninit<T>; N]` is valid
		let this = unsafe { this.assume_init() };

		this.into()
	}

	#[inline]
	pub fn new_zeroed() -> ArrayChain<MaybeUninit<T>, N> {
		let this = MaybeUninit::<[MaybeUninit<T>; N]>::zeroed();

		// SAFETY: `MaybeUninit<T>` has no initialisation requirement, so
		// zeroed `[MaybeUninit<T>; N]` is valid
		let this = unsafe { this.assume_init() };

		this.into()
	}
}

impl<const N: usize> ArrayChain<f32, N> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f32::total_cmp)
	}
}

impl<const N: usize> ArrayChain<f64, N> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f64::total_cmp)
	}
}

impl<T, const N: usize> ArrayChain<MaybeUninit<T>, N> {
	/// Assumes all slots inside the array are initialised according to `T`'s
	/// requirements, and converts into an array of T
	///
	/// Note: this implementation is currently subpar, as it does fully copy `self`
	/// into a new container. I have to do this because, at the time of writing:
	///
	/// - `transmute` is a bit too dumb, and is not able to prove `[T; N]` and
	///   `[MaybeUninit<T>; N]` are guaranteed to be equal sized, even though
	///   we can see and prove it
	/// - `transmute_unchecked` is like `transmute` but without that size check,
	///   but it is unstable, and according to a code comment will almost certainly
	///   never be stabilised (reasoning is that it's too unsafe, too much power to
	///   give users :p, and to hopefully find other methods for achieving things
	///   without it so its no longer needed)
	/// - `MaybeUninit::array_assume_init` is unstable (it internally makes use of
	///   `transmute_unchecked`)
	///
	/// # Safety
	///
	/// All slots in `self` must be fully initialised with valid values of `T`.
	#[inline]
	pub unsafe fn assume_init(self) -> ArrayChain<T, N> {
		// TODO: this fn's impl is subpar (its copying), see note in doc comment

		let ptr = self.as_nonchain().as_ptr().cast::<[T; N]>();

		// SAFETY: this `ptr::read` call is valid because:
		// - `ptr` points to / is obtained from the [MaybeUninit<T>; N] inside ArrayChain
		// - `ptr`, type [MaybeUninit<T>; N] is cast to ptr of [T; N], which is
		//   valid because MaybeUninit<T> has same layout as T, so [MaybeUninit<T>; N]
		//   will have same layout as [T; N]
		// - caller promises that all elements in the array are initialised `T`
		unsafe { ptr::read(ptr).into() }
	}
}

impl<T, const N: usize> AsMut<[T]> for ArrayChain<T, N> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_nonchain_mut()
	}
}

impl<T, const N: usize> AsRef<[T]> for ArrayChain<T, N> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		self.as_nonchain()
	}
}

// TODO: as_ascii
// TODO: as_ascii_unchecked
// TODO: each_mut
// TODO: each_ref
// TODO: map
// TODO: rsplit_array_mut
// TODO: rsplit_array_ref
// TODO: split_array_mut
// TODO: split_array_ref
// TODO: transpose
// TODO: try_map

// TODO: trait impls
// TODO: Borrow<[T]>
// TODO: BorrowMut<[T]>
// TODO: ConstParamTy
// TODO: Eq
// TODO: From<&'a [T; N]>
// TODO: From<&[T; N]>
// TODO: From<&mut [T; N]>
// TODO: From<(T, T)>
// TODO: From<(T, T, T)>
// TODO: From<(T, T, T, T)>
// TODO: From<(T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T, T, T, T, T)>
// TODO: From<(T, T, T, T, T, T, T, T, T, T, T, T)>
// TODO: From<(T,)>
// TODO: From<Mask<T, N>>
// TODO: From<Simd<T, N>>
// TODO: From<[(K, V); N]>
// TODO: From<[(K, V); N]>
// TODO: From<[T; 10]>
// TODO: From<[T; 11]>
// TODO: From<[T; 12]>
// TODO: From<[T; 1]>
// TODO: From<[T; 2]>
// TODO: From<[T; 3]>
// TODO: From<[T; 4]>
// TODO: From<[T; 5]>
// TODO: From<[T; 6]>
// TODO: From<[T; 7]>
// TODO: From<[T; 8]>
// TODO: From<[T; 9]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[T; N]>
// TODO: From<[bool; N]>
// TODO: From<[u16; 8]>
// TODO: From<[u16; 8]>
// TODO: From<[u8; 16]>
// TODO: From<[u8; 16]>
// TODO: From<[u8; 4]>
// TODO: From<[u8; 4]>
// TODO: Hash
// TODO: Index<I>
// TODO: IndexMut<I>
// TODO: IntoIterator
// TODO: IntoIterator
// TODO: IntoIterator
// TODO: Ord
// TODO: PartialEq<&[U; N]>
// TODO: PartialEq<&[U; N]>
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&mut [U; N]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U]>
// TODO: PartialOrd
// TODO: Pattern<'a>
// TODO: Pattern<'a>
// TODO: SlicePattern
// TODO: StructuralPartialEq
// TODO: TryFrom<&'a [T]>
// TODO: TryFrom<&'a mut [T]>
// TODO: TryFrom<&[T]>
// TODO: TryFrom<&mut [T]>
// TODO: TryFrom<Box<[T]>>
// TODO: TryFrom<Vec<T, A>>
// TODO: TryFrom<Vec<T>>

// TODO: slice methods
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

// TODO: slice trait impls?
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
