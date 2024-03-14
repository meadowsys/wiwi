//! Wrapper for initialisation function, initialising it only on first access.
//!
//! Works in static contexts (static variables)

use ::parking_lot::{ Once, OnceState };
use ::std::cell::UnsafeCell;
use ::std::fmt::{ self, Debug, Display };
use ::std::mem::ManuallyDrop;
use ::std::ops::{ Deref, DerefMut };
use ::std::panic::{ RefUnwindSafe, UnwindSafe };
use ::std::ptr;

union Data<T, F> {
	init: ManuallyDrop<F>,
	value: ManuallyDrop<T>
}

/// A lazily initialised data wrapper that initialises itself on first access
pub struct LazyWrap<T, F = fn() -> T> {
	data: UnsafeCell<Data<T, F>>,
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
			let data = unsafe { &mut (*this.data.get()) };
			let init = unsafe { ManuallyDrop::take(&mut data.init) };
			let value = init();
			data.value = ManuallyDrop::new(value);
		});
	}

	#[inline]
	fn ref_inner(this: &Self) -> &T {
		Self::ensure_initialised(this);
		unsafe { &(*this.data.get()).value }
	}

	#[inline]
	fn mut_inner(this: &mut Self) -> &mut T {
		Self::ensure_initialised(this);
		unsafe { &mut (*this.data.get()).value }
	}

	/// Returns true or false, depending on if the value is initialised.
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
	pub fn into_inner(this: Self) -> LazyWrapState<T, F> {
		let initialised = Self::is_initialised(&this);
		let this = ManuallyDrop::new(this);
		let data = unsafe { ptr::read(this.data.get()) };

		if initialised {
			let value = ManuallyDrop::into_inner(unsafe { data.value });
			LazyWrapState::Initialised(value)
		} else {
			let init = ManuallyDrop::into_inner(unsafe { data.init });
			LazyWrapState::Uninitialised(init)
		}
	}

	/// Ensures that the value is initialised, then returns the value.
	pub fn into_inner_initialised(this: Self) -> T {
		Self::ensure_initialised(&this);
		let this = ManuallyDrop::new(this);
		let data = unsafe { ptr::read(this.data.get()) };
		ManuallyDrop::into_inner(unsafe { data.value })
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

unsafe impl<T, F> Send for LazyWrap<T, F> where T: Send, F: Send {}
unsafe impl<T, F> Sync for LazyWrap<T, F> where T: Sync, F: Send {}
impl<T, F> UnwindSafe for LazyWrap<T, F> where T: UnwindSafe, F: UnwindSafe {}
impl<T, F> RefUnwindSafe for LazyWrap<T, F> where T: RefUnwindSafe, F: UnwindSafe {}
impl<T, F> Unpin for LazyWrap<T, F> where T: Unpin, F: Unpin {}

impl<T, F> Debug for LazyWrap<T, F>
where
	T: Debug,
	F: FnOnce() -> T
{
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&**self, f)
	}
}

impl<T, F> Drop for LazyWrap<T, F> {
	fn drop(&mut self) {
		use OnceState::*;
		match self.once.state() {
			New => {
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().init) }
			}
			Poisoned => {}
			InProgress => {
				// ???
				// lets drop the thing once its done just in case
				// this cannot happen though. if we're dropping, we're the last one with a reference.

				self.once.call_once(|| {});
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().value) }
			}
			Done => {
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().value) }
			}
		}
	}
}
