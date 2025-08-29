use crate::prelude::*;
use crate::macro_util::void;
use self::alloc_mod::Layout;
use self::atomic::Ordering::*;

#[repr(transparent)]
pub struct RcInner<C, V, S> {
	ptr: ptr::NonNull<RcLayout<C, V, S>>
}

// if fields in this struct need to change,
// make sure to change `calc_layout` accordingly
#[repr(C)]
struct RcLayout<C, V, S> {
	/// The reference counter (handles counting both strong and weak references)
	counter: C,

	/// The length of the slice stored in the unsized portion
	slice_len: usize,

	/// The value (the sized portion)
	value: V,

	/// A "header" of the unsized slice portion I guess?
	///
	/// This forces this struct to have an alignment of (at least) S's alignment,
	/// while also not requiring that there be at least 1 S element in this struct
	/// itself, and the slice will follow right after this field.
	slice: [S; 0]
}

#[inline]
pub fn new_from_value<C, V>(value: V) -> RcInner<C, V, ()>
where
	C: Counter
{
	new_from_value_and_slice_copy(value, &[])
}

#[inline]
pub fn new_from_array_into_slice<C, S, const N: usize>(array: [S; N]) -> RcInner<C, (), S>
where
	C: Counter
{
	new_from_value_and_array_into_slice((), array)
}

#[inline]
pub fn new_from_slice_clone<C, S>(slice: &[S]) -> RcInner<C, (), S>
where
	C: Counter,
	S: Clone
{
	new_from_value_and_slice_clone((), slice)
}

#[inline]
pub fn new_from_slice_copy<C, S>(slice: &[S]) -> RcInner<C, (), S>
where
	C: Counter,
	S: Copy
{
	new_from_value_and_slice_copy((), slice)
}

#[inline]
pub fn new_from_value_and_array_into_slice<C, V, S, const N: usize>(value: V, array: [S; N]) -> RcInner<C, V, S>
where
	C: Counter
{
	let array = ManuallyDrop::new(array);

	// SAFETY: we put the array into `ManuallyDrop`
	unsafe { new_from_value_and_slice_copy_unchecked(value, &*array) }
}

#[inline]
pub fn new_from_value_and_slice_clone<C, V, S>(value: V, slice: &[S]) -> RcInner<C, V, S>
where
	C: Counter,
	S: Clone
{
	let instance = alloc_instance::<_, _, S>(slice.len());

	// SAFETY:
	// - instance just allocated in statement above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { void!(value_uninit(instance).write(value)) }

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	slice.iter().enumerate().for_each(|(i, s)| {
		// SAFETY: `ptr` is writeable for `slice.len()` elements
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: see above
		unsafe { ptr.write(s.clone()) }
	});

	instance
}

#[inline]
pub fn new_from_value_and_slice_copy<C, V, S>(value: V, slice: &[S]) -> RcInner<C, V, S>
where
	C: Counter,
	S: Copy
{
	// SAFETY: `S: Copy` enforced by trait bound
	unsafe { new_from_value_and_slice_copy_unchecked(value, slice) }
}

/// # Safety
///
/// The provided slice should either contain elements that implement [`Copy`],
/// or the input slice should be prevented from dropping to avoid double
/// dropping elements.
#[inline]
unsafe fn new_from_value_and_slice_copy_unchecked<C, V, S>(value: V, slice: &[S]) -> RcInner<C, V, S>
where
	C: Counter
{
	let instance = alloc_instance(slice.len());

	// SAFETY:
	// - instance just allocated in statement above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { void!(value_uninit(instance).write(value)) }

	// SAFETY: instance just allocated in statement above
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: `ptr` is writeable for `slice.len()` elements
	unsafe {
		ptr::copy_nonoverlapping(
			slice.as_ptr(),
			ptr,
			slice.len()
		)
	}

	instance
}

