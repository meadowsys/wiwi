use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::MaybeUninit;
use std::ptr;

pub trait WithVars: Sized {
	/// # Safety
	///
	/// If `f` or any of its code panics, as of now this will cause
	/// undefined behaviour. Additionally, you _must_ write to the `MaybeUninit`
	/// instance provided.
	unsafe fn with_vars_uninit<V, F>(&mut self, f: F) -> V
	where
		F: FnOnce(Self, &mut MaybeUninit<V>) -> Self
	{
		let mut vars = MaybeUninit::uninit();

		// SAFETY: we have a mut reference, so there's static guarantee
		// that no one else will have / try to access this var
		let temp_self = ptr::read(self);
		let temp_self = f(temp_self, &mut vars);

		// SAFETY: we "took" self above, and now we're returning it
		// after passing it to the fn
		ptr::write(self, temp_self);

		// SAFETY: caller promises that `f` will write to `vars`
		vars.assume_init()
	}
}

impl<T> WithVars for T {}
