use super::{ chainer, chain_fn, ChainHalf, NonChainHalf, SliceMutChain, SliceRefChain };
use std::{ ptr, str, vec };
use std::cmp::Ordering;
use std::mem::{ self, MaybeUninit };
use std::ops::RangeBounds;
use std::slice::{ self, SliceIndex };

chainer! {
	generics_decl: [T]
	generics: [T]
	chainer: VecChain
	nonchain: Vec<T>
}

/// Creates a [`VecChain`] containing the arguments
///
/// Usage is same as [`vec!`], except it returns [`VecChain`] instead of [`Vec`].
///
/// # Examples
///
/// ```
/// # use wiwi::chainer::{ VecChain, NonChainHalf, vec_chain };
/// let chain = vec![0u8; 32].into_chainer();
/// let chain = vec_chain![0u8; 32];
/// ```
#[macro_export]
macro_rules! vec_chain {
	[$($tt:tt)*] => { $crate::chainer::VecChain::from(vec![$($tt)*]) }
}
pub use vec_chain;

impl<T> VecChain<T> {
	/// Creates a new vector chain without allocating any capacity
	///
	/// It will not allocate until it needs to, either by pushing an element,
	/// calling the [`reserve`](Self::reserve) function to explicitly request
	/// allocation, or something else.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// // a chain thingie! yay!...
	/// let chain = VecChain::new();
	/// # let chain: VecChain<String> = chain;
	/// ```
	#[inline]
	pub fn new() -> Self {
		Vec::new().into()
	}

	#[inline]
	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		Vec::from_raw_parts(ptr, length, capacity).into()
	}

	/// Creates a new vector chain, and preallocate some memory
	///
	/// The amount of memory allocated will be _at least_ enough to hold `capacity`
	/// elements without reallocating. No allocation will happen if the provided
	/// capacity is zero, or if `T` is a ZST.
	///
	/// There is NO GUARANTEE that this function will allocate an exact amount
	/// of memory, so do not rely on this for soundness. If knowing the actual
	/// allocated capacity is important, always do so using the
	/// [`capacity`](Self::capacity) function.
	///
	/// If the element type (ie. `T`) is a ZST, the vector chainer will never
	/// allocate, and will always have a capacity of `usize::MAX` bytes.
	///
	/// # Panics
	///
	/// Panics if the new capacity exceeds `isize::MAX` _bytes_ (not elements,
	/// bytes). This is the same behaviour of [`Vec::with_capacity`].
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// # let mut len = 0;
	/// # let mut initial_capacity = 0;
	/// # let mut capacity = 0;
	/// let chain = VecChain::with_capacity(10)
	///    // chaining methods to get the len and capacity of the vec chain
	///    .len(&mut len)
	///    .capacity(&mut initial_capacity);
	///
	/// // The vector chain contains zero elements, and at least room for 10 more
	/// assert_eq!(len, 0);
	/// assert!(initial_capacity >= 10);
	///
	/// // These are all done without reallocating
	/// let chain = (0..10)
	///    .fold(chain, |chain, i| chain.push(i))
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 10);
	/// assert_eq!(capacity, initial_capacity);
	///
	/// // Now however, pushing another element can make the vector reallocate
	/// let chain = chain
	///    .push(11)
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 11);
	/// assert!(capacity >= 11);
	///
	/// # let mut capacity1 = 0;
	/// # let mut capacity2 = 0;
	/// // ZSTs never allocate and always have a capacity of `usize::MAX`
	/// let chain1 = VecChain::<()>::new()
	///    .capacity(&mut capacity1);
	/// let chain2 = VecChain::<()>::with_capacity(10)
	///    .capacity(&mut capacity2);
	///
	/// assert_eq!(capacity1, usize::MAX);
	/// assert_eq!(capacity2, usize::MAX);
	/// ```
	#[inline]
	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}
}

impl<T> VecChain<T> {
	// #[inline]
	// pub fn nonchain_as_ptr(&self) -> *const T {
	// 	self.as_nonchain().as_ptr()
	// }

	// #[inline]
	// pub fn nonchain_as_ptr_mut(&mut self) -> *mut T {
	// 	self.as_nonchain_mut().as_mut_ptr()
	// }

