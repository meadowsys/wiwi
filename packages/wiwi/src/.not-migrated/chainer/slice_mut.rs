use super::{ chainer, chain_fn, ChainHalf, NonChainHalf };
use std::{ ptr, vec };
use std::cmp::Ordering;
use std::mem::MaybeUninit;
use std::ops::{ Range, RangeBounds };
use std::slice::{ self, SliceIndex };

chainer! {
	generics_decl: ['h, T]
	generics: ['h, T]
	chainer: SliceMutChain
	nonchain: &'h mut [T]
}

impl<'h, T> SliceMutChain<'h, T> {
	#[inline]
	pub fn from_ref(val: &'h mut T) -> Self {
		slice::from_mut(val).into()
	}

	#[inline]
	pub unsafe fn from_raw_parts(data: *mut T, len: usize) -> Self {
		// SAFETY: caller promises to uphold invariants of `slice::from_raw_parts_mut`
		let slice = unsafe { slice::from_raw_parts_mut(data, len) };

		slice.into()
	}

	// TODO: from_ptr_range nightly
}

impl<'h, T> SliceMutChain<'h, T> {
	chain_fn! {
		as_chunks[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&[[T; N]], &[T])
		} => {
			let len = nc.len();
			let ptr = nc.as_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr.cast::<[T; N]>();
			// SAFETY: len % N will never return greater than `len` elements
			// so the ptr sub is safe, and ptr will be valid for reads for
			// `len % N` elements, which is what `partial` is
			let partial_ptr = unsafe { ptr.add(len - partial) };

			// SAFETY: `full_ptr` casted from `T` to `[T; N]` will be valid for reads
			// for `floor(N / len)` elements of `[T; N]`, which is what `full` is
			let full_chunk = unsafe { slice::from_raw_parts(full_ptr, full) };

			// SAFETY: as established in comment for `partial_ptr`, `partial_ptr`
			// is valid for reads for `partial` elements
			let partial_chunk = unsafe { slice::from_raw_parts(partial_ptr, partial) };

			cb(full_chunk, partial_chunk);
		}
	}

	chain_fn! {
		as_chunks_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&mut [[T; N]], &mut [T])
		} => {
			let len = nc.len();
			let ptr = nc.as_mut_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr.cast::<[T; N]>();
			// SAFETY: len % N will never return greater than `len` elements
			// so the ptr sub is safe, and ptr will be valid for reads for
			// `len % N` elements, which is what `partial` is
			let partial_ptr = unsafe { ptr.add(len - partial) };

			// SAFETY: `full_ptr` casted from `T` to `[T; N]` will be valid for
			// read/write for `floor(len / N)` elements of `[T; N]`, which is what
			// `full` is. The remainder amount is `len % N` elements, which is what
			// `partial` is. We also logically "give" exclusive access to those
			// `partial` elements to `partial_ptr`, keeping `full * N` (or,
			// `floor(len / N)`) elements for this ptr, so we don't create
			// overlapping mut refs that alias each other
			let full_chunk = unsafe { slice::from_raw_parts_mut(full_ptr, full) };

			// SAFETY: See previous comment. We logically "give" exclusive access
			// to the last `partial` elements to `partial_ptr`, so `full_chunk`
			// and `partial_chunk` don't alias overlapping regions
			let partial_chunk = unsafe { slice::from_raw_parts_mut(partial_ptr, partial) };

			cb(full_chunk, partial_chunk);
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&[[T; N]])
		} => {
			let ptr = nc.as_ptr().cast::<[T; N]>();
			let chunks = nc.len() / N;

			// SAFETY: `ptr`, casted from `T` to `[T; N]`, is valid for reads for
			// `floor(len / N)` elements, and `chunks` is `len / N`
			let slice = unsafe { slice::from_raw_parts(ptr, chunks) };

			cb(slice);
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&mut [[T; N]])
		} => {
			let ptr = nc.as_mut_ptr().cast::<[T; N]>();
			let chunks = nc.len() / N;

			// SAFETY: `ptr`, casted from `T` to `[T; N]`, is valid for read/write
			// for `floor(len / N)` elements, and `chunks` is `len / N`
			let slice = unsafe { slice::from_raw_parts_mut(ptr, chunks) };
			cb(slice);
		}
	}

	chain_fn! {
		as_rchunks[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&[T], &[[T; N]])
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial).cast::<[T; N]>();

			let partial_chunk = slice::from_raw_parts(ptr, partial);
			let full_chunk = slice::from_raw_parts(full_ptr, full);

			cb(partial_chunk, full_chunk);
		}
	}

	chain_fn! {
		as_rchunks_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(&mut [T], &mut [[T; N]])
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_mut_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial).cast::<[T; N]>();

			let partial_chunk = slice::from_raw_parts_mut(ptr, partial);
			let full_chunk = slice::from_raw_parts_mut(full_ptr, full);

			cb(partial_chunk, full_chunk);
		}
	}

	chain_fn! {
		binary_search(nc, x: &T, out: &mut Result<usize, usize>) where {
			T: Ord
		} => *out = nc.binary_search(x)
	}

	chain_fn! {
		binary_search_uninit(nc, x: &T, out: &mut MaybeUninit<Result<usize, usize>>) where {
			T: Ord
		} => void out.write(nc.binary_search(x))
	}

	chain_fn! {
		binary_search_by[F](nc, f: F, out: &mut Result<usize, usize>) where {
			F: FnMut(&T) -> Ordering
		} => *out = nc.binary_search_by(f)
	}

	chain_fn! {
		binary_search_by_uninit[F](nc, f: F, out: &mut MaybeUninit<Result<usize, usize>>) where {
			F: FnMut(&T) -> Ordering
		} => void out.write(nc.binary_search_by(f))
	}

	chain_fn! {
		binary_search_by_key[B, F](nc, b: &B, f: F, out: &mut Result<usize, usize>) where {
			F: FnMut(&T) -> B,
			B: Ord
		} => *out = nc.binary_search_by_key(b, f)
	}

	chain_fn! {
		binary_search_by_key_uninit[B, F](nc, b: &B, f: F, out: &mut MaybeUninit<Result<usize, usize>>) where {
			F: FnMut(&T) -> B,
			B: Ord
		} => void out.write(nc.binary_search_by_key(b, f))
	}

	chain_fn! {
		clone_from_slice(nc, src: &[T]) where {
			T: Clone
		} => nc.clone_from_slice(src)
	}

	chain_fn! {
		contains(nc, x: &T, out: &mut bool) where {
			T: PartialEq
		} => *out = nc.contains(x)
	}

	chain_fn! {
		contains_uninit(nc, x: &T, out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => void out.write(nc.contains(x))
	}

	chain_fn! {
		copy_from_slice(nc, src: &[T]) where {
			T: Copy
		} => nc.copy_from_slice(src)
	}

	chain_fn! {
		copy_within[R](nc, src: R, dest: usize) where {
			R: RangeBounds<usize>,
			T: Copy
		} => nc.copy_within(src, dest)
	}

	chain_fn! {
		ends_with(nc, needle: &[T], out: &mut bool) where {
			T: PartialEq
		} => *out = nc.ends_with(needle)
	}

	chain_fn! {
		ends_with_uninit(nc, needle: &[T], out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => void out.write(nc.ends_with(needle))
	}

	chain_fn! {
		fill(nc, value: T) where {
			T: Clone
		} => nc.fill(value)
	}

	chain_fn! {
		fill_with[F](nc, f: F) where {
			F: FnMut() -> T
		} => nc.fill_with(f)
	}

	chain_fn! {
		first[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&T>)
		} => cb(nc.first())
	}

	chain_fn! {
		first_mut[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&mut T>)
		} => cb(nc.first_mut())
	}

	chain_fn! {
		first_chunk[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&[T; N]>)
		} => cb(nc.first_chunk())
	}

	chain_fn! {
		first_chunk_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&mut [T; N]>)
		} => cb(nc.first_chunk_mut())
	}

	chain_fn! {
		get[I, CB](nc, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			// TODO: chainer?
			CB: FnOnce(Option<&I::Output>)
		} => cb(nc.get(index))
	}

	chain_fn! {
		get_mut[I, CB](nc, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			// TODO: chainer?
			CB: FnOnce(Option<&mut I::Output>)
		} => cb(nc.get_mut(index))
	}

	chain_fn! {
		unsafe get_unchecked[I, CB](nc, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			// TODO: chainer?
			CB: FnOnce(&I::Output)
		} => {
			// SAFETY: caller promises `index` is in bounds
			let val = unsafe { nc.get_unchecked(index) };
			cb(val);
		}
	}

	chain_fn! {
		unsafe get_unchecked_mut[I, CB](nc, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			// TODO: chainer?
			CB: FnOnce(&mut I::Output)
		} => {
			// SAFETY: caller promises `index` is in bounds
			let val = unsafe { nc.get_unchecked_mut(index) };
			cb(val);
		}
	}

	chain_fn! {
		/// Writes `true` into the output if the vector contains no elements, and
		/// false otherwise
		///
		/// # Examples
		///
		/// ```
		/// # use wiwi::chainer::VecChain;
		/// # let mut before = false;
		/// # let mut after = false;
		/// let chain = VecChain::new()
		///    .is_empty(&mut before)
		///    .push(1)
		///    .is_empty(&mut after);
		///
		/// assert!(before);
		/// assert!(!after);
		/// ```
		is_empty(nc, out: &mut bool) => *out = nc.is_empty()
	}

	chain_fn! {
		/// Writes `true` into the output if the vector contains no elements, and
		/// false otherwise
		///
		//  TODO: eventually some kind of "see module documentation for info on uninit apis"
		///
		/// # Examples
		///
		/// ```
		/// # use std::mem::MaybeUninit;
		/// # use wiwi::chainer::VecChain;
		/// # let mut before = MaybeUninit::uninit();
		/// # let mut after = MaybeUninit::uninit();
		/// let chain = VecChain::new()
		///    .is_empty_uninit(&mut before)
		///    .push(1)
		///    .is_empty_uninit(&mut after);
		///
		/// let before = unsafe { before.assume_init() };
		/// let after = unsafe { after.assume_init() };
		///
		/// assert!(before);
		/// assert!(!after);
		/// ```
		is_empty_uninit(nc, out: &mut MaybeUninit<bool>) => void out.write(nc.is_empty())
	}

	chain_fn! {
		last[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&T>)
		} => cb(nc.last())
	}

	chain_fn! {
		last_mut[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&mut T>)
		} => cb(nc.last_mut())
	}

	chain_fn! {
		last_chunk[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&[T; N]>)
		} => cb(nc.last_chunk())
	}

	chain_fn! {
		last_chunk_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<&mut [T; N]>)
		} => cb(nc.last_chunk_mut())
	}

	chain_fn! {
		/// Writes the number of elements in the vector (the length) into the output
		///
		/// # Examples
		///
		/// ```
		/// # use wiwi::chainer::VecChain;
		/// # let mut len = 0;
		/// let chain = VecChain::new()
		///    .extend_from_slice(&[1, 2, 3, 4, 5])
		///    .len(&mut len);
		///
		/// assert_eq!(len, 5);
		/// ```
		len(nc, out: &mut usize) => *out = nc.len()
	}

	chain_fn! {
		/// Writes the number of elements in the vector (the length) into the output
		///
		/// # Examples
		///
		/// ```
		/// # use std::mem::MaybeUninit;
		/// # use wiwi::chainer::VecChain;
		/// # let mut len = MaybeUninit::uninit();
		/// let chain = VecChain::new()
		///    .extend_from_slice(&[1, 2, 3, 4, 5])
		///    .len_uninit(&mut len); // writes to `len`
		///
		/// let len = unsafe { len.assume_init() };
		/// assert_eq!(len, 5);
		/// ```
		len_uninit(nc, out: &mut MaybeUninit<usize>) => void out.write(nc.len())
	}

	// TODO: repeat...???

	chain_fn! {
		reverse(nc) => nc.reverse()
	}

	chain_fn! {
		rotate_left(nc, mid: usize) => nc.rotate_left(mid)
	}

	chain_fn! {
		rotate_right(nc, mid: usize) => nc.rotate_right(mid)
	}

	chain_fn! {
		sort(nc) where {
			T: Ord
		} => nc.sort()
	}

	chain_fn! {
		sort_by[F](nc, compare: F) where {
			F: FnMut(&T, &T) -> Ordering
		} => nc.sort_by(compare)
	}

	chain_fn! {
		sort_by_cached_key[K, F](nc, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => nc.sort_by_cached_key(f)
	}

	chain_fn! {
		sort_by_key[K, F](nc, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => nc.sort_by_key(f)
	}

	chain_fn! {
		sort_unstable(nc) where {
			T: Ord
		} => nc.sort_unstable()
	}

	chain_fn! {
		sort_unstable_by[F](nc, compare: F) where {
			F: FnMut(&T, &T) -> Ordering
		} => nc.sort_unstable_by(compare)
	}

	chain_fn! {
		sort_unstable_by_key[K, F](nc, f: F) where {
			F: FnMut(&T) -> K,
			K: Ord
		} => nc.sort_unstable_by_key(f)
	}

	chain_fn! {
		split_first[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&T, &[T])>)
		} => cb(nc.split_first())
	}

	chain_fn! {
		split_first_mut[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&mut T, &mut [T])>)
		} => cb(nc.split_first_mut())
	}

	chain_fn! {
		split_first_chunk[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&[T; N], &[T])>)
		} => cb(nc.split_first_chunk())
	}

	chain_fn! {
		split_first_chunk_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&mut [T; N], &mut [T])>)
		} => cb(nc.split_first_chunk_mut())
	}

	chain_fn! {
		split_last[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&T, &[T])>)
		} => cb(nc.split_last())
	}

	chain_fn! {
		split_last_mut[CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&mut T, &mut [T])>)
		} => cb(nc.split_last_mut())
	}

	chain_fn! {
		split_last_chunk[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&[T], &[T; N])>)
		} => cb(nc.split_last_chunk())
	}

	chain_fn! {
		split_last_chunk_mut[const N: usize, CB](nc, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(Option<(&mut [T], &mut [T; N])>)
		} => cb(nc.split_last_chunk_mut())
	}

	chain_fn! {
		starts_with(nc, needle: &[T], out: &mut bool) where {
			T: PartialEq
		} => *out = nc.starts_with(needle)
	}

	chain_fn! {
		starts_with_uninit(nc, needle: &[T], out: &mut MaybeUninit<bool>) where {
			T: PartialEq
		} => void out.write(nc.starts_with(needle))
	}

	chain_fn! {
		swap(nc, a: usize, b: usize) => nc.swap(a, b)
	}

	chain_fn! {
		unsafe swap_unchecked(nc, a: usize, b: usize) => {
			// TODO: replace with std impl once stabilised
			let ptr = nc.as_mut_ptr();

			// SAFETY: caller guarantees `a` is in bounds, meaning `a_ptr` will
			// be valid for read/write
			let a_ptr = unsafe { ptr.add(a) };

			// SAFETY: caller guarantees `b` is in bounds, meaning `b_ptr` will
			// be valid for read/write
			let b_ptr = unsafe { ptr.add(b) };

			// SAFETY: `a_ptr` and `b_ptr` are both valid for reads/writes
			unsafe { ptr::swap(a_ptr, b_ptr) }
		}
	}

	chain_fn! {
		swap_with_slice(nc, other: &mut [T]) => nc.swap_with_slice(other)
	}

	// TODO: try_reserve/exact

	chain_fn! {
		windows[CB](nc, size: usize, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(slice::Windows<T>)
		} => cb(nc.windows(size))
	}
}

