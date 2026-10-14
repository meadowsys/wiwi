use crate::macros::impl_memory_usage_static;
use core::ptr::NonNull;

// we cannot trust that the pointer is safe to dereference, so we can only take
// size of the pointer itself. Abstractions built on top of pointers probably
// shouldn't rely on these implementations for data and can calculate their own
// with more accurate info on the contents the pointers point to

impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	*const T
	=> size_of::<*const T>();
}

impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	*mut T
	=> size_of::<*mut T>();
}

impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	NonNull<T>
	=> size_of::<NonNull<T>>();
}
