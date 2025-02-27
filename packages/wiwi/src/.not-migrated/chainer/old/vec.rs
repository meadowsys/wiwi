use crate::iter::{ IterAdapter, IntoIter, IntoStdIterator };
use crate::to_maybeuninit::ToMaybeUninit as _;
use std::{ ptr, vec };
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem::{ self, ManuallyDrop, MaybeUninit };
use std::ops::{ Range, RangeBounds };
use std::slice::{ self, SliceIndex };
use super::{ SliceBoxChain, SliceRefChain, SliceMutChain };

// /// Vec type that provides a chaining API.
// ///
// /// It contains similar methods as [`Vec`], but in some cases, the API differs
// /// to accomodate the chaining API.
// TODO: allocator param
#[repr(transparent)]
#[derive(Clone, Debug, Default)]
pub struct VecChain<T> {
	inner: Vec<T>
}

impl<T> VecChain<T> {
	/// Creates a new vector chain without allocating any capacity.
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
	pub fn new() -> Self {
		Vec::new().into()
	}

	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		Vec::from_raw_parts(ptr, length, capacity).into()
	}

	/// Creates a new vector, and preallocate some memory.
	///
	/// The amount of memory allocated will be _at least_ enough to hold `capacity`
	/// elements without reallocating. No allocation will happen if the provided
	/// capacity is zero.
	///
	/// There is NO GUARANTEE that this function will allocate an exact amount
	/// of memory. If knowing the actual allocated capacity is important, always
	/// do so using the [`capacity`](Self::capacity) function.
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
	pub fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into()
	}

	// TODO: try_with_capacity
}

// TODO: for alloc param
impl<T> VecChain<T> {
	// TODO: new_in
	// TODO: with_capacity_in
	// TODO: try_with_capacity_in
	// TODO: from_raw_parts_in

	// TODO: into_raw_parts_with_alloc
	// TODO: fn allocator
}

impl<T> VecChain<T> {
	pub fn nonchain_boxed_slice(self) -> Box<[T]> {
		self.inner.into_boxed_slice()
	}

	pub fn nonchain_boxed_slice_chainer(self) -> SliceBoxChain<T> {
		self.nonchain_boxed_slice().into()
	}

	/// Unwraps and retrieves the underlying [`Vec`] out.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// # let vec_chain = VecChain::<String>::new();
	/// let regular_vec = vec_chain.nonchain_inner();
	/// ```
	pub fn nonchain_inner(self) -> Vec<T> {
		self.inner
	}

	pub fn nonchain_raw_parts(self) -> (*mut T, usize, usize) {
		// TODO: use vec impl when stabilised
		let mut me = ManuallyDrop::new(self.inner);
		(me.as_mut_ptr(), me.len(), me.capacity())
	}

	pub fn nonchain_ptr(&self) -> *const T {
		self.inner.as_ptr()
	}

	pub fn nonchain_ptr_mut(&mut self) -> *mut T {
		self.inner.as_mut_ptr()
	}

	pub fn nonchain_ptr_range(&self) -> Range<*const T> {
		self.inner.as_ptr_range()
	}

	pub fn nonchain_ptr_range_mut(&mut self) -> Range<*mut T> {
		self.inner.as_mut_ptr_range()
	}

	pub fn nonchain_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn nonchain_slice_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}

	/// Borrow this vector chain immutably as a [`SliceRefChain`].
	///
	/// Note: this does not consume `self`, but only immutably borrow from it. So,
	/// you will need to keep `self` in somewhere owned.
	pub fn nonchain_slice_chainer_ref(&self) -> SliceRefChain<T> {
		(*self.inner).into()
	}

	/// Borrow this vector chain mutably as a [`SliceMutChain`].
	///
	/// Note: this does not consume `self`, but only mutably borrow from it. So,
	/// you will need to keep `self` in somewhere owned.
	pub fn nonchain_slice_chainer_mut(&mut self) -> SliceMutChain<T> {
		(&mut *self.inner).into()
	}

	pub fn nonchain_vec(&self) -> &Vec<T> {
		&self.inner
	}

	pub fn nonchain_vec_mut(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}
}

