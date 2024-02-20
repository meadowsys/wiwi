use ::parking_lot::{ Once, OnceState };
use ::std::cell::UnsafeCell;
use ::std::mem::ManuallyDrop;
use ::std::ops::{ Deref, DerefMut };
use ::std::panic::{ UnwindSafe, RefUnwindSafe };

pub struct LazyWrap<T, F = fn() -> T>
where
	F: FnOnce() -> T
{
	data: UnsafeCell<Data<T, F>>,
	once: Once
}

union Data<T, F> {
	value: ManuallyDrop<T>,
	init: ManuallyDrop<F>
}

impl<T, F> LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	#[inline]
	pub const fn new(init: F) -> Self {
		let init = ManuallyDrop::new(init);
		let data = UnsafeCell::new(Data { init });
		let once = Once::new();
		Self { data, once }
	}

	#[inline]
	pub fn ensure_initialised(this: &Self) {
		this.once.call_once(|| {
			// SAFETY: if this is executing, this must be the first time its
			// executing, so its safe to take initialiser out of union

			let data = unsafe { &mut *this.data.get() };
			let init = unsafe { ManuallyDrop::take(&mut data.init) };
			let value = init();
			data.value = ManuallyDrop::new(value);
		});
	}

	#[inline]
	fn as_ref(this: &Self) -> &T {
		Self::ensure_initialised(this);
		unsafe { &(*this.data.get()).value }
	}

	#[inline]
	fn as_mut(this: &mut Self) -> &mut T {
		Self::ensure_initialised(this);
		unsafe { &mut (*this.data.get()).value }
	}
}

impl<T, F> Deref for LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	type Target = T;
	#[inline]
	fn deref(&self) -> &T {
		Self::as_ref(self)
	}
}

impl<T, F> DerefMut for LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		Self::as_mut(self)
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
		(**self).as_mut()
	}
}

impl<T, F> Drop for LazyWrap<T, F>
where
	F: FnOnce() -> T
{
	fn drop(&mut self) {
		use OnceState::*;

		match self.once.state() {
			New => {
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().init) }
			}
			Poisoned => {}
			InProgress => {
				self.once.call_once(|| {});
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().value) }
			}
			Done => {
				unsafe { ManuallyDrop::drop(&mut self.data.get_mut().value) }
			}
		}
	}
}

unsafe impl<T, F> Send for LazyWrap<T, F> where T: Send, F: FnOnce() -> T + Send {}
unsafe impl<T, F> Sync for LazyWrap<T, F> where T: Sync, F: FnOnce() -> T + Send {}

impl<T, F> UnwindSafe for LazyWrap<T, F> where T: UnwindSafe, F: FnOnce() -> T + UnwindSafe {}
impl<T, F> RefUnwindSafe for LazyWrap<T, F> where T: RefUnwindSafe, F: FnOnce() -> T + UnwindSafe {}