	// leak
}

// TODO: alloc related
// new_in
// with_capacity_in
// try_with_capacity_in
// from_raw_parts_in
// into_raw_parts_with_alloc
// allocator

impl<T> VecChain<T> {
	chain_fn! {
		/// Takes and moves all elements from another `VecChain` into `self`,
		/// leaving it empty.
		///
		/// # Examples
		///
		/// TODO
		append(nc, other: &mut Self)
			=> nc.append(other.as_nonchain_mut())
	}

	chain_fn! {
		/// Takes and moves all elements from a `Vec` into `self`,
		/// leaving it empty.
		///
		/// # Examples
		///
		/// TODO
		append_nonchain(nc, other: &mut Vec<T>)
			=> nc.append(other)
	}

	chain_fn! {
		as_chunks[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceRefChain<[T; N]>, SliceRefChain<T>)
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr.cast::<[T; N]>();
			let partial_ptr = ptr.add(len - partial);

			let full_chunk = slice::from_raw_parts(full_ptr, full);
			let partial_chunk = slice::from_raw_parts(partial_ptr, partial);

			cb(full_chunk.into(), partial_chunk.into());
		}
	}

	chain_fn! {
		as_chunks_mut[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceMutChain<[T; N]>, SliceMutChain<T>)
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_mut_ptr();

			let full = len / N;
			let partial = len % N;

			let full_ptr = ptr.cast::<[T; N]>();
			let partial_ptr = ptr.add(len - partial);

			let full_chunk = slice::from_raw_parts_mut(full_ptr, full);
			let partial_chunk = slice::from_raw_parts_mut(partial_ptr, partial);

			cb(full_chunk.into(), partial_chunk.into());
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceRefChain<[T; N]>)
		} => {
			let ptr = nc.as_ptr().cast::<[T; N]>();
			let chunks = nc.len() / N;

			let slice = slice::from_raw_parts(ptr, chunks);
			cb(slice.into());
		}
	}

	chain_fn! {
		unsafe as_chunks_unchecked_mut[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceMutChain<[T; N]>)
		} => {
			let ptr = nc.as_mut_ptr().cast::<[T; N]>();
			let chunks = nc.len() / N;

			let slice = slice::from_raw_parts_mut(ptr, chunks);
			cb(slice.into());
		}
	}

	chain_fn! {
		as_rchunks[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceRefChain<T>, SliceRefChain<[T; N]>)
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial).cast::<[T; N]>();

			let partial_chunk = slice::from_raw_parts(ptr, partial);
			let full_chunk = slice::from_raw_parts(full_ptr, full);

			cb(partial_chunk.into(), full_chunk.into());
		}
	}

	chain_fn! {
		as_rchunks_mut[const N: usize, CB](nc, cb: CB) where {
			CB: FnOnce(SliceMutChain<T>, SliceMutChain<[T; N]>)
		} => unsafe {
			let len = nc.len();
			let ptr = nc.as_mut_ptr();

			let partial = len % N;
			let full = len / N;

			let full_ptr = ptr.add(partial).cast::<[T; N]>();

			let partial_chunk = slice::from_raw_parts_mut(ptr, partial);
			let full_chunk = slice::from_raw_parts_mut(full_ptr, full);

			cb(partial_chunk.into(), full_chunk.into());
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
		capacity(nc, out: &mut usize)
			=> *out = nc.capacity()
	}

	chain_fn! {
		capacity_uninit(nc, out: &mut MaybeUninit<usize>)
			=> void out.write(nc.capacity())
	}

	chain_fn! {
		clear(nc)
			=> nc.clear()
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
		dedup(nc) where {
			T: PartialOrd
		} => nc.dedup()
	}

	chain_fn! {
		dedup_by[F](nc, mut same_bucket: F) where {
			F: FnMut(&T, &T) -> bool
		} => nc.dedup_by(move |a, b| same_bucket(a, b))
	}

	chain_fn! {
		dedup_by_mut[F](nc, same_bucket: F) where {
			F: FnMut(&mut T, &mut T) -> bool
		} => nc.dedup_by(same_bucket)
	}

	chain_fn! {
		dedup_by_key[F, K](nc, mut key: F) where {
			F: FnMut(&T) -> K,
			K: PartialEq
		} => nc.dedup_by_key(|v| key(v))
	}

	chain_fn! {
		dedup_by_key_mut[F, K](nc, mut key: F) where {
			F: FnMut(&mut T) -> K,
			K: PartialEq
		} => nc.dedup_by_key(key)
	}

	// TODO: drain

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
		extend_from_slice(nc, other: &[T]) where {
			T: Clone
		} => nc.extend_from_slice(other)
	}

	chain_fn! {
		extend_from_within[R](nc, src: R) where {
			T: Clone,
			R: RangeBounds<usize>
		} => nc.extend_from_within(src)
	}

	// TODO: extract_if

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
		} => cb(nc.get_unchecked(index))
	}

	chain_fn! {
		unsafe get_unchecked_mut[I, CB](nc, index: I, cb: CB) where {
			I: SliceIndex<[T]>,
			// TODO: chainer?
			CB: FnOnce(&mut I::Output)
		} => cb(nc.get_unchecked_mut(index))
	}

	chain_fn! {
		insert(nc, index: usize, element: T)
			=> nc.insert(index, element)
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
		is_empty(nc, out: &mut bool)
			=> *out = nc.is_empty()
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
		is_empty_uninit(nc, out: &mut MaybeUninit<bool>)
			=> void out.write(nc.is_empty())
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
		len(nc, out: &mut usize)
			=> *out = nc.len()
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
		len_uninit(nc, out: &mut MaybeUninit<usize>)
			=> void out.write(nc.len())
	}

	chain_fn! {
		pop(nc, out: &mut Option<T>)
			=> *out = nc.pop()
	}

	chain_fn! {
		pop_uninit(nc, out: &mut MaybeUninit<Option<T>>)
			=> void out.write(nc.pop())
	}

	chain_fn! {
		pop_if[F](nc, f: F, out: &mut Option<T>) where {
			F: FnOnce(&T) -> bool
		} => *out = {
			match nc.last() {
				Some(v) if f(v) => { nc.pop() }
				_ => None
			}
		}
	}

	chain_fn! {
		pop_if_mut[F](nc, f: F, out: &mut Option<T>) where {
			F: FnOnce(&mut T) -> bool
		} => *out = {
			match nc.last_mut() {
				Some(v) => if f(v) { nc.pop() } else { None }
				_ => None
			}
		}
	}

	chain_fn! {
		push(nc, value: T)
			=> nc.push(value)
	}

	chain_fn! {
		// TODO: use std when stabilised
		push_within_capacity(nc, item: T, out: &mut Result<(), T>) => *out = {
			if nc.len() == nc.capacity() {
				unsafe {
					let len = nc.len();
					nc.as_mut_ptr().add(len).write(item);
					nc.set_len(len + 1);
					Ok(())
				}
			} else {
				Err(item)
			}
		}
	}

	chain_fn! {
		remove(nc, index: usize, out: &mut T)
			=> *out = nc.remove(index)
	}

	chain_fn! {
		remove_uninit(nc, index: usize, out: &mut MaybeUninit<T>)
			=> void out.write(nc.remove(index))
	}

	chain_fn! {
		// TODO: ...this can be more efficient (done in place?)
		move repeat(nc, n: usize) where {
			T: Copy
		} => nc.repeat(n)
	}

	chain_fn! {
		reserve(nc, additional: usize)
			=> nc.reserve(additional)
	}

	chain_fn! {
		reserve_exact(nc, additional: usize)
			=> nc.reserve_exact(additional)
	}

	chain_fn! {
		resize(nc, new_len: usize, value: T) where {
			T: Clone
		} => nc.resize(new_len, value)
	}

	chain_fn! {
		resize_with[F](nc, new_len: usize, f: F) where {
			F: FnMut() -> T
		} => nc.resize_with(new_len, f)
	}

	chain_fn! {
		retain[F](nc, f: F) where {
			F: FnMut(&T) -> bool
		} => nc.retain(f)
	}

	chain_fn! {
		retain_mut[F](nc, f: F) where {
			F: FnMut(&mut T) -> bool
		} => nc.retain_mut(f)
	}

	chain_fn! {
		reverse(nc)
			=> nc.reverse()
	}

	chain_fn! {
		rotate_left(nc, mid: usize)
			=> nc.rotate_left(mid)
	}

	chain_fn! {
		rotate_right(nc, mid: usize)
			=> nc.rotate_right(mid)
	}

	chain_fn! {
		unsafe set_len(nc, new_len: usize)
			// SAFETY: caller promises that `new_len <= capacity` and
			// `..new_len` elements are initialised
			=> unsafe { nc.set_len(new_len) }
	}

	chain_fn! {
		shrink_to(nc, min_capacity: usize)
			=> nc.shrink_to(min_capacity)
	}

	chain_fn! {
		shrink_to_fit(nc)
			=> nc.shrink_to_fit()
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
		/// Calls the provided closure with the spare capacity of the vector as
		/// a slice of [`MaybeUninit<T>`]
		///
		/// # Examples
		///
		/// ```
		/// # use wiwi::chainer::VecChain;
		/// # let mut spare_len = 0;
		/// # let mut new_spare_len = 0;
		/// let chain = VecChain::with_capacity(10)
		///    .extend_from_slice(&[1, 2, 3, 4, 5])
		///    .spare_capacity_mut(|mut spare| {
		///       spare.len(&mut spare_len);
		///
		///       // VecChain allocated at least 10 elements worth of space
		///       // since we pushed 5 elements, it should have at least
		///       // 5 elements of spare capacity left
		///       assert!(spare_len >= 5);
		///    })
		///    .push(6)
		///    .push(7)
		///    .spare_capacity_mut(|mut spare| {
		///       spare.len(&mut new_spare_len);
		///
		///       // Just pushed 2 more elements, so there's 2 less spare capacity left
		///       assert_eq!(spare_len - 2, new_spare_len);
		///    });
		/// ```
		spare_capacity_mut[CB](nc, cb: CB) where {
			CB: FnOnce(SliceMutChain<MaybeUninit<T>>)
		} => cb(nc.spare_capacity_mut().into())
	}

	chain_fn! {
		splice[R, I, CB](nc, range: R, replace_with: I, cb: CB) where {
			R: RangeBounds<usize>,
			I: IntoIterator<Item = T>,
			// TODO: chainer?
			CB: FnOnce(vec::Splice<I::IntoIter>)
		} => cb(nc.splice(range, replace_with))
	}

	chain_fn! {
		split_at_spare_mut[CB](nc, cb: CB) where {
			CB: FnOnce(SliceMutChain<T>, SliceMutChain<MaybeUninit<T>>)
		} => unsafe {
			// TODO: use std impl when its stabilised
			let ptr = nc.as_mut_ptr();
			let len = nc.len();
			let cap = nc.capacity();

			let spare_ptr = ptr.add(len).cast::<MaybeUninit<T>>();
			let spare_len = cap - len;

			let init = slice::from_raw_parts_mut(ptr, len);
			let spare = slice::from_raw_parts_mut(spare_ptr, spare_len);

			cb(init.into(), spare.into());
		}
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

	// TODO: split_off

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
		swap(nc, a: usize, b: usize)
			=> nc.swap(a, b)
	}

	chain_fn! {
		unsafe swap_unchecked(nc, a: usize, b: usize) => {
			// TODO: replace with std impl once stabilised
			let ptr = nc.as_mut_ptr();
			ptr::swap(ptr.add(a), ptr.add(b));
		}
	}

	chain_fn! {
		swap_remove(nc, index: usize, out: &mut T)
			=> *out = nc.swap_remove(index)
	}

	chain_fn! {
		swap_remove_uninit(nc, index: usize, out: &mut MaybeUninit<T>)
			=> void out.write(nc.swap_remove(index))
	}

	chain_fn! {
		swap_with_slice(nc, other: &mut [T])
			=> nc.swap_with_slice(other)
	}

	chain_fn! {
		truncate(nc, len: usize)
			=> nc.truncate(len)
	}

	// TODO: try_reserve/exact

	chain_fn! {
		windows[CB](nc, size: usize, cb: CB) where {
			// TODO: chainer?
			CB: FnOnce(slice::Windows<T>)
		} => cb(nc.windows(size))
	}

	// TODO: iter/mut

	// TODO: chunks/mut
	// TODO: chunks_exact/mut
	// TODO: array_chunks
	// TODO: array_chunks_mut
	// TODO: nightly array_windows
	// TODO: rchunks/mut
	// TODO: rchunks_exact/mut
	// TODO: chunk_by/mut
	// TODO: split_at/mut
	// TODO: split_at_unchecked/mut
	// TODO: split_at_checked/mut
	// TODO: split/mut
	// TODO: split_inclusive/mut
	// TODO: rsplit/mut
	// TODO: splitn/mut
	// TODO: rsplitn/mut
	// TODO: split_once/mut
	// TODO: rsplit_once/mut
	// TODO: starts/ends_with
	// TODO: strip_prefix/suffix
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: align_to/mut
	// TODO: as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: why not non-mut of the above 2?
	// TODO: flatten/mut
	// TODO: as_str
	// TODO: as_bytes
	// TODO: to_ascii_uppercase/lowercase
	// TODO: to_vec/in
	// TODO: repeat
	// TODO: concat
	// TODO: join

	// TODO: nightly array_chunks/mut

	// TODO: rchunks/mut
	// TODO: rchunks_exact/mut
	// TODO: chunk_by/mut
	// TODO: split_at/mut
	// TODO: split_at_unchecked/mut
	// TODO: split_at_checked/mut
	// TODO: split/mut
	// TODO: split_inclusive/mut
	// TODO: rsplit/mut
	// TODO: splitn/mut
	// TODO: rsplitn/mut
	// TODO: split_once
	// TODO: rsplit_once
	// TODO: strip_prefix/suffix
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: clone_within (not in std)?
	// TODO: align_to/mut
	// TODO: nightly as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: get_many/get_many_unchecked (non mut? not in std?)
	// TODO: make_ascii_uppercase/lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii
	// TODO: trim_ascii_start/end
}

