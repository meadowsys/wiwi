use std::fmt::{ self, Debug, Display };
use std::mem::ManuallyDrop;
use std::ops::{ Deref, DerefMut };
use std::thread::panicking;

/// Defer some code to run at the end of the current scope
///
/// Note: this macro will capture all values it uses, and hold it until the end
/// of the scope, where the code is actually run. You can specify `move` at the
/// start of the macro to move them in, use [`Cell`](std::cell::Cell) or something
/// similar for interior mutability, use wiwi's `with-cloned` feature and `move`
/// to clone them in, ... or just write the code at the end of the scope, to get
/// around this limitation. Unfortunately, there isn't any (compiler) magic going
/// on here :<
#[macro_export]
macro_rules! defer {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferAlways::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferAlways::new((), |()| { $($defer)* });
	};
}
pub use defer;

/// Defer some code to run at the end of the current scope, but only if the thread
/// is not panicking
///
/// See [`defer!`] for more information
#[macro_export]
macro_rules! defer_success {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferSuccess::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferSuccess::new((), |()| { $($defer)* });
	};
}
pub use defer_success;

/// Defer some code to run at the end of the current scope, but only if the thread
/// is unwinding due to panic
///
/// See [`defer!`] for more information
#[macro_export]
macro_rules! defer_unwind {
	{ move $($defer:tt)* } => {
		let __defer = $crate::defer::DeferUnwind::new((), move |()| { $($defer)* });
	};

	{ $($defer:tt)* } => {
		let __defer = $crate::defer::DeferUnwind::new((), |()| { $($defer)* });
	};
}
pub use defer_unwind;

/// The main deferring container struct
///
/// You're probably looking for one of [`DeferAlways`], [`DeferSuccess`],
/// [`DeferUnwind`], [`DeferRuntime`], or [`DeferRuntimeFn`], as those type aliases
/// are the ones that contain the actual useful interfaces.
#[must_use = "the code intended to be deferred would run immediately because rust drops the value immediately (if you only want to defer running some code, consider `defer!` or its success/unwind variants)"]
pub struct Defer<T, W: when::When, F = fn(T)>
where
	F: FnOnce(T)
{
	value: ManuallyDrop<T>,
	when: ManuallyDrop<W>,
	f: ManuallyDrop<F>
}

mod when {
	use super::*;

	pub trait When {
		fn should_run(self) -> bool;
	}

	#[derive(Debug)]
	pub struct Always;

	impl When for Always {
		#[inline]
		fn should_run(self) -> bool { true }
	}

	#[derive(Debug)]
	pub struct Success;

	impl When for Success {
		#[inline]
		fn should_run(self) -> bool { !panicking() }
	}

	#[derive(Debug)]
	pub struct Unwind;

	impl When for Unwind {
		#[inline]
		fn should_run(self) -> bool { panicking() }
	}

	#[derive(Debug)]
	pub struct Runtime {
		pub(super) should_run: bool
	}

	impl When for Runtime {
		#[inline]
		fn should_run(self) -> bool { self.should_run }
	}

	pub struct RuntimeFn<T, F = fn(T) -> bool>
	where
		F: FnOnce(T) -> bool
	{
		pub(super) value: ManuallyDrop<T>,
		pub(super) f: ManuallyDrop<F>
	}

	impl<T, F> Debug for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool,
		T: Debug
	{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Defer<Runtime>")
				.field("value", &*self.value)
				.finish_non_exhaustive()
		}
	}

	impl<T, F> When for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool
	{
		#[inline]
		fn should_run(self) -> bool {
			unsafe {
				let mut me = ManuallyDrop::new(self);
				let value = ManuallyDrop::take(&mut me.value);
				let f = ManuallyDrop::take(&mut me.f);
				f(value)
			}
		}
	}

	impl<T, F> Drop for RuntimeFn<T, F>
	where
		F: FnOnce(T) -> bool
	{
		fn drop(&mut self) {
			unsafe {
				ManuallyDrop::drop(&mut self.value);
				ManuallyDrop::drop(&mut self.f);
			}
		}
	}
}

/// Run the deferred code always
pub type DeferAlways<T, F = fn(T)> = Defer<T, when::Always, F>;

/// Run the deferred code only if the thread is not panicking
pub type DeferSuccess<T, F = fn(T)> = Defer<T, when::Success, F>;

/// Run the deferred code only if the thread is unwinding due to panick
pub type DeferUnwind<T, F = fn(T)> = Defer<T, when::Unwind, F>;