/// Initialise a new instance with the provided length
///
/// The instance returned will have fields `counter` and `slice_length` fields
/// initialised. Counter is set to 1 strong and 1 weak according to contract of
/// [`Counter`]. Caller is responsible for initialising the `value` and `slice`
/// fields.
#[inline]
fn alloc_instance<C, V, S>(slice_len: usize) -> RcInner<C, V, S>
where
	C: Counter
{
	let layout = calc_layout::<C, V, S>(slice_len);

	// SAFETY: `calc_layout` never returns layout with 0 size
	let ptr = unsafe { alloc(layout) };

	let Some(ptr) = ptr::NonNull::new(ptr.cast()) else {
		alloc_mod::handle_alloc_error(layout)
	};

	let instance = RcInner { ptr };

	// we can fill in counter since we know the type of counter already
	// SAFETY:
	// - instance just allocated in statements above
	// - because just allocated, we must have exclusive reference to `instance`
	// - reference is used just for this single `write` statement and
	//   dropped immediately after
	unsafe { void!(counter_uninit(instance).write(C::new())) }

	// we can fill in length since that will never change
	// SAFETY: same as above
	unsafe { void!(slice_len_uninit(instance).write(slice_len)) }

	instance
}

/// Drop the value and slice contents of the provided instance
///
/// # Safety
///
/// This instance must be fully initialised, and this must be the first time
/// this function is called on this particular `instance`.
///
/// There also must not be any existing references to `slice`
/// so we can safely create an exclusive reference to the slice to drop it.
/// Ideally we can do this through a fat pointer directly without needing to
/// create an intermediate wide reference, but, those APIs are unstable right
/// now smh
#[inline]
pub unsafe fn drop_instance<C, V, S>(instance: RcInner<C, V, S>)
where
	C: Counter
{
	// SAFETY: caller promises `instance` is fully initialised
	let value_ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: see above
	unsafe { ptr::drop_in_place(value_ptr) }

	// SAFETY: caller promises `instance` is fully initialised, and that we can
	// safely create an exclusive reference. If this is only called in `drop`
	// handler of the last strong pointer, this should be safe. We drop here
	// using this temporary exclusive reference which is dropped before this
	// function returns, so dealloc can run without triggering UB
	let slice_ref = unsafe { slice_mut(instance) };

	// SAFETY: see above
	unsafe { ptr::drop_in_place(slice_ref) }
}

/// Drop the counter and deallocate the backing allocation of the provided instance
///
/// # Safety
///
/// This instance must be in the partially initialised state following a call to
/// [`drop_instance`], and this must be the first time this function is called on
/// this particular `instance`. This may be called on an instance that is still
/// fully initialised (ie. [`drop_instance`] has not been called on it), but
/// that is equivalent to leaking the value and slice fields, and is almost
/// certainly incorrect.
#[inline]
pub unsafe fn dealloc_instance<C, V, S>(instance: RcInner<C, V, S>)
where
	C: Counter
{
	// SAFETY: caller promises `counter` is initialised
	let counter_ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: see above
	unsafe { ptr::drop_in_place(counter_ptr) }

	// SAFETY: caller promises `slice_len` is initialised
	let slice_len = unsafe { slice_len(instance) };

	let layout = calc_layout::<C, V, S>(slice_len);

	// SAFETY: see above
	unsafe { dealloc(instance.ptr.as_ptr().cast(), layout) }
}

