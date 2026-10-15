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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic as _
	};

	trait Trait {}
	impl Trait for () {}

	#[test]
	fn const_ptr_usage() {
		let expected = cfg_select! {
			target_pointer_width = "16" => { 2 }
			target_pointer_width = "32" => { 4 }
			target_pointer_width = "64" => { 8 }
		};

		let val = ();
		let ptr = &raw const val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*const ()>::MEMORY_USAGE, expected);

		let val = [1, 2, 3, 4, 5];
		let ptr = &raw const val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*const [i32; 5]>::MEMORY_USAGE, expected);
	}

	#[test]
	fn const_ptr_dst_usage() {
		let expected = cfg_select! {
			target_pointer_width = "16" => { 4 }
			target_pointer_width = "32" => { 8 }
			target_pointer_width = "64" => { 16 }
		};

		let val = &[1, 2, 3, 4, 5] as &[_];
		let ptr = &raw const *val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*const [i32]>::MEMORY_USAGE, expected);

		let val = &() as &dyn Trait;
		let ptr = &raw const *val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*const dyn Trait>::MEMORY_USAGE, expected);
	}

	#[test]
	fn mut_ptr_usage() {
		let expected = cfg_select! {
			target_pointer_width = "16" => { 2 }
			target_pointer_width = "32" => { 4 }
			target_pointer_width = "64" => { 8 }
		};

		let mut val = ();
		let ptr = &raw mut val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*mut ()>::MEMORY_USAGE, expected);

		let mut val = [1, 2, 3, 4, 5];
		let ptr = &raw mut val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*mut [i32; 5]>::MEMORY_USAGE, expected);
	}

	#[test]
	fn mut_ptr_dst_usage() {
		let expected = cfg_select! {
			target_pointer_width = "16" => { 4 }
			target_pointer_width = "32" => { 8 }
			target_pointer_width = "64" => { 16 }
		};

		let val = &mut [1, 2, 3, 4, 5] as &mut [_];
		let ptr = &raw mut *val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*mut [i32]>::MEMORY_USAGE, expected);

		let val = &mut () as &mut dyn Trait;
		let ptr = &raw mut *val;

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<*mut dyn Trait>::MEMORY_USAGE, expected);
	}

	#[test]
	fn non_null_ptr_usage() {
		let expected = <*const ()>::MEMORY_USAGE;

		let val = ();
		let ptr = NonNull::from_ref(&val);

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<NonNull<()>>::MEMORY_USAGE, expected);

		let mut val = [1, 2, 3, 4, 5];
		let ptr = NonNull::from_mut(&mut val);

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<NonNull<[i32; 5]>>::MEMORY_USAGE, expected);
	}

	#[test]
	fn non_null_ptr_dst_usage() {
		let expected = <*const [u8]>::MEMORY_USAGE;

		let val = &[1, 2, 3, 4, 5] as &[_];
		let ptr = NonNull::from_ref(val);

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<NonNull<[i32]>>::MEMORY_USAGE, expected);

		let val = &mut () as &mut dyn Trait;
		let ptr = NonNull::from_mut(val);

		assert_eq!(ptr.memory_usage(), expected);
		assert_eq!(ptr.memory_usage_const(), expected);
		assert_eq!(<NonNull<dyn Trait>>::MEMORY_USAGE, expected);
	}
}