impl VecChain<f32> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc)
			=> nc.sort_unstable_by(f32::total_cmp)
	}
}

impl VecChain<f64> {
	chain_fn! {
		// TODO: call std once stabilised
		sort_floats(nc)
			=> nc.sort_unstable_by(f64::total_cmp)
	}
}

impl VecChain<u8> {
	chain_fn! {
		eq_ignore_ascii_case(nc, other: &VecChain<u8>, out: &mut bool)
			=> *out = nc.eq_ignore_ascii_case(other.as_nonchain())
	}

	chain_fn! {
		eq_ignore_ascii_case_uninit(nc, other: &VecChain<u8>, out: &mut MaybeUninit<bool>)
			=> void out.write(nc.eq_ignore_ascii_case(other.as_nonchain()))
	}

	chain_fn! {
		eq_ignore_ascii_case_nonchain(nc, other: &[u8], out: &mut bool)
			=> *out = nc.eq_ignore_ascii_case(other)
	}

	chain_fn! {
		eq_ignore_ascii_case_nonchain_uninit(nc, other: &[u8], out: &mut MaybeUninit<bool>)
			=> void out.write(nc.eq_ignore_ascii_case(other))
	}

	chain_fn! {
		escape_ascii[CB](nc, cb: CB) where {
			CB: FnOnce(slice::EscapeAscii)
		} => cb(nc.escape_ascii())
	}