impl<T> VecChain<T> {
	pub fn append(mut self, other: &mut Vec<T>) -> Self {
		self.inner.append(other);
		self
	}

	pub fn append_vec_chain(mut self, other: &mut Self) -> Self {
		self.append(&mut other.inner)
	}

	pub fn as_chunks<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(&[[T; N]], &[T])
		// CB: FnOnce(SliceRefChain<>, &[T])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			let len = self.inner.len();
			let remainder = len % N;
			let ptr = self.nonchain_ptr().add(len - remainder);
			let partial_chunk = slice::from_raw_parts(ptr, remainder);

			// SAFETY: our impl of this unchecked fn just uses int division
			// (round down), so this is sound. However we're not promising this
			// in the public API
			self.as_chunks_unchecked(|chunks| cb(chunks, partial_chunk))
		}
	}

	pub fn as_chunks_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(&mut [[T; N]], &mut [T])
	{
		// TODO: call std equivalent when its stabilised

		unsafe {
			let len = self.inner.len();
			let remainder = len % N;
			let ptr = self.nonchain_ptr_mut().add(len - remainder);
			let partial_chunk = slice::from_raw_parts_mut(ptr, remainder);

			// SAFETY: our impl of this unchecked fn just uses int division
			// (round down), so this is sound. However we're not promising this
			// in the public API
			self.as_chunks_unchecked_mut(|chunks| cb(chunks, partial_chunk))
		}
	}

	pub unsafe fn as_chunks_unchecked<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(&[[T; N]])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			// SAFETY: the non-unsafe versions of this function is relying on this
			// int (round down) division, rather than exact division
			// (like std::intrinsics::exact_div). Do not change this without
			// changing those uses also
			let chunks = self.inner.len() / N;

			let ptr = self.nonchain_ptr() as *const [T; N];
			let slice = slice::from_raw_parts(ptr, chunks);
			cb(slice);
		}

		self
	}

	pub unsafe fn as_chunks_unchecked_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(&mut [[T; N]])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			// SAFETY: the non-unsafe versions of this function is relying on this
			// int (round down) division, rather than exact division
			// (like std::intrinsics::exact_div). Do not change this without
			// changing those uses also
			let chunks = self.inner.len() / N;

			let ptr = self.nonchain_ptr_mut() as *mut [T; N];
			let slice = slice::from_raw_parts_mut(ptr, chunks);
			cb(slice);
		}

		self
	}

	pub fn as_rchunks<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(&[T], &[[T; N]])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			let len = self.inner.len();
			let remainder = len % N;
			let full_chunks = len / N;

			let partial_ptr = self.nonchain_ptr();
			let full_ptr = partial_ptr.add(remainder) as *const [T; N];

			let partial = slice::from_raw_parts(partial_ptr, remainder);
			let full = slice::from_raw_parts(full_ptr, full_chunks);

			cb(partial, full);
		}

		self
	}

	pub fn as_rchunks_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(&mut [T], &mut [[T; N]])
	{
		// TODO: call std equivalent after its stabilised

		unsafe {
			let len = self.inner.len();
			let remainder = len % N;
			let full_chunks = len / N;

			let partial_ptr = self.nonchain_ptr_mut();
			let full_ptr = partial_ptr.add(remainder) as *mut [T; N];

			let partial = slice::from_raw_parts_mut(partial_ptr, remainder);
			let full = slice::from_raw_parts_mut(full_ptr, full_chunks);

			cb(partial, full);
		}

		self
	}

	pub fn binary_search(self, x: &T, out: &mut Result<usize, usize>) -> Self
	where
		T: Ord
	{
		self.binary_search_uninit(x, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn binary_search_uninit(self, x: &T, out: &mut MaybeUninit<Result<usize, usize>>) -> Self
	where
		T: Ord
	{
		out.write(self.inner.binary_search(x));
		self
	}

	pub fn binary_search_by<F>(self, f: F, out: &mut Result<usize, usize>) -> Self
	where
		F: FnMut(&T) -> Ordering
	{
		self.binary_search_by_uninit(f, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn binary_search_by_uninit<F>(self, f: F, out: &mut MaybeUninit<Result<usize, usize>>) -> Self
	where
		F: FnMut(&T) -> Ordering
	{
		out.write(self.inner.binary_search_by(f));
		self
	}

	pub fn binary_search_by_key<B, F>(self, b: &B, f: F, out: &mut Result<usize, usize>) -> Self
	where
		F: FnMut(&T) -> B,
		B: Ord
	{
		self.binary_search_by_key_uninit(b, f, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn binary_search_by_key_uninit<B, F>(self, b: &B, f: F, out: &mut MaybeUninit<Result<usize, usize>>) -> Self
	where
		F: FnMut(&T) -> B,
		B: Ord
	{
		out.write(self.inner.binary_search_by_key(b, f));
		self
	}

	pub fn capacity(self, out: &mut usize) -> Self {
		self.capacity_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn capacity_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.capacity());
		self
	}

	pub fn clear(mut self) -> Self {
		self.inner.clear();
		self
	}

	pub fn clone_from_slice(mut self, src: &[T]) -> Self
	where
		T: Clone
	{
		self.inner.clone_from_slice(src);
		self
	}

	pub fn contains(self, x: &T, out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.contains_uninit(x, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn contains_uninit(self, x: &T, out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.contains(x));
		self
	}

	pub fn copy_from_slice(mut self, src: &[T]) -> Self
	where
		T: Copy
	{
		self.inner.copy_from_slice(src);
		self
	}

	pub fn copy_within<R>(mut self, src: R, dest: usize) -> Self
	where
		R: RangeBounds<usize>,
		T: Copy
	{
		self.inner.copy_within(src, dest);
		self
	}

	pub fn dedup(mut self) -> Self
	where
		T: PartialOrd
	{
		self.inner.dedup();
		self
	}

	pub fn dedup_by<F>(mut self, mut same_bucket: F) -> Self
	where
		F: FnMut(&T, &T) -> bool
	{
		// let rust coerce &mut T to &T
		self.inner.dedup_by(move |a, b| same_bucket(a, b));
		self
	}

	pub fn dedup_by_mut<F>(mut self, same_bucket: F) -> Self
	where
		F: FnMut(&mut T, &mut T) -> bool
	{
		self.inner.dedup_by(same_bucket);
		self
	}

	pub fn dedup_by_key<F, K>(mut self, mut key: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: PartialEq
	{
		// let rust coerce &mut T to &T
		self.inner.dedup_by_key(|v| key(v));
		self
	}

	pub fn dedup_by_key_mut<F, K>(mut self, key: F) -> Self
	where
		F: FnMut(&mut T) -> K,
		K: PartialEq
	{
		self.inner.dedup_by_key(key);
		self
	}

	pub fn drain<R, CB>(mut self, range: R, cb: CB) -> Self
	where
		R: RangeBounds<usize>,
		CB: FnOnce(vec::Drain<T>)
	{
		cb(self.inner.drain(range));
		self
	}

	pub fn ends_with(self, needle: &[T], out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.ends_with_uninit(needle, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn ends_with_uninit(self, needle: &[T], out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.ends_with(needle));
		self
	}

	pub fn extend_from_slice(mut self, other: &[T]) -> Self
	where
		T: Clone
	{
		self.inner.extend_from_slice(other);
		self
	}

	pub fn extend_from_within<R>(mut self, src: R) -> Self
	where
		T: Clone,
		R: RangeBounds<usize>
	{
		self.inner.extend_from_within(src);
		self
	}

	// TODO: extract_if
	// pub fn extract_if(mut self) {
	// 	self.inner.extract_if(filter)
	// }

	pub fn fill(mut self, value: T) -> Self
	where
		T: Clone
	{
		self.inner.fill(value);
		self
	}

	pub fn fill_with<F>(mut self, f: F) -> Self
	where
		F: FnMut() -> T
	{
		self.inner.fill_with(f);
		self
	}

	pub fn first<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.first());
		self
	}

	pub fn first_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut T>)
	{
		cb(self.inner.first_mut());
		self
	}

	pub fn first_chunk<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&[T; N]>)
	{
		cb(self.inner.first_chunk());
		self
	}

	pub fn first_chunk_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut [T; N]>)
	{
		cb(self.inner.first_chunk_mut());
		self
	}

	pub fn get<I, CB>(self, index: I, cb: CB) -> Self
	where
		I: SliceIndex<[T]>,
		CB: FnOnce(Option<&I::Output>)
	{
		cb(self.inner.get(index));
		self
	}

	pub fn get_mut<I, CB>(mut self, index: I, cb: CB) -> Self
	where
		I: SliceIndex<[T]>,
		CB: FnOnce(Option<&mut I::Output>)
	{
		cb(self.inner.get_mut(index));
		self
	}

	pub unsafe fn get_unchecked<I, CB>(self, index: I, cb: CB) -> Self
	where
		I: SliceIndex<[T]>,
		CB: FnOnce(&I::Output)
	{
		cb(self.inner.get_unchecked(index));
		self
	}

	pub unsafe fn get_unchecked_mut<I, CB>(mut self, index: I, cb: CB) -> Self
	where
		I: SliceIndex<[T]>,
		CB: FnOnce(&mut I::Output)
	{
		cb(self.inner.get_unchecked_mut(index));
		self
	}

	pub fn insert(mut self, index: usize, element: T) -> Self {
		self.inner.insert(index, element);
		self
	}

	/// Writes `true` into the output if the vector contains no elements, and
	/// false otherwise
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// // output variables...
	/// let mut before = false;
	/// let mut after = false;
	///
	/// let chain = VecChain::new()
	///    // chains are evaluated eagerly
	///    .is_empty(&mut before)
	///    .push(1)
	///    .is_empty(&mut after);
	///
	/// assert!(before);
	/// assert!(!after);
	/// ```
	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	/// Writes `true` into the output if the vector contains no elements, and
	/// false otherwise
	///
	/// # Examples
	///
	/// ```
	/// # use std::mem::MaybeUninit;
	/// # use wiwi::chainer::VecChain;
	/// // output variables...
	/// let mut before = MaybeUninit::uninit();
	/// let mut after = MaybeUninit::uninit();
	///
	/// let chain = VecChain::new()
	///    // chains are evaluated eagerly
	///    .is_empty_uninit(&mut before)
	///    .push(1)
	///    .is_empty_uninit(&mut after);
	///
	/// // this is safe!
	/// let before = unsafe { before.assume_init() };
	/// let after = unsafe { after.assume_init() };
	///
	/// assert!(before);
	/// assert!(!after);
	/// ```
	pub fn is_empty_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn last<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.last());
		self
	}

	pub fn last_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut T>)
	{
		cb(self.inner.last_mut());
		self
	}

	pub fn last_chunk<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&[T; N]>)
	{
		cb(self.inner.last_chunk());
		self
	}

	pub fn last_chunk_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&mut [T; N]>)
	{
		cb(self.inner.last_chunk_mut());
		self
	}

	/// Consumes `self` and leaks it, returning a mutable reference to the content.
	/// You may choose any lifetime `'h`, as long as `T` outlives `'h`. It can even
	/// be `'static`.
	///
	/// The vector is shrunk as much as it can be (ie. [`shrink_to_fit`] is called),
	/// but it might still have some excess capacity, in the same way [`with_capacity`]
	/// and [`reserve`] can allocate more than is requested.
	///
	/// This function is mainly useful for something that needs to be kept for
	/// the remainder of a program's life. As this function's name implies, if
	/// you drop the returned reference, it will leak memory. Absence of memory
	/// leaks is not part of Rust's memory model guarantees (for some reason...),
	/// so this is safe.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// let mut static_ref = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3])
	///    .leak::<'static>();
	///
	/// static_ref.nonchain_slice_mut()[1] = 20;
	/// assert_eq!(static_ref.nonchain_slice(), [1, 20, 3]);
	/// ```
	///
	/// [`shrink_to_fit`]: Self::shrink_to_fit
	/// [`with_capacity`]: Self::with_capacity
	/// [`reserve`]: Self::reserve
	pub fn leak<'h>(self) -> SliceMutChain<'h, T> {
		self.shrink_to_fit().nonchain_inner().leak().into()
	}

	/// Writes the number of elements (also known as the length) in the vector
	/// into `out`.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chainer::VecChain;
	/// let mut len = 0;
	///
	/// let chain = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3, 4, 5])
	///    .len(&mut len);
	///
	/// assert_eq!(len, 5);
	/// ```
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	/// Writes the number of elements (also known as the length) in the vector
	/// into `out`.
	///
	/// This function will always write to the output, so it is safe to call
	/// [`assume_init`](MaybeUninit::assume_init) after invoking this function.
	///
	/// # Examples
	///
	/// ```
	/// # use std::mem::MaybeUninit;
	/// # use wiwi::chainer::VecChain;
	/// let mut len = MaybeUninit::uninit();
	///
	/// let chain = VecChain::new()
	///    .extend_from_slice(&[1, 2, 3, 4, 5])
	///    .len_uninit(&mut len); // writes to `len`
	///
	/// // this is safe!
	/// let len = unsafe { len.assume_init() };
	/// assert_eq!(len, 5);
	/// ```
	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	pub fn pop(mut self, out: &mut Option<T>) -> Self {
		self.pop_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn pop_uninit(mut self, out: &mut MaybeUninit<Option<T>>) -> Self {
		out.write(self.inner.pop());
		self
	}

	// TODO: pop_if

	pub fn push(mut self, value: T) -> Self {
		self.inner.push(value);
		self
	}

	// TODO: push_within_capacity

	pub fn remove(mut self, index: usize, out: &mut T) -> Self {
		self.remove_uninit(index, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn remove_uninit(mut self, index: usize, out: &mut MaybeUninit<T>) -> Self {
		out.write(self.inner.remove(index));
		self
	}

	pub fn repeat(mut self, n: usize) -> Self
	where
		T: Copy
	{
		// TODO: ...this can be more efficient (done in place?)
		self = self.inner.repeat(n).into();
		self
	}

	pub fn reserve(mut self, additional: usize) -> Self {
		self.inner.reserve(additional);
		self
	}

	pub fn reserve_exact(mut self, additional: usize) -> Self {
		self.inner.reserve_exact(additional);
		self
	}

	pub fn resize(mut self, new_len: usize, value: T) -> Self
	where
		T: Clone
	{
		self.inner.resize(new_len, value);
		self
	}

	pub fn resize_with<F>(mut self, new_len: usize, f: F) -> Self
	where
		F: FnMut() -> T
	{
		self.inner.resize_with(new_len, f);
		self
	}

	pub fn retain<F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> bool
	{
		self.inner.retain(f);
		self
	}

	pub fn retain_mut<F>(mut self, f: F) -> Self
	where
		F: FnMut(&mut T) -> bool
	{
		self.inner.retain_mut(f);
		self
	}

	pub fn reverse(mut self) -> Self {
		self.inner.reverse();
		self
	}

	pub unsafe fn set_len(mut self, new_len: usize) -> Self {
		self.inner.set_len(new_len);
		self
	}

	pub fn shrink_to_fit(mut self) -> Self {
		self.inner.shrink_to_fit();
		self
	}

	pub fn shrink_to(mut self, min_capacity: usize) -> Self {
		self.inner.shrink_to(min_capacity);
		self
	}

	pub fn sort(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort();
		self
	}

	pub fn sort_by<F>(mut self, compare: F) -> Self
	where
		F: FnMut(&T, &T) -> Ordering
	{
		self.inner.sort_by(compare);
		self
	}

	pub fn sort_by_cached_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_cached_key(f);
		self
	}

	pub fn sort_by_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_by_key(f);
		self
	}

	pub fn sort_unstable(mut self) -> Self
	where
		T: Ord
	{
		self.inner.sort_unstable();
		self
	}

	pub fn sort_unstable_by<F>(mut self, compare: F) -> Self
	where
		F: FnMut(&T, &T) -> Ordering
	{
		self.inner.sort_unstable_by(compare);
		self
	}

	pub fn sort_unstable_by_key<K, F>(mut self, f: F) -> Self
	where
		F: FnMut(&T) -> K,
		K: Ord
	{
		self.inner.sort_unstable_by_key(f);
		self
	}

	pub fn splice<R, I, CB>(mut self, range: R, replace_with: I, cb: CB) -> Self
	where
		R: RangeBounds<usize>,
		I: IntoIter<Item = T>,
		CB: FnOnce(vec::Splice<IterAdapter<I::Iter>>)
	{
		cb(self.inner.splice(range, replace_with.convert_wiwi_into_std_iterator()));
		self
	}

	pub fn split_at_spare_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(SliceMutChain<T>, SliceMutChain<MaybeUninit<T>>)
	{
		// TODO: call Vec impl when it is stabilised
		unsafe {
			let ptr = self.nonchain_ptr_mut();
			let len = self.inner.len();
			let cap = self.inner.capacity();

			let spare_ptr = ptr.add(len) as *mut MaybeUninit<T>;
			let spare_len = cap - len;

			let init = slice::from_raw_parts_mut(ptr, len).into();
			let spare = slice::from_raw_parts_mut(spare_ptr, spare_len).into();

			cb(init, spare);
		}

		self
	}

	pub fn split_first<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&T, SliceRefChain<T>)>)
	{
		cb(self.inner.split_first().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_first_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&mut T, SliceMutChain<T>)>)
	{
		cb(self.inner.split_first_mut().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_first_chunk<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&[T; N], &[T])>)
	{
		cb(self.inner.split_first_chunk());
		self
	}
	pub fn split_first_chunk_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&mut [T; N], &mut [T])>)
	{
		cb(self.inner.split_first_chunk_mut());
		self
	}

	pub fn split_last<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&T, SliceRefChain<T>)>)
	{
		cb(self.inner.split_last().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_last_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&mut T, SliceMutChain<T>)>)
	{
		cb(self.inner.split_last_mut().map(|(a, b)| (a, b.into())));
		self
	}

	pub fn split_last_chunk<const N: usize, CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&[T], &[T; N])>)
	{
		cb(self.inner.split_last_chunk());
		self
	}

	pub fn split_last_chunk_mut<const N: usize, CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(Option<(&mut [T], &mut [T; N])>)
	{
		cb(self.inner.split_last_chunk_mut());
		self
	}

	// TODO: split_off

	pub fn starts_with(self, needle: &[T], out: &mut bool) -> Self
	where
		T: PartialEq
	{
		self.starts_with_uninit(needle, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn starts_with_uninit(self, needle: &[T], out: &mut MaybeUninit<bool>) -> Self
	where
		T: PartialEq
	{
		out.write(self.inner.starts_with(needle));
		self
	}

	pub fn swap(mut self, mut a: usize, b: usize) -> Self {
		self.inner.swap(a, b);
		self
	}

	pub unsafe fn swap_unchecked(mut self, a: usize, b: usize) -> Self {
		// TODO: replace with Vec::swap_unchecked call when it's stabilised?
		let ptr = self.nonchain_ptr_mut();
		ptr::swap(ptr.add(a), ptr.add(b));
		self
	}

	pub fn swap_remove(mut self, index: usize, out: &mut T) -> Self {
		self.swap_remove_uninit(index, unsafe { out.to_maybeuninit_drop() })
	}

	pub fn swap_remove_uninit(mut self, index: usize, out: &mut MaybeUninit<T>) -> Self {
		out.write(self.inner.swap_remove(index));
		self
	}

	pub fn swap_with_slice(mut self, other: &mut [T]) -> Self {
		self.inner.swap_with_slice(other);
		self
	}

	pub fn truncate(mut self, len: usize) -> Self {
		self.inner.truncate(len);
		self
	}

	// TODO: try_reserve/exact

	/// Calls the provided closure with the spare capacity of the vector as
	/// a [`SliceMutChain`] of [`MaybeUninit`]s.
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
	///       spare = spare.len(&mut spare_len);
	///
	///       // VecChain allocated at least 10 elements worth of space
	///       // since we pushed 5 elements, it should have at least
	///       // 5 elements of spare capacity left
	///       assert!(spare_len >= 5);
	///    })
	///    .push(6)
	///    .push(7)
	///    .spare_capacity_mut(|mut spare| {
	///       spare = spare.len(&mut new_spare_len);
	///
	///       // Just pushed 2 more elements...
	///       assert!(spare_len >= 3);
	///       assert_eq!(spare_len - 2, new_spare_len);
	///    });
	/// ```
	pub fn spare_capacity_mut<CB>(mut self, cb: CB) -> Self
	where
		CB: FnOnce(SliceMutChain<MaybeUninit<T>>)
	{
		cb(self.inner.spare_capacity_mut().into());
		self
	}

	pub fn windows<CB>(self, size: usize, cb: CB) -> Self
	where
		CB: FnOnce(slice::Windows<T>)
	{
		cb(self.inner.windows(size));
		self
	}

	// TODO: utf8_chunks
	// TODO: eq_ignore_ascii_case
	// TODO: escape_ascii
	// TODO: trim_ascii_start/end
	// TODO: trim_ascii

	// doc link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.first_chunk
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
	// TODO: rotate_left/right
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
	// TODO: rotate_left/right
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
	// TODO: eq_ignore_ascii_case
	// TODO: make_ascii_uppercase/lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii
	// TODO: trim_ascii_start/end
}

impl VecChain<f32> {
	pub fn sort_floats(mut self) -> Self {
		self.sort_unstable_by(f32::total_cmp)
	}
}

impl VecChain<f64> {
	pub fn sort_floats(mut self) -> Self {
		self.sort_unstable_by(f64::total_cmp)
	}
}

impl VecChain<u8> {
	pub fn is_ascii(self, out: &mut bool) -> Self {
		self.is_ascii_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn is_ascii_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_ascii());
		self
	}

	pub fn make_ascii_lowercase(mut self) -> Self {
		self.inner.make_ascii_lowercase();
		self
	}

	pub fn make_ascii_uppercase(mut self) -> Self {
		self.inner.make_ascii_uppercase();
		self
	}

	// TODO: as_ascii/unchecked nightly
}

