extern crate parking_lot;

use crate::prelude::*;
use parking_lot::{ Once, OnceState };

/// Inner data union
///
/// Note: in implementation of [`LazyWrap`], the "discriminant" is held by its
/// [`Once`] instance
union Data<T, F> {
	/// Initialisation function
	init: ManuallyDrop<F>,

	/// Initialised value (from initialisation function)
	value: ManuallyDrop<T>
}

/// A lazily initialised data wrapper that initialises itself on first access
pub struct LazyWrap<T, F = fn() -> T> {
	/// The data, either the initialisation function or the initialised value
	///
	/// Note: state is kept track of in [`once`](LazyWrap::once)
	data: UnsafeCell<Data<T, F>>,

	/// The [`Once`] instance responsible for syncronising references,
	/// ensuring the provided initialisation function is only taken/called once,
	/// and holding the state of the [data union](LazyWrap::data)
	once: Once
}

/// Returned by [`LazyWrap::into_inner`], containing the initialised value if
/// its already initialised, or otherwise the initialisation function.
pub enum LazyWrapState<T, F> {
	/// Contains previously initialised value
	Initialised(T),
	/// Value is not initialised, contains initialisation function.
	Uninitialised(F)
}

impl<T, F> LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	/// Creates a new uninitialised instance that will be initialised with the
	/// provided initialisation function.
	#[inline]
	pub const fn new(init: F) -> Self {
		let init = ManuallyDrop::new(init);

		// `Once` instance is just initialised, and data enum instance holds
		// the init fn
		let data = UnsafeCell::new(Data { init });
		let once = Once::new();

		Self { data, once }
	}

	/// Runs initialisation if the value is not initialised yet, and
	/// blocks until it is complete.
	///
	/// Note: [`Deref`] and [`DerefMut`] automatically initialise if necessary.
	#[inline]
	pub fn ensure_initialised(this: &Self) {
		this.once.call_once(|| {
			// SAFETY: `Once` ensures only one closure is only being run in one thread
			// at a time, and this is the only code site that uses `call_once`
			// (ie. if this closure is running / we're in this closure,
			// it's guaranteed that no other closures are being run), so its safe
			// to get a reference to the UnsafeCell contents
			let data = unsafe { &mut *this.data.get() };

			// SAFETY: `Once` ensures this closure is only called once, globally,
			// per `LazyWrap` instance, so the data in here must be the
			// initialisation function
			let init = unsafe { ManuallyDrop::take(&mut data.init) };

			let value = init();
			data.value = ManuallyDrop::new(value);
		});
	}

	/// Ensures the inner value is initialised, then
	/// gets a reference to it
	#[inline]
	fn ref_inner(this: &Self) -> &T {
		Self::ensure_initialised(this);

		// SAFETY: we have immutable borrow access to the data (guaranteed by rust
		// type system), and so can create an immutable reference to the data
		// within the `UnsafeCell`
		let data = unsafe { &*this.data.get() };

		// SAFETY: we just ensured initialised earlier in this fn,
		// so accessing the data from `value` is sound
		unsafe { &data.value }
	}

	/// Ensures the inner value is initialised, then
	/// gets a mut reference to it
	#[inline]
	fn mut_inner(this: &mut Self) -> &mut T {
		Self::ensure_initialised(this);

		// SAFETY: we have mut borrow access to the data (guaranteed by rust
		// type system), and so can create a mut reference to the data
		// within the `UnsafeCell`
		let data = unsafe { &mut *this.data.get() };

		// SAFETY: we just ensured initialised earlier in this fn,
		// so accessing the data from `value` is sound
		unsafe { &mut data.value }
	}

	/// Returns true or false, depending on if the value is initialised.
	///
	/// # Panics
	///
	/// Panics if the provided initialisation function panicked.
	#[inline]
	pub fn is_initialised(this: &Self) -> bool {
		use OnceState::*;
		match this.once.state() {
			New => { false }
			Poisoned => { panic!("initialiser panicked") }
			InProgress => {
				this.once.call_once(|| {});
				true
			}
			Done => { true }
		}
	}

	/// Fetch the value if its initialised, or return the initialisation function
	/// if it isn't.
	#[inline]
	pub fn into_inner(this: Self) -> LazyWrapState<T, F> {
		let this = ManuallyDrop::new(this);

		// SAFETY: we own `this` and have wrapped it in `ManuallyDrop` to
		// prevent double drops, and so taking this will be the only copy of data.
		// Additionally we wrapped it before taking it, so if `Self::is_initialised`
		// panicks there's no issue there
		let data = unsafe { ptr::read(this.data.get()) };

		if Self::is_initialised(&this) {
			// SAFETY: `this` is initialised (checked by if statement),
			// so `data` will contain the initialised value
			// and accessing it is sound
			let value = unsafe { data.value };

			let value = ManuallyDrop::into_inner(value);
			LazyWrapState::Initialised(value)
		} else {
			// SAFETY: `this` is not initialised (checked by if statement),
			// so `init` will contain the initialisation function
			// and accessing it is sound
			let init = unsafe { data.init };

			let init = ManuallyDrop::into_inner(init);
			LazyWrapState::Uninitialised(init)
		}
	}

	/// Ensures that the value is initialised, then returns the value.
	#[inline]
	pub fn into_inner_initialised(this: Self) -> T {
		Self::ensure_initialised(&this);
		let this = ManuallyDrop::new(this);

		// SAFETY: we own `this` and have wrapped it in `ManuallyDrop` to
		// prevent double drops, and so taking this will be the only copy of data.
		// Additionally we wrapped it before taking it, so if `Self::is_initialised`
		// panicks there's no issue there
		let data = unsafe { ptr::read(this.data.get()) };

		// SAFETY: we just ensured initialised earlier in this fn,
		// so accessing the data from `value` is sound
		let value = unsafe { data.value };

		ManuallyDrop::into_inner(value)
	}
}