	chain_fn! {
		is_ascii(nc, out: &mut bool)
			=> *out = nc.is_ascii()
	}

	chain_fn! {
		is_ascii_uninit(nc, out: &mut MaybeUninit<bool>)
			=> void out.write(nc.is_ascii())
	}

	chain_fn! {
		make_ascii_lowercase(nc)
			=> nc.make_ascii_lowercase()
	}

	chain_fn! {
		make_ascii_uppercase(nc)
			=> nc.make_ascii_uppercase()
	}

	chain_fn! {
		/// Trims off the leading and trailing ASCII whitespace in place
		trim_ascii(nc) => unsafe {
			let trimmed_start = _tmp_trim_ascii_start_amount(nc);
			if trimmed_start == nc.len() {
				// the fn ate everything, everything is whitespace
				nc.set_len(0);
				// TODO: cannot early return. But I wanna make it possible somehow in chain_fn!
			} else {
				let trimmed_end = _tmp_trim_ascii_end_amount(nc);
				let total_trimmed = trimmed_start + trimmed_end;

				let ptr = nc.as_mut_ptr();
				let new_len = nc.len() - total_trimmed;

				ptr::copy(nc.as_ptr().add(trimmed_start), ptr, new_len);
				nc.set_len(new_len);
			}
		}
	}