impl<T, const N: usize> VecChain<[T; N]> {
	pub fn flatten(mut self) -> VecChain<T> {
		let (len, cap) = if mem::size_of::<T>() == 0 {
			let len = self.inner.len()
				.checked_mul(N)
				.expect("vec len overflow");
			(len, usize::MAX)
		} else {
			// TODO: wait until 1.79 when this is stabilised
			// unsafe { (
			// 	self.inner.len().unchecked_mul(N),
			// 	self.inner.capacity().unchecked_mul(N)
			// ) }
			(
				self.inner.len() * N,
				self.inner.capacity() * N
			)
		};

		// TODO: switch to into_raw_parts impl when it is stabilised?
		// let (ptr, _len, _capacity) = self.inner.into_raw_parts();

		let ptr = self.nonchain_ptr_mut() as *mut T;
		mem::forget(self);

		unsafe { Vec::from_raw_parts(ptr, len, cap).into() }
	}
}

// nonstandard methods
impl<T> VecChain<T> {
	/// Sorts, then dedups, the vector chain.
	///
	/// Nonstandard API, suggested by my good friend
	/// [Silk Rose] c:
	///
	/// This works exactly the same as `chain.sort().dedup()`.
	///
	/// # Examples
	///
	/// TODO
	///
	/// [Silk Rose]: https://github.com/silkrose
	pub fn sort_and_dedup(mut self) -> Self
	where
		T: Ord
	{
		self.sort().dedup()
	}
}