/// Run the deferred code, based on a condition (`bool` value) evaluated at runtime
pub type DeferRuntime<T, F = fn(T)> = Defer<T, when::Runtime, F>;

/// Run the deferred code, based on a more complex condition (closure taking a
/// stored value, returning a bool) evaluated at runtime
pub type DeferRuntimeFn<
	T,
	Twhen,
	F = fn(T),
	Fwhen = fn(Twhen) -> bool
> = Defer<T, when::RuntimeFn<Twhen, Fwhen>, F>;

impl<T, W, F> Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	/// Internal fn to construct [`Defer`] with a given [`When`](when::When) instance
	#[inline]
	fn _new(value: T, f: F, when: W) -> Self {
		let value = ManuallyDrop::new(value);
		let when = ManuallyDrop::new(when);
		let f = ManuallyDrop::new(f);
		Defer { value, when, f }
	}

	/// Internal fn to disassemble `self`, drop `when`, and reconstruct the
	/// [`Defer`] with the given `when` instance
	#[inline]
	fn _replace_when<W2>(self, when: W2) -> Defer<T, W2, F>
	where
		W2: when::When
	{
		unsafe {
			let mut me = ManuallyDrop::new(self);

			let value = ManuallyDrop::take(&mut me.value);
			let f = ManuallyDrop::take(&mut me.f);
			ManuallyDrop::drop(&mut me.when);

			Defer::_new(value, f, when)
		}
	}

	/// Consumes and returns an instance of [`DeferAlways`] with the same
	/// closure and value
	#[inline]
	pub fn into_always(self) -> DeferAlways<T, F> {
		self._replace_when(when::Always)
	}

	/// Consumes and returns an instance of [`DeferSuccess`] with the same
	/// closure and value
	#[inline]
	pub fn into_on_success(self) -> DeferSuccess<T, F> {
		self._replace_when(when::Success)
	}

	/// Consumes and returns an instance of [`DeferUnwind`] with the same
	/// closure and value
	#[inline]
	pub fn into_on_unwind(self) -> DeferUnwind<T, F> {
		self._replace_when(when::Unwind)
	}

	/// Consumes and returns an instance of [`DeferRuntime`] with the same
	/// closure and value
	#[inline]
	pub fn into_runtime(self, should_run: bool) -> DeferRuntime<T, F> {
		self._replace_when(when::Runtime { should_run })
	}

	/// Consumes and returns an instance of [`DeferRuntimeFn`] with the same
	/// closure and value
	#[inline]
	pub fn into_runtime_fn<Twhen, Fwhen>(
		self,
		should_run_value: Twhen,
		should_run: Fwhen
	) -> DeferRuntimeFn<T, Twhen, F, Fwhen>
	where
		Fwhen: FnOnce(Twhen) -> bool
	{
		let value = ManuallyDrop::new(should_run_value);
		let f = ManuallyDrop::new(should_run);
		self._replace_when(when::RuntimeFn { value, f })
	}

	/// Consumes and returns back the value and the function
	#[inline]
	pub fn into_inner(self) -> (T, F) {
		let mut me = ManuallyDrop::new(self);

		let value = unsafe { ManuallyDrop::take(&mut me.value) };
		let _when = unsafe { ManuallyDrop::take(&mut me.when) };
		let f = unsafe { ManuallyDrop::take(&mut me.f) };

		(value, f)
	}
}

impl<T, F> DeferAlways<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Always)
	}
}

impl<T, F> DeferSuccess<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Success)
	}
}

impl<T, F> DeferUnwind<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F) -> Self {
		Self::_new(value, f, when::Unwind)
	}
}

impl<T, F> DeferRuntime<T, F>
where
	F: FnOnce(T)
{
	#[inline]
	pub fn new(value: T, f: F, should_run: bool) -> Self {
		Self::_new(value, f, when::Runtime { should_run })
	}

	/// Returns if this [`DeferRuntime`] will run
	#[inline]
	pub fn should_run(&self) -> bool {
		self.when.should_run
	}

	/// Set if this [`DeferRuntime`] should run
	#[inline]
	pub fn set_should_run(&mut self, should_run: bool) {
		self.when.should_run = should_run;
	}
}