	chain_fn! {
		trim_ascii_end(nc) => unsafe {
			nc.set_len(nc.len() - _tmp_trim_ascii_end_amount(nc))
		}
	}

	chain_fn! {
		trim_ascii_start(nc) => unsafe {
			let trimmed = _tmp_trim_ascii_start_amount(nc);

			let ptr = nc.as_mut_ptr();
			let new_len = nc.len() - trimmed;

			ptr::copy(nc.as_ptr().add(trimmed), ptr, new_len);
			nc.set_len(new_len);
		}
	}

	chain_fn! {
		utf8_chunks[CB](nc, cb: CB) where {
			CB: FnOnce(str::Utf8Chunks)
		} => cb(nc.utf8_chunks())
	}

	// TODO: as_ascii/unchecked nightly
}

impl<T, const N: usize> VecChain<[T; N]> {
	#[inline]
	pub fn flatten(mut self) -> VecChain<T> {
		let (len, cap) = if mem::size_of::<T>() == 0 {
			let len = self.as_nonchain().len()
				.checked_mul(N)
				.expect("vec len overflow");
			(len, usize::MAX)
		} else {
			// TODO: use unchecked mul when rust 1.79
			(
				self.as_nonchain().len() * N,
				self.as_nonchain().capacity() * N
			)
		};

		let ptr = self.as_nonchain_mut().as_mut_ptr().cast::<T>();
		mem::forget(self);

		unsafe { Vec::from_raw_parts(ptr, len, cap) }.into()
	}
}

