extern crate cfg_if;

use crate::prelude::*;
use self::private::SealedStruct;
use cfg_if::cfg_if;

#[cfg_attr(feature = "nightly", const_trait)]
pub trait MemUse {
	/// Unsure the use case, but let's check that these are dyn compatible for now?
	#[doc(hidden)]
	#[inline]
	fn __assert_dyn_compat(&self, _: &dyn MemUse, _: SealedStruct) {}

	/// Calculates total memory usage, including inline and indirect,
	#[inline]
	fn mem_use(&self) -> usize {
		self.mem_use_inline() + self.mem_use_indirect()
	}

	/// Calculates inline memory usage (ex. heap usage of a [`Vec`], always 24
	/// bytes on 64-bit architextures)
	fn mem_use_inline(&self) -> usize;

	/// Calculates indirect memory usage (ex. heap usage of a [`Vec`])
	fn mem_use_indirect(&self) -> usize;
}

// todo: trying to remove the sized bound (remove me when done)
// const _: usize = <() as MemUseConst>::MEM_USE;
pub trait MemUseConst
where
	Self: MemUse
{
	/// The constant memory usage for this type, if this value is always the same
	const MEM_USE_MAYBE: Option<usize> = add_maybe_const(
		Self::MEM_USE_INLINE_MAYBE,
		Self::MEM_USE_INDIRECT_MAYBE
	);

	/// The constant inline memory usage for this type, if this value is always the same
	const MEM_USE_INLINE_MAYBE: Option<usize>;

	/// The constant indirect memory usage for this type, if this value is always the same
	const MEM_USE_INDIRECT_MAYBE: Option<usize>;

	/// The constant memory usage for this type
	const MEM_USE: usize = Self::MEM_USE_MAYBE.unwrap();

	/// The constant memory usage for this type
	const MEM_USE_INLINE: usize = Self::MEM_USE_INLINE_MAYBE.unwrap();

	/// The constant memory usage for this type
	const MEM_USE_INDIRECT: usize = Self::MEM_USE_INDIRECT_MAYBE.unwrap();
}

cfg_if! {
	if #[cfg(feature = "nightly")] {
		#[inline]
		pub const fn mem_use<T>(val: &T) -> usize
		where
			T: ~const MemUse
		{
			val.mem_use()
		}
	} else {
		#[inline]
		pub fn mem_use<T>(val: &T) -> usize
		where
			T: MemUse
		{
			val.mem_use()
		}
	}
}

cfg_if! {
	if #[cfg(feature = "nightly")] {
		#[inline]
		pub const fn mem_use_inline<T>(val: &T) -> usize
		where
			T: ~const MemUse
		{
			val.mem_use_inline()
		}
	} else {
		#[inline]
		pub fn mem_use_inline<T>(val: &T) -> usize
		where
			T: MemUse
		{
			val.mem_use_inline()
		}
	}
}

cfg_if! {
	if #[cfg(feature = "nightly")] {
		#[inline]
		pub const fn mem_use_indirect<T>(val: &T) -> usize
		where
			T: ~const MemUse
		{
			val.mem_use_indirect()
		}
	} else {
		#[inline]
		pub fn mem_use_indirect<T>(val: &T) -> usize
		where
			T: MemUse
		{
			val.mem_use_indirect()
		}
	}
}

#[inline]
pub const fn mem_use_const<T>() -> usize
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE
}

#[inline]
pub const fn mem_use_const_inline<T>() -> usize
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE_INLINE
}

#[inline]
pub const fn mem_use_const_indirect<T>() -> usize
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE_INDIRECT
}

#[inline]
pub const fn mem_use_const_maybe<T>() -> Option<usize>
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE_MAYBE
}

#[inline]
pub const fn mem_use_const_inline_maybe<T>() -> Option<usize>
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE_INLINE_MAYBE
}

#[inline]
pub const fn mem_use_const_indirect_maybe<T>() -> Option<usize>
where
	T: ?Sized + MemUseConst
{
	T::MEM_USE_INDIRECT_MAYBE
}

const fn add_maybe_const(a1: Option<usize>, a2: Option<usize>) -> Option<usize> {
	if let (Some(a1), Some(a2)) = (a1, a2) {
		Some(a1 + a2)
	} else {
		None
	}
}

mod private {
	pub struct SealedStruct {
		__private: ()
	}
}