impl<'h> SliceMutChain<'h, f32> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f32::total_cmp)
	}
}

impl<'h> SliceMutChain<'h, f64> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc) => nc.sort_unstable_by(f64::total_cmp)
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
// TODO: as_mut_ptr_range
// TODO: as_ptr_range
// TODO: as_simd
// TODO: as_simd_mut
// TODO: as_str
// TODO: assume_init
// TODO: chunk_by
// TODO: chunk_by_mut
// TODO: chunks
// TODO: chunks_exact
// TODO: chunks_exact_mut
// TODO: chunks_mut
// TODO: concat
// TODO: connect
// TODO: eq_ignore_ascii_case
// TODO: escape_ascii
// TODO: flatten
// TODO: flatten_mut
// TODO: get_many_mut
// TODO: get_many_unchecked_mut
// TODO: into_vec
// TODO: is_ascii
// TODO: is_sorted
// TODO: is_sorted_by
// TODO: is_sorted_by_key
// TODO: iter
// TODO: iter_mut
// TODO: join
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
// TODO: rsplit
// TODO: rsplit_mut
// TODO: rsplit_once
// TODO: rsplitn
// TODO: rsplitn_mut
// TODO: select_nth_unstable
// TODO: select_nth_unstable_by
// TODO: select_nth_unstable_by_key
// TODO: split
// TODO: split_at
// TODO: split_at_checked
// TODO: split_at_mut
// TODO: split_at_mut_checked
// TODO: split_at_mut_unchecked
// TODO: split_at_unchecked
// TODO: split_inclusive
// TODO: split_inclusive_mut
// TODO: split_mut
// TODO: split_once
// TODO: splitn
// TODO: splitn_mut
// TODO: strip_prefix
// TODO: strip_suffix
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