impl<T> AsMut<[T]> for VecChain<T> {
	#[inline]
	fn as_mut(&mut self) -> &mut [T] {
		self.as_nonchain_mut()
	}
}

impl<T> AsRef<[T]> for VecChain<T> {
	#[inline]
	fn as_ref(&self) -> &[T] {
		self.as_nonchain()
	}
}

// TODO: allocator
// TODO: drain
// TODO: extract_if
// TODO: from_raw_parts
// TODO: from_raw_parts_in
// TODO: into_boxed_slice
// TODO: into_raw_parts
// TODO: into_raw_parts_with_alloc
// TODO: leak
// TODO: new_in
// TODO: split_off
// TODO: try_reserve
// TODO: try_reserve_exact
// TODO: try_with_capacity
// TODO: try_with_capacity_in
// TODO: with_capacity
// TODO: with_capacity_in
// TODO: Methods from Deref<Target=[T]>
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
// TODO: chunk_by
// TODO: chunk_by_mut
// TODO: chunks
// TODO: chunks_exact
// TODO: chunks_exact_mut
// TODO: chunks_mut
// TODO: concat
// TODO: connect
// TODO: escape_ascii
// TODO: flatten_mut
// TODO: get_many_mut
// TODO: get_many_unchecked_mut
// TODO: is_sorted
// TODO: is_sorted_by
// TODO: is_sorted_by_key
// TODO: iter
// TODO: iter_mut
// TODO: join
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
// TODO: utf8_chunks