// stack_only_impl! { () }
// stack_only_impl! { bool }
// stack_only_impl! { char }
// stack_only_impl! { u8 }
// stack_only_impl! { u16 }
// stack_only_impl! { u32 }
// stack_only_impl! { u64 }
// stack_only_impl! { u128 }
// stack_only_impl! { usize }
// stack_only_impl! { i8 }
// stack_only_impl! { i16 }
// stack_only_impl! { i32 }
// stack_only_impl! { i64 }
// stack_only_impl! { i128 }
// stack_only_impl! { isize }
// stack_only_impl! { #[cfg(feature = "nightly")] [] f16 }
// stack_only_impl! { f32 }
// stack_only_impl! { f64 }
// stack_only_impl! { #[cfg(feature = "nightly")] [] f128 }
// // stack_only_impl! { [T: ?Sized] *const T }
// // stack_only_impl! { [T: ?Sized] *mut T }
// stack_only_impl! { NonZero<u8> }
// stack_only_impl! { NonZero<u16> }
// stack_only_impl! { NonZero<u32> }
// stack_only_impl! { NonZero<u64> }
// stack_only_impl! { NonZero<u128> }
// stack_only_impl! { NonZero<usize> }
// stack_only_impl! { NonZero<i8> }
// stack_only_impl! { NonZero<i16> }
// stack_only_impl! { NonZero<i32> }
// stack_only_impl! { NonZero<i64> }
// stack_only_impl! { NonZero<i128> }
// stack_only_impl! { NonZero<isize> }
// stack_only_impl! { Saturating<u8> }
// stack_only_impl! { Saturating<u16> }
// stack_only_impl! { Saturating<u32> }
// stack_only_impl! { Saturating<u64> }
// stack_only_impl! { Saturating<u128> }
// stack_only_impl! { Saturating<usize> }
// stack_only_impl! { Saturating<i8> }
// stack_only_impl! { Saturating<i16> }
// stack_only_impl! { Saturating<i32> }
// stack_only_impl! { Saturating<i64> }
// stack_only_impl! { Saturating<i128> }
// stack_only_impl! { Saturating<isize> }
// stack_only_impl! { Wrapping<u8> }
// stack_only_impl! { Wrapping<u16> }
// stack_only_impl! { Wrapping<u32> }
// stack_only_impl! { Wrapping<u64> }
// stack_only_impl! { Wrapping<u128> }
// stack_only_impl! { Wrapping<usize> }
// stack_only_impl! { Wrapping<i8> }
// stack_only_impl! { Wrapping<i16> }
// stack_only_impl! { Wrapping<i32> }
// stack_only_impl! { Wrapping<i64> }
// stack_only_impl! { Wrapping<i128> }
// stack_only_impl! { Wrapping<isize> }
// stack_only_impl! { AtomicBool }
// stack_only_impl! { AtomicU8 }
// stack_only_impl! { AtomicU16 }
// stack_only_impl! { AtomicU32 }
// stack_only_impl! { AtomicU64 }
// stack_only_impl! { #[cfg(feature = "nightly")] [] atomic::AtomicU128 }
// stack_only_impl! { AtomicUsize }
// stack_only_impl! { AtomicI8 }
// stack_only_impl! { AtomicI16 }
// stack_only_impl! { AtomicI32 }
// stack_only_impl! { AtomicI64 }
// stack_only_impl! { #[cfg(feature = "nightly")] [] atomic::AtomicI128 }
// stack_only_impl! { AtomicIsize }
// stack_only_impl! { [T] AtomicPtr<T> }
// stack_only_impl! { [T: ?Sized] PhantomData<T> }
// stack_only_impl! { PhantomPinned }

// impl_mem_use! {
// 	[T] [&T]
// 	where { T: ?Sized + MemUse }
// 	where const { T: ?Sized + ~const MemUse }
// 	where const maybe { T: ?Sized + MemUseConstMaybe }
// 	const { map_maybe_const(Some(size_of::<&T>()), T::MEM_USE_MAYBE) }
// 	const inline { map_maybe_const(Some(size_of::<&T>()), T::MEM_USE_MAYBE) }
// 	const indirect { map_maybe_const(Some(size_of::<&T>()), T::MEM_USE_MAYBE) }
// 	// mem_use_inline(&self) { size_of::<&T>() }
// 	// mem_use_indirect(&self) { T::mem_use(self) }
// }