/// Calculate the layout to allocate a new instance with the specified counter,
/// value type, slice type, and slice length
// TODO: make this fn `const` when `feature(const_alloc_layout)` is stable
#[inline]
fn calc_layout<C, V, S>(slice_len: usize) -> Layout {
	fn inner<C, V, S>(slice_len: usize) -> Option<Layout> {
		// if the size of `V` is not an even multiple of the align of the rest of the
		// struct (max of `usize` and `C`), and align of `S` is less than or equal to
		// align of `V`, the `slice` field will be at the end of `V` and there will be
		// some padding after it. I don't think this causes UB, but it will allocate
		// and use more memory than is necessary in these edge cases. So, we calculate
		// it manually (we can do this because `repr(C)`), to attach the real layout
		// of the slice where `slice` would have been, and place some additional checks
		// in debug to assert it would have been the same as just using `Layout::new`.

		let mut layout = Layout::new::<()>();

		macro_rules! extend_layout {
			($field_name:ident, $layout:expr) => {
				let new = layout
					.extend($layout)
					.ok()?;

				debug_assert_eq!(
					mem::offset_of!(RcLayout<C, V, S>, $field_name),
					new.1
				);

				layout = new.0;
			}
		}

		extend_layout!(counter, Layout::new::<C>());
		extend_layout!(slice_len, Layout::new::<usize>());
		extend_layout!(value, Layout::new::<V>());
		extend_layout!(slice, Layout::array::<S>(slice_len).ok()?);

		Some(layout.pad_to_align())
	}

	inner::<C, V, S>(slice_len).expect("rc layout calculation failed")
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
unsafe fn counter_ptr<C, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<C>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw mut (*instance.ptr.as_ptr()).counter };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr) }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into `counter` (exclusive borrow)
#[inline]
unsafe fn counter_uninit<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<C>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - The provided `instance` must have field `counter` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
pub unsafe fn counter_ref<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h C
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { counter_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { &*ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
#[inline]
unsafe fn slice_len_ptr<C, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<usize>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw mut (*instance.ptr.as_ptr()).slice_len };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr) }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into `slice_len` (exclusive borrow)
#[inline]
unsafe fn slice_len_uninit<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<usize>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_len_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been deallocated
/// - The provided `instance` must have field `slice_len` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
unsafe fn slice_len<C, V, S>(instance: RcInner<C, V, S>) -> usize
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_len_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { *ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn value_ptr<C, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<V>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw mut (*instance.ptr.as_ptr()).value };

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - The returned reference must be the only mut reference into `value` (exclusive borrow)
#[inline]
unsafe fn value_uninit<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h mut MaybeUninit<V>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid, and `MaybeUninit` has same ABI as inner type
	unsafe { &mut *ptr.cast() }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - The provided `instance` must have field `value` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
pub unsafe fn value_ref<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h V
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { value_ptr(instance).as_ptr() };

	// SAFETY: ptr is valid
	unsafe { &*ptr }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
#[inline]
unsafe fn slice_thin_ptr<C, V, S>(instance: RcInner<C, V, S>) -> ptr::NonNull<S>
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { &raw mut (*instance.ptr.as_ptr()).slice };
	let ptr = ptr.cast::<S>();

	// SAFETY: ptr is guaranteed to be nonnull
	unsafe { ptr::NonNull::new_unchecked(ptr) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - The provided `instance` must have field `slice_len` already initialised
/// - The provided `instance` must have `slice_len` elements in `slice` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
#[inline]
pub unsafe fn slice_ref<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h [S]
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: caller promises to uphold the requirements
	let slice_len = unsafe { slice_len(instance) };

	// SAFETY: caller promises ptr is valid for at least `len` elements
	unsafe { slice::from_raw_parts(ptr, slice_len) }
}

/// # Safety
///
/// - The provided `instance` must not have been dropped or deallocated
/// - The provided `instance` must have field `slice_len` already initialised
/// - The provided `instance` must have `slice_len` elements in `slice` already initialised
/// - `instance` must outlive `'h` (the lifetime of the returned reference)
/// - there must not be any other references, shared or exclusive, to `instance`,
///   so the returned exclusive reference can be valid
#[inline]
pub unsafe fn slice_mut<'h, C, V, S>(instance: RcInner<C, V, S>) -> &'h mut [S]
where
	C: Counter
{
	// SAFETY: caller promises to uphold the requirements
	let ptr = unsafe { slice_thin_ptr(instance).as_ptr() };

	// SAFETY: caller promises to uphold the requirements
	let slice_len = unsafe { slice_len(instance) };

	// SAFETY: caller promises ptr is valid for at least `len` elements
	unsafe { slice::from_raw_parts_mut(ptr, slice_len) }
}

impl<C, V, S> Clone for RcInner<C, V, S>
where
	C: Counter
{
	#[inline]
	fn clone(&self) -> Self {
		*self
	}
}

impl<C, V, S> Copy for RcInner<C, V, S>
where
	C: Counter
{}

/// Trait for structs that can count references
///
/// `wiwi` includes two implementations: one for single threaded access (akin
/// to `std`'s [`Rc`]), and the other for atomic multithreaded access (akin to
/// `std`'s [`Arc`]).
///
/// # Safety
///
/// You must implement this trait correctly (ie. functions must return correct
/// values), as values returned from functions are directly used to control the
/// allocation/deallocation of memory and dropping of values.
pub unsafe trait Counter: Sized {
	/// Create a new couter with strong and weak count both set to 1
	fn new() -> Self;