// TODO: trait impls
// TODO: Borrow<[T]>
// TODO: BorrowMut<[T]>
// TODO: Eq
// TODO: Extend<&'a T>
// TODO: Extend<T>
// TODO: From<&'a Vec<T>>
// TODO: From<&[T; N]>
// TODO: From<&[T]>
// TODO: From<&mut [T; N]>
// TODO: From<&mut [T]>
// TODO: From<&str>
// TODO: From<BinaryHeap<T, A>>
// TODO: From<Box<[T], A>>
// TODO: From<CString>
// TODO: From<Cow<'a, [T]>>
// TODO: From<String>
// TODO: From<Vec<NonZero<u8>>>
// TODO: From<Vec<T, A>>
// TODO: From<Vec<T, A>>
// TODO: From<Vec<T, A>>
// TODO: From<Vec<T, A>>
// TODO: From<Vec<T, A>>
// TODO: From<Vec<T>>
// TODO: From<VecDeque<T, A>>
// TODO: From<[T; N]>
// TODO: FromIterator<T>
// TODO: Hash
// TODO: Index<I>
// TODO: IndexMut<I>
// TODO: IntoIterator
// TODO: IntoIterator
// TODO: IntoIterator
// TODO: Ord
// TODO: PartialEq<&[U; N]>
// TODO: PartialEq<&[U]>
// TODO: PartialEq<&mut [U]>
// TODO: PartialEq<Vec<U, A2>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<Vec<U, A>>
// TODO: PartialEq<[U; N]>
// TODO: PartialEq<[U]>
// TODO: PartialOrd<Vec<T, A2>>
// TODO: TryFrom<Vec<T, A>>
// TODO: TryFrom<Vec<T>>
// TODO: Write

#[inline]
fn _tmp_trim_ascii_start_amount(mut bytes: &[u8]) -> usize {
	let mut count = 0;
	while let [first, rest @ ..] = bytes {
		if first.is_ascii_whitespace() {
			bytes = rest;
			count += 1;
		} else {
			break;
		}
	}
	count
}