// impl_mem_use! {
// 	[T] [&mut T]
// 	where { T: ?Sized + MemUseConstMaybe }
// 	where const { T: ?Sized + ~const MemUse }
// 	where const maybe { T: ?Sized + MemUseConstMaybe }
// 	const = map_maybe_const(Some(size_of::<&mut T>()), T::MEM_USE_MAYBE);
// 	mem_use_inline(&self) { size_of::<&T>() }
// 	mem_use_indirect(&self) { T::mem_use(self) }
// }

// impl_mem_use! {
// 	[T] [Option<T>]
// 	where { T: MemUse }
// 	where const { T: ~const MemUse }
// 	where const maybe { T: MemUseConstMaybe }
// 	const = None;
// 	mem_use_inline(&self) { size_of::<Option<T>>() }
// 	mem_use_indirect(&self) {
// 		match self {
// 			Some(v) => { T::mem_use_indirect(v) }
// 			None => { 0 }
// 		}
// 	}
// }

// // cfg_if! {
// // 	if #[cfg(feature = "nightly")] {
// // 		impl<T, E> const MemUse for Result<T, E>
// // 		where
// // 			T: ~const MemUse,
// // 			E: ~const MemUse
// // 		{
// // 			#[inline]
// // 			fn mem_use_inline(&self) -> usize {
// // 				size_of::<Result<T, E>>()
// // 			}

// // 			#[inline]
// // 			fn mem_use_indirect(&self) -> usize {
// // 				match self {
// // 					Ok(v) => { T::mem_use_indirect(v) }
// // 					Err(e) => { E::mem_use_indirect(e) }
// // 				}
// // 			}
// // 		}
// // 	} else {
// // 		impl<T, E> MemUse for Result<T, E>
// // 		where
// // 			T: MemUse,
// // 			E: MemUse
// // 		{
// // 			#[inline]
// // 			fn mem_use_inline(&self) -> usize {
// // 				size_of::<Result<T, E>>()
// // 			}

// // 			#[inline]
// // 			fn mem_use_indirect(&self) -> usize {
// // 				match self {
// // 					Ok(v) => { T::mem_use_indirect(v) }
// // 					Err(e) => { E::mem_use_indirect(e) }
// // 				}
// // 			}
// // 		}
// // 	}
// // }

// // impl<T, E> MemUseConst for Result<T, E>
// // where
// // 	T: MemUseConst,
// // 	E: MemUseConst
// // {
// // 	const MEM_USE: usize = size_of::<Result<T, E>>();
// // }

// // impl<T> MemUse for [T]
// // where
// // 	T: MemUse
// // {
// // 	#[inline]
// // 	fn mem_use(&self) -> usize {
// // 		self.iter().map(T::mem_use).sum()
// // 	}

// // 	#[inline]
// // 	fn mem_use_inline(&self) -> usize {
// // 		size_of_val(self)
// // 	}

// // 	#[inline]
// // 	fn mem_use_indirect(&self) -> usize {
// // 		self.iter().map(T::mem_use_indirect).sum()
// // 	}
// // }

// // impl<T> MemUseStatic for [T]
// // where
// // 	T: MemUseStatic
// // {
// // 	#[inline]
// // 	fn mem_use_static(&self) -> usize {
// // 		let mut i = 0;
// // 		let mut mem_use = 0;

// // 		while i < self.len() {
// // 			mem_use += self[i].mem_use_static();
// // 			i += 1;
// // 		}

// // 		mem_use
// // 	}
// // }

// // impl MemUse for str {
// // 	#[inline]
// // 	fn mem_use(&self) -> usize {
// // 		self.len()
// // 	}

// // 	#[inline]
// // 	fn mem_use_inline(&self) -> usize {
// // 		self.len()
// // 	}

// // 	#[inline]
// // 	fn mem_use_indirect(&self) -> usize {
// // 		0
// // 	}
// // }

// // impl MemUseStatic for str {
// // 	#[inline]
// // 	fn mem_use_static(&self) -> usize {
// // 		self.len()
// // 	}
// // }

// // TODO: Option, tuples, Result, Vec, String, [T; N], the ranges, cells, path, osstr, cstr, rc, arc, types in this crate
// // TODO: go through std's list of types lol

// macro_rules! impl_mem_use {
// 	{
// 		[$($generics:tt)*] [$($type:tt)*]

// 		$(#[$impl_meta:meta])*
// 		where { $($where:tt)* }
// 		$(#[$impl_const_meta:meta])*
// 		where const { $($where_const:tt)* }
// 		$(#[$impl_const_maybe_meta:meta])*
// 		where const maybe { $($where_const_maybe:tt)* }

