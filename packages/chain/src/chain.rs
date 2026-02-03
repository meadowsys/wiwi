#[must_use = "a chain takes ownership of itself, performs the action, then returns itself again"]
pub struct Chain<T> {
	inner: T
}

impl<T> Chain<T> {
	#[inline]
	pub const fn from_inner(inner: T) -> Self {
		Self { inner }
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.inner
	}

	#[inline]
	pub const fn as_inner(&self) -> &T {
		&self.inner
	}

	#[inline]
	pub const fn as_inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	#[inline]
	pub const fn as_inner_chain(&self) -> Chain<&T> {
		Chain::from_inner(self.as_inner())
	}

	#[inline]
	pub const fn as_inner_mut_chain(&mut self) -> Chain<&mut T> {
		Chain::from_inner(self.as_inner_mut())
	}

	#[inline]
	pub fn with_inner<F>(self, f: F) -> Self
	where
		F: FnOnce(&T)
	{
		f(self.as_inner());
		self
	}

	#[inline]
	pub fn with_inner_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut T)
	{
		f(self.as_inner_mut());
		self
	}
}

pub trait ChainInner: Sized {
	#[inline]
	fn from_chain(chain: Chain<Self>) -> Self {
		chain.into_inner()
	}

	#[inline]
	fn into_chain(self) -> Chain<Self> {
		Chain::from_inner(self)
	}
}

impl<T> ChainInner for T {}

pub(crate) trait ChainInnerType {
	type Inner;
}

impl<T> ChainInnerType for Chain<T> {
	type Inner = T;
}

/// # Safety
///
/// By using this trait, implementors of functions are promising to call
/// [`write`](Output::write), so that when the function returns, there is a
/// value written to the output. For example, users can pass a reference to
/// [`MaybeUninit`](core::mem::MaybeUninit) and rely on the fact that it got
/// initialised to safely call [`assume_init`](core::mem::MaybeUninit::assume_init).
///
/// idk how to enforce the above properly using unsafe etc.
pub unsafe trait Output<T>: Sized + private::Sealed<T> {
	/// Stores a value
	fn write(self, item: T);
}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut T {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		*self = item;
	}
}
impl<T> private::Sealed<T> for &mut T {}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut Option<T> {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		*self = Some(item);
	}
}
impl<T> private::Sealed<T> for &mut Option<T> {}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut core::mem::MaybeUninit<T> {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		self.write(item);
	}
}
impl<T> private::Sealed<T> for &mut core::mem::MaybeUninit<T> {}

/// Tool for helping to debug [`Output`] trait usage in debug mode (if `out` is
/// not written to, the function will panic)
///
/// This function should optimise out to a no-op in release mode.
#[inline]
pub fn out_dbg<T, O: Output<T>>(out: O) -> OutputDebug<T, O> {
	OutputDebug {
		inner: out,
		__marker: std::marker::PhantomData
	}
}

#[repr(transparent)]
pub struct OutputDebug<T, O>
where
	O: Output<T>
{
	inner: O,
	__marker: core::marker::PhantomData<fn(T)>
}

impl<T, O> OutputDebug<T, O>
where
	O: Output<T>
{
	/// Unwraps self and returns the inner output (without ever panicking)
	#[inline]
	pub fn into_inner(self) -> O {
		// in cfg(debug_assertions), we have Drop impl,
		// so we need to do a funny to get `inner` out
		#[cfg(debug_assertions)]
		let inner = {
			let this = core::mem::ManuallyDrop::new(self);

			// SAFETY: ManuallyDrop above prevents double drops
			unsafe { core::ptr::read(&raw const this.inner) }
		};

		// in not(cfg(debug_assertions)), we don't have
		// Drop impl, so we can just normally move it out
		#[cfg(not(debug_assertions))]
		let inner = self.inner;

		inner
	}
}

// SAFETY: we write once to `self`
unsafe impl<T, O> Output<T> for OutputDebug<T, O>
where
	O: Output<T>
{
	#[inline]
	fn write(self, item: T) {
		self.into_inner().write(item);
	}
}

impl<T, O> private::Sealed<T> for OutputDebug<T, O>
where
	O: Output<T>
{}

#[cfg(debug_assertions)]
impl<T, O> Drop for OutputDebug<T, O>
where
	O: Output<T>
{
	#[inline]
	fn drop(&mut self) {
		// writing to this slot will prevent this panic from being called (via ManuallyDrop)
		// additionally this drop impl only is enabled if debug assertions is enabled
		panic!("`write` not called on created instance of `Output` (this is a bug)")
	}
}

mod private {
	pub trait Sealed<T> {}
}