impl<T> From<Vec<T>> for VecChain<T> {
	fn from(value: Vec<T>) -> Self {
		Self { inner: value }
	}
}

impl<T> AsRef<Vec<T>> for VecChain<T> {
	fn as_ref(&self) -> &Vec<T> {
		&self.inner
	}
}

impl<T> AsMut<Vec<T>> for VecChain<T> {
	fn as_mut(&mut self) -> &mut Vec<T> {
		&mut self.inner
	}
}

impl<T> AsRef<[T]> for VecChain<T> {
	fn as_ref(&self) -> &[T] {
		&self.inner
	}
}

impl<T> AsMut<[T]> for VecChain<T> {
	fn as_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}
}

// TODO: look through traits

#[cfg(test)]
mod tests {
	use super::*;

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
	fn conversion() {
		let slice = &[1u8, 2, 3, 4, 5];
		let mut chain = VecChain::new()
			.extend_from_slice(slice);

		assert_eq!(slice, chain.nonchain_slice());
		assert_eq!(slice, chain.nonchain_slice_mut());
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
	fn with_split_at_spare_mut() {
		let mut uninit_len = 0;
		let chain = VecChain::new()
			.extend_from_slice(&[1usize, 2, 3, 4, 5, 6, 7, 8])
			.reserve(8)
			.split_at_spare_mut(|mut slice, mut uninit| {
				let slice = slice.nonchain_slice_mut();
				let uninit = uninit.nonchain_slice_mut();
				uninit_len = uninit.len();

				assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8]);
				assert!(uninit.len() >= 8);

				uninit.iter_mut()
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
					let slice = slice.nonchain_slice_mut();
					let uninit = uninit.nonchain_slice_mut();

					assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3]);
					assert_eq!(uninit_len - 4, uninit.len());
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
			assert_eq!(chain.nonchain_slice(), &[4, 5, 8, 1, 7, 6, 3, 2]);
		}
	}

	#[test]
	fn reverse() {
		let chain = VecChain::new()
			.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8])
			.reverse();
		assert_eq!(chain.nonchain_slice(), &[8, 7, 6, 5, 4, 3, 2, 1]);
	}

	#[test]
	fn with_chunks() {
		const N: usize = 5;

		let slice = b"1234";

		fn check<'h>(
			expected_chunks: &[&[u8; N]],
			expected_remainder: &'h [u8]
		) -> impl FnOnce(&[[u8; N]], &[u8]) + 'h {
			let expected_chunks = expected_chunks
				.into_iter()
				.map(|item| **item)
				.collect::<Vec<_>>();

			move |chunks, rem| {
				assert_eq!(expected_chunks.len(), chunks.len(), "wrong num of chunks");
				assert_eq!(expected_remainder.len(), rem.len(), "wrong num of elements in remainder");

				assert_eq!(expected_chunks, chunks);
				assert_eq!(expected_remainder, rem);
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