	/// Get the strong reference count
	fn strong_count(&self) -> usize;

	/// Get the weak reference count
	///
	/// Don't subtract the "fake" weak reference that
	/// is held by all the strong references.
	fn weak_count(&self) -> usize;

	/// Increment the strong count for creation of a new strong reference
	fn inc_strong_for_new_ref(&self);

	/// Decrements the strong count for dropping a reference, returning `true`
	/// if there are no more strong pointers left (and the value and items in
	/// the slice should be dropped)
	fn dec_strong_for_drop(&self) -> bool;

	/// Increments the weak count for creation of a new weak reference
	fn inc_weak_for_new_ref(&self);

	/// Decrements the weak count for dropping a reference, returning `true`
	/// if there are no more weak pointers left (and the allocation should be
	/// deallocated)
	fn dec_weak_for_drop(&self) -> bool;

	/// Increment the strong count if it is possible to upgrade a weak pointer
	/// to strong, and return `true`, otherwise return `false` and do nothing
	fn try_inc_strong_for_upgrade(&self) -> bool;
}

pub struct ThreadCounter {
	strong: cell::Cell<usize>,
	weak: cell::Cell<usize>,
	__not_thread_safe: PhantomData<*const ()>
}

// SAFETY: we implement everything correctly
unsafe impl Counter for ThreadCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: cell::Cell::new(1),
			weak: cell::Cell::new(1),
			__not_thread_safe: PhantomData
		}
	}

	#[inline]
	fn strong_count(&self) -> usize {
		self.strong.get()
	}

	#[inline]
	fn weak_count(&self) -> usize {
		self.weak.get()
	}

	#[inline]
	fn inc_strong_for_new_ref(&self) {
		let old = self.strong.get();
		self.strong.set(old + 1);
	}

	#[inline]
	fn dec_strong_for_drop(&self) -> bool {
		let old = self.strong.get();
		self.strong.set(old - 1);
		old == 1
	}

	#[inline]
	fn inc_weak_for_new_ref(&self) {
		let old = self.weak.get();
		self.weak.set(old + 1);
	}

	#[inline]
	fn dec_weak_for_drop(&self) -> bool {
		let old = self.weak.get();
		self.weak.set(old - 1);
		old == 1
	}

	#[inline]
	fn try_inc_strong_for_upgrade(&self) -> bool {
		let old = self.strong.get();
		let should_upgrade = old > 0;

		if should_upgrade {
			self.strong.set(old + 1)
		}

		should_upgrade
	}
}

pub struct AtomicCounter {
	strong: AtomicUsize,
	weak: AtomicUsize
}

// SAFETY: we implement everything correctly
unsafe impl Counter for AtomicCounter {
	#[inline]
	fn new() -> Self {
		Self {
			strong: AtomicUsize::new(1),
			weak: AtomicUsize::new(1)
		}
	}

	#[inline]
	fn strong_count(&self) -> usize {
		self.strong.load(Relaxed)
	}

	#[inline]
	fn weak_count(&self) -> usize {
		self.weak.load(Relaxed)
	}

	#[inline]
	fn inc_strong_for_new_ref(&self) {
		self.strong.fetch_add(1, Relaxed);
	}

	#[inline]
	fn dec_strong_for_drop(&self) -> bool {
		let old = self.strong.fetch_sub(1, Release);
		if old != 1 { return false }

		atomic::fence(Acquire);
		true
	}

	#[inline]
	fn inc_weak_for_new_ref(&self) {
		self.weak.fetch_add(1, Relaxed);
	}

	#[inline]
	fn dec_weak_for_drop(&self) -> bool {
		let old = self.weak.fetch_sub(1, Release);
		if old != 1 { return false }

		atomic::fence(Acquire);
		true
	}

	#[inline]
	fn try_inc_strong_for_upgrade(&self) -> bool {
		self.strong
			.fetch_update(
				Acquire,
				Relaxed,
				|old| (old > 0).then(move || old + 1)
			)
			.is_ok()
	}
}
