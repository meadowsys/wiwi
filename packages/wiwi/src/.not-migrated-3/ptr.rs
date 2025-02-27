use crate::rust_std::marker::Sized;

#[inline(always)]
pub const fn coerce_ptr<T: ?Sized>(ptr: &T) -> *const T {
	ptr
}

#[inline(always)]
pub fn coerce_mut_ptr<T: ?Sized>(ptr: &mut T) -> *mut T {
	ptr
}