impl<T, F> Deref for LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	type Target = T;
	#[inline]
	fn deref(&self) -> &Self::Target {
		// ensure_initialised is called by ref_inner
		Self::ref_inner(self)
	}
}

impl<T, F> DerefMut for LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		// ensure_initialised is called by mut_inner
		Self::mut_inner(self)
	}
}

impl<T, U, F> AsRef<U> for LazyWrap<T, F>
where
	F: FnOnce() -> T,
	T: AsRef<U>,
	U: ?Sized
{
	#[inline]
	fn as_ref(&self) -> &U {
		// ensure_initialised called by Deref
		(**self).as_ref()
	}
}

impl<T, U, F> AsMut<U> for LazyWrap<T, F>
where
	F: FnOnce() -> T,
	T: AsMut<U>,
	U: ?Sized
{
	#[inline]
	fn as_mut(&mut self) -> &mut U {
		// ensure_initialised called by DerefMut
		(**self).as_mut()
	}
}

// SAFETY: `LazyWrap` is `Send` if `T` and `F` are both `Send`.
// Sending `LazyWrap` across threads can cause both `T` and `F` to be sent.
// `F` can be sent to and run on another thread, so `F` must also be `Send`.
unsafe impl<T, F> Send for LazyWrap<T, F>
where
	T: Send,
	F: Send
{}

// SAFETY: `LazyWrap` is `Sync` if `T` is `Sync` and `F` is `Send`.
// Sharing `LazyWrap` across threads can cause `T` to be shared. `F` may be
// run on the other thread via the shared reference if `LazyWrap` hasn't been
// initialised/accessed yet, so `F` must be `Send`.
unsafe impl<T, F> Sync for LazyWrap<T, F>
where
	T: Sync,
	F: Send
{}

// `UnwindSafe` if `T` and `F` are both `UnwindSafe`.
// Sending `LazyWrap` across an unwind boundary will send `T` and `F` both. `T`
// may be accessed and `F` may be called across an unwind boundary, and code may
// panic while both is happening, so both `T` and `F` must be `UnwindSafe`.
impl<T, F> UnwindSafe for LazyWrap<T, F>
where
	T: UnwindSafe,
	F: UnwindSafe
{}

// `RefUnwindSafe` if `T` is `RefUnwindSafe` and `F` is `UnwindSafe`.
// Sending references of `LazyWrap` will send `T` as a reference across. `F`
// may be run and panic on the other side of the boundary if `LazyWrap` hasn't
// been initialised yet, so must be `UnwindSafe`.
impl<T, F> RefUnwindSafe for LazyWrap<T, F>
where
	T: RefUnwindSafe,
	F: UnwindSafe
{}

// `Unpin` if `T` and `F` are both `Unpin`.
// If either `T` or `F` cannot move, we cannot move either
// (ie. cannot implement Unpin).
impl<T, F> Unpin for LazyWrap<T, F>
where
	T: Unpin,
	F: Unpin
{}

impl<T, F> Debug for LazyWrap<T, F>
where
	T: Debug,
	F: FnOnce() -> T
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if Self::is_initialised(self) {
			f.debug_struct("LazyWrap")
				.field("initialised", &true)
				.field("data", &**self)
				.finish()
		} else {
			f.debug_struct("LazyWrap")
				.field("initialised", &false)
				.finish_non_exhaustive()
		}
	}
}

impl<T, F> Display for LazyWrap<T, F>
where
	T: Display,
	F: FnOnce() -> T
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&**self, f)
	}
}

impl<T, F> Drop for LazyWrap<T, F> {
	#[inline]
	fn drop(&mut self) {
		use OnceState::*;
		match self.once.state() {
			New => {
				// SAFETY: `self.once` has not been run yet, so this must be
				// the initialisation function, so this field access must be sound
				let init = unsafe { &mut self.data.get_mut().init };

				// SAFETY: no one has dropped `init` before, and we are in drop hook,
				// so no one can drop after
				unsafe { ManuallyDrop::drop(init) }
			}
			Poisoned => {
				// just let it drop without panicking again I guess..
			}
			InProgress => {
				// ???

				// lets wait on `call_once`, then drop the thing, just in case
				// this cannot happen though. if we're dropping, we're the last one with a reference.
				self.once.call_once(|| {});

				// SAFETY: `self.once` has been run, so this must be
				// the initialised value, so this field access must be sound
				let value = unsafe { &mut self.data.get_mut().value };

				// SAFETY: no one has dropped `value` before, and we are in drop hook,
				// so no one can drop after
				unsafe { ManuallyDrop::drop(value) }
			}
			Done => {
				// SAFETY: `self.once` has been run, so this must be
				// the initialised value, so this field access must be sound
				let value = unsafe { &mut self.data.get_mut().value };

				// SAFETY: no one has dropped `value` before, and we are in drop hook,
				// so no one can drop after
				unsafe { ManuallyDrop::drop(value) }
			}
		}
	}
}