impl<T, Twhen, F, Fwhen> DeferRuntimeFn<T, Twhen, F, Fwhen>
where
	F: FnOnce(T),
	Fwhen: FnOnce(Twhen) -> bool
{
	#[inline]
	pub fn new(value: T, f: F, should_run_value: Twhen, should_run: Fwhen) -> Self {
		Self::_new(value, f, when::RuntimeFn {
			value: ManuallyDrop::new(should_run_value),
			f: ManuallyDrop::new(should_run)
		})
	}

	/// Takes a copy of the should run value, if it implements [`Copy`]
	#[inline]
	pub fn should_run_value(&self) -> Twhen
	where
		Twhen: Copy
	{
		*self.when.value
	}

	/// Returns a reference to the should run value
	#[inline]
	pub fn should_run_value_ref(&self) -> &Twhen {
		&self.when.value
	}

	/// Returns a mut reference to the should run value
	#[inline]
	pub fn should_run_value_mut(&mut self) -> &mut Twhen {
		&mut self.when.value
	}

	/// Sets the should run value to the provided value, dropping the
	/// previous one
	#[inline]
	pub fn set_should_run_value(&mut self, should_run_value: Twhen) {
		let old = self.swap_should_run_value(should_run_value);
		drop(old);
	}

	/// Swaps the stored should run value with the provided value, then
	/// returns it
	#[inline]
	pub fn swap_should_run_value(&mut self, should_run_value: Twhen) -> Twhen {
		unsafe {
			let old = ManuallyDrop::take(&mut self.when.value);
			self.when.value = ManuallyDrop::new(should_run_value);
			old
		}
	}
}

impl<T, W, F> Deref for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		&self.value
	}
}

impl<T, W, F> DerefMut for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T, W, F> Drop for Defer<T, W, F>
where
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn drop(&mut self) {
		unsafe {
			let value = ManuallyDrop::take(&mut self.value);
			let when = ManuallyDrop::take(&mut self.when);
			let f = ManuallyDrop::take(&mut self.f);

			if !when.should_run() { return }
			f(value);
		}
	}
}

impl<T, W, F> Debug for Defer<T, W, F>
where
	T: Debug,
	W: when::When + Debug,
	F: FnOnce(T)
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Defer")
			.field("value", &**self)
			.field("when", &*self.when)
			.finish_non_exhaustive()
	}
}

impl<T, W, F> Display for Defer<T, W, F>
where
	T: Display,
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		(**self).fmt(f)
	}
}

impl<T, W, F, U> AsRef<U> for Defer<T, W, F>
where
	T: AsRef<U>,
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn as_ref(&self) -> &U {
		self.value.as_ref()
	}
}

impl<T, W, F, U> AsMut<U> for Defer<T, W, F>
where
	T: AsMut<U>,
	W: when::When,
	F: FnOnce(T)
{
	#[inline]
	fn as_mut(&mut self) -> &mut U {
		self.value.as_mut()
	}
}

pub trait OnDrop: Sized {
	#[inline]
	fn on_drop<F>(self, f: F) -> DeferAlways<Self, F>
	where
		F: FnOnce(Self)
	{
		DeferAlways::new(self, f)
	}

	#[inline]
	fn on_success_drop<F>(self, f: F) -> DeferSuccess<Self, F>
	where
		F: FnOnce(Self)
	{
		DeferSuccess::new(self, f)
	}

	#[inline]
	fn on_unwind_drop<F>(self, f: F) -> DeferUnwind<Self, F>
	where
		F: FnOnce(Self)
	{
		DeferUnwind::new(self, f)
	}
}

impl<T> OnDrop for T {}

#[cfg(test)]
mod tests {
	// TODO: need to do something about the way these test are run, since
	// assert/etc panic and we catch_unwind sometimes, so that's, hmmm
	use super::*;
	use std::cell::Cell;
	use std::panic::{ AssertUnwindSafe, catch_unwind };

	#[test]
	fn defer_always_success() {
		let cell = Cell::new("");

		{
			defer!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_always_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_success_success() {
		let cell = Cell::new("");

		{
			defer_success!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "glory is cute");
	}

	#[test]
	fn defer_success_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer_success!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "");
	}

	#[test]
	fn defer_unwind_success() {
		let cell = Cell::new("");

		{
			defer_unwind!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
		}

		assert_eq!(cell.get(), "");
	}

	#[test]
	fn defer_unwind_unwind() {
		let cell = Cell::new("");

		let _ = catch_unwind(AssertUnwindSafe(|| {
			defer_unwind!(cell.set("glory is cute"));
			assert_eq!(cell.get(), "");
			panic!("panick a");
		}));

		assert_eq!(cell.get(), "glory is cute");
	}
}