#[inline]
fn _tmp_trim_ascii_end_amount(mut bytes: &[u8]) -> usize {
	let mut count = 0;
	while let [rest @ .., last] = bytes {
		if last.is_ascii_whitespace() {
			bytes = rest;
			count += 1;
		} else {
			break;
		}
	}
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn conversion() {
		let slice = &[1u8, 2, 3, 4, 5] as &[_];
		let mut chain = VecChain::new()
			.extend_from_slice(slice);

		assert_eq!(slice, chain.as_nonchain());
		assert_eq!(slice, chain.as_nonchain_mut());
	}

	#[test]
	fn creation() {
		let mut new_len = 0;
		let mut new_capacity = 0;
		let mut with_capacity_len = 0;
		let mut with_capacity_capacity = 0;
		let mut zst_len = 0;
		let mut zst_capacity = 0;

		let _ = VecChain::<u8>::new()
			.len(&mut new_len)
			.capacity(&mut new_capacity);

		let _ = VecChain::<u8>::with_capacity(30)
			.len(&mut with_capacity_len)
			.capacity(&mut with_capacity_capacity);

		let _ = VecChain::<()>::with_capacity(30)
			.len(&mut zst_len)
			.capacity(&mut zst_capacity);

		assert_eq!(new_len, 0);
		assert_eq!(new_capacity, 0);
		assert_eq!(with_capacity_len, 0);
		assert_eq!(with_capacity_capacity, 30);
		assert_eq!(zst_len, 0);
		assert_eq!(zst_capacity, usize::MAX);
	}

	#[test]
	fn len_and_capacity() {
		let mut len1 = 0;
		let mut cap1 = 0;
		let mut len2 = 0;
		let mut cap2 = 0;
		let mut is_empty = true;
		let mut is_empty_new = false;
		let mut is_empty_with_cap = false;

		let _ = VecChain::with_capacity(8)
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.len(&mut len1)
			.capacity(&mut cap1)
			.push(9)
			.push(10)
			.len(&mut len2)
			.capacity(&mut cap2)
			.is_empty(&mut is_empty);
		let _ = VecChain::<u8>::new()
			.is_empty(&mut is_empty_new);
		let _ = VecChain::<u8>::with_capacity(8)
			.is_empty(&mut is_empty_with_cap);

		assert_eq!(len1, 8);
		assert!(cap1 >= 8);

		assert_eq!(len2, 10);
		assert!(cap2 >= 10);

		assert!(!is_empty);
		assert!(is_empty_new);
		assert!(is_empty_with_cap);
	}

	#[test]
	fn reverse() {
		let chain = VecChain::new()
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.reverse();
		assert_eq!(chain.as_nonchain(), &[8, 7, 6, 5, 4, 3, 2, 1]);
	}

	#[test]
	fn split_at_spare_mut() {
		let mut uninit_len = 0;
		let chain = VecChain::new()
			.extend_from_slice(&[1usize, 2, 3, 4, 5, 6, 7, 8])
			.reserve(8)
			.split_at_spare_mut(|mut slice, mut uninit| {
				uninit_len = uninit.as_nonchain().len();

				assert_eq!(slice.as_nonchain(), &[1, 2, 3, 4, 5, 6, 7, 8]);
				assert!(uninit.as_nonchain().len() >= 8);

				uninit.as_nonchain_mut()
					.iter_mut()
					.enumerate()
					.take(4)
					.for_each(|(i, slot)| {
						slot.write(i);
					});
			});

		unsafe {
			let mut len = 0;
			let _ = chain
				.len(&mut len)
				.set_len(len + 4)
				.split_at_spare_mut(|mut slice, mut uninit| {
					assert_eq!(slice.as_nonchain(), &[1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3]);
					assert_eq!(uninit_len - 4, uninit.as_nonchain().len());
				});
		}
	}

	#[test]
	fn swap_unchecked() {
		unsafe {
			let chain = VecChain::new()
				.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
				.swap_unchecked(4, 6)
				.swap_unchecked(0, 3)
				.swap_unchecked(1, 6)
				.swap_unchecked(6, 7)
				.swap_unchecked(2, 6);
			assert_eq!(chain.as_nonchain(), &[4, 5, 8, 1, 7, 6, 3, 2]);
		}
	}

	#[test]
	fn trim_ascii() {
		// (original, trimmed_start, trimmed_end, trimmed_both)
		let strs = [
			// generic random
			(" uwu ", "uwu ", " uwu", "uwu"),
			// all spaces
			("     ", "", "", ""),
			// different types of whitespace (also space in the middle)
			(
				"\t\n\x0c\r you're cute \x0c\n\r \t",
				"you're cute \x0c\n\r \t",
				"\t\n\x0c\r you're cute",
				"you're cute",
			),
			// all whitespace
			("\t\n\x0c\r   \t\t\n\n\t     ", "", "", "")
		];

		for (original, trimmed_start, trimmed_end, trimmed_both) in strs {
			let original = original.as_bytes().to_vec().into_chainer();

			assert_eq!(&**original.clone().trim_ascii_start().as_nonchain(), trimmed_start.as_bytes());
			assert_eq!(&**original.clone().trim_ascii_end().as_nonchain(), trimmed_end.as_bytes());
			assert_eq!(&**original.trim_ascii().as_nonchain(), trimmed_both.as_bytes());
		}
	}

	#[test]
	fn as_chunks() {
		const N: usize = 5;

		let slice = b"1234";

		fn check<'h>(
			expected_chunks: &[&[u8; N]],
			expected_remainder: &'h [u8]
		) -> impl FnOnce(SliceRefChain<[u8; N]>, SliceRefChain<u8>) + 'h {
			let expected_chunks = expected_chunks
				.into_iter()
				.map(|item| **item)
				.collect::<Vec<_>>();

			move |chunks, rem| {
				assert_eq!(
					expected_chunks.len(),
					chunks.as_nonchain().len(),
					"wrong num of chunks"
				);
				assert_eq!(
					expected_remainder.len(),
					rem.as_nonchain().len(),
					"wrong num of elements in remainder"
				);

				assert_eq!(expected_chunks, *chunks.as_nonchain());
				assert_eq!(expected_remainder, *rem.as_nonchain());
			}
		}

		let _ = VecChain::with_capacity(20)
			.extend_from_slice(slice)
			.as_chunks(check(&[], b"1234"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341"], b"234"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412"], b"34"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412", b"34123"], b"4"))

			.extend_from_slice(slice)
			.as_chunks(check(&[b"12341", b"23412", b"34123", b"41234"], b""));
	}
}