// 		$self:ident

// 		$(#[$fn_meta:meta])*
// 		fn { $($fn:tt)* }
// 		$(#[$fn_inline_meta:meta])*
// 		fn inline { $($fn_inline:tt)* }
// 		$(#[$fn_indirect_meta:meta])*
// 		fn indirect { $($fn_indirect:tt)* }

// 		$(#[$const_meta:meta])*
// 		const { $($const_maybe:tt)* }
// 		$(#[$const_inline_meta:meta])*
// 		const inline { $($const_inline_maybe:tt)* }
// 		$(#[$const_indirect_meta:meta])*
// 		const indirect { $($const_indirect_maybe:tt)* }
// 	} => {
// 		cfg_if! {
// 			if #[cfg(not(feature = "nightly"))] {
// 				$(#[$impl_const_meta])*
// 				impl<$($generics)*> MemUse for $($type)*
// 				where $($where)*
// 				{
// 					$(#[$fn_meta])*
// 					#[inline]
// 					fn mem_use(&$self) -> usize {
// 						impl_mem_use! {
// 							@process_default
// 							{ $($fn)* }
// 							{ $self.mem_use_inline() + $self.mem_use_indirect() }
// 						}
// 					}

// 					$(#[$fn_inline_meta])*
// 					fn mem_use_inline(&$self) -> usize {
// 						$($fn_inline)*
// 					}

// 					$(#[$fn_indirect_meta])*
// 					fn mem_use_indirect(&$self) -> usize {
// 						$($fn_indirect)*
// 					}
// 				}
// 			} else {
// 				$(#[$impl_meta])*
// 				impl<$($generics)*> const MemUse for $($type)*
// 				where $($where_const)*
// 				{
// 					$(#[$fn_meta])*
// 					#[inline]
// 					fn mem_use(&$self) -> usize {
// 						impl_mem_use! {
// 							@process_default
// 							{ $($fn)* }
// 							{ $self.mem_use_inline() + $self.mem_use_indirect() }
// 						}
// 					}

// 					$(#[$fn_inline_meta])*
// 					#[inline]
// 					fn mem_use_inline(&$self) -> usize {
// 						$($fn_inline)*
// 					}

// 					$(#[$fn_indirect_meta])*
// 					#[inline]
// 					fn mem_use_indirect(&$self) -> usize {
// 						$($fn_indirect)*
// 					}
// 				}
// 			}
// 		}

// 		$(#[$impl_const_maybe_meta])*
// 		impl<$($generics)*> MemUseConstMaybe for $($type)*
// 		where $($where_const_maybe)*
// 		{
// 			$(#[$const_meta])*
// 			const MEM_USE_MAYBE: Option<usize> = impl_mem_use! { @process_default { $($const_maybe)* } {
// 				map_maybe_const(
// 					<Self as MemUseConstMaybe>::MEM_USE_INLINE_MAYBE,
// 					<Self as MemUseConstMaybe>::MEM_USE_INDIRECT_MAYBE
// 				)
// 			} };

// 			$(#[$const_inline_meta])*
// 			const MEM_USE_INLINE_MAYBE: Option<usize> = $($const_inline_maybe)*;

// 			$(#[$const_indirect_meta])*
// 			const MEM_USE_INDIRECT_MAYBE: Option<usize> = $($const_indirect_maybe)*;
// 		}
// 	};

// 	{ @process_default { _ } { $($default:tt)* } } => { $($default)* };
// 	{ @process_default { $($stuff:tt)* } { $($unused:tt)* } } => { $($stuff)* };
// }
// use impl_mem_use;

// macro_rules! stack_only_impl {
// 	{ $(#[$meta:meta])* [$($generics:tt)*] $($type:tt)* } => {
// 		impl_mem_use! {
// 			$(#[$meta])*
// 			[$($generics)*] [$($type)*]
// 			where {}
// 			where const {}
// 			where const maybe {}

// 			self

// 			fn { _ }
// 			fn inline { size_of::<$($type)*>() }
// 			fn indirect { 0 }

// 			const { _ }
// 			const inline { Some(size_of::<$($type)*>()) }
// 			const indirect { Some(0) }
// 		}
// 	};

// 	{ $(#[$meta:meta])* $($ty:tt)* } => { stack_only_impl! { $(#[$meta])* [] $($ty)* } };
// }
// use stack_only_impl;
