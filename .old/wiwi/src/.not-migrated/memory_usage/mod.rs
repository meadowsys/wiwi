use std::cell::{ Cell, LazyCell, OnceCell, RefCell, UnsafeCell };
use std::ffi::{ CStr, CString, OsStr, OsString };
use std::marker::{ PhantomData, PhantomPinned };
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
use std::num::{ NonZero, Saturating, Wrapping };
use std::path::{ Path, PathBuf };
use std::ops::{ Bound, ControlFlow, Range, RangeFull, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive };
use std::sync::atomic::*;
use std::time::Duration;

// todo: write doc comments on the _impl fns themselves_ describing the logic inside

/// Trait for types that can report its current total memory usage
pub trait MemoryUsage {
	/// Gets the amount of memory this value uses on the "stack"
	///
	/// "Stack" here is defined vaguely as the amount of memory that a value
	/// itself takes up, excluding any indirections. Self-explanatorily, for
	/// sized values, this is the same as [`size_of`], (ie. `u64` would return
	/// 8 (bytes), and `&str` would return 16 (bytes) on a 64-bit platform)).
	/// However... this trait and method can be implemented on unsized types,
	/// such as `[T]` (not `&[T]`). For `[T]` this would be the len of the slice
	/// multiplied by the size of T (so does not include the reference size).
	fn mem_use_stack(&self) -> usize;

	/// Gets the amount of memory this value uses on the "heap", excluding extra
	/// if applicable
	///
	/// For example, a [`Vec`] would be using its `capacity`, rather than its `len`.
	///
	/// "Heap" here is defined vaguely as the amount of memory that a value itself
	/// holds, via indirections. So this would include the contents for a [`Box`],
	/// including its total usage.
	fn mem_use_heap(&self) -> usize;

	/// Gets the amount of memory this value uses on the "heap", excluding extra
	/// if applicable
	///
	/// For example, a [`Vec`] would be using its `len`, rather than its `capacity`.
	///
	/// See [`mem_use_heap`](MemoryUsage::mem_use_heap) for information on what
	/// "heap" means here.
	fn mem_use_heap_excl_extra_capacity(&self) -> usize;

	/// Gets the total amount of memory this value has allocated, including extra
	/// if applicable
	///
	/// This is the same as adding the values from
	/// [`mem_use_stack`](MemoryUsage::mem_use_stack) and
	/// [`mem_use_heap`](MemoryUsage::mem_use_heap) together, but can be
	/// overridden if there are more efficient ways to calculate this for a
	/// specific type.
	#[inline]
	fn mem_use(&self) -> usize {
		self.mem_use_stack() + self.mem_use_heap()
	}

	/// Gets the total amount of memory this value is using, excluding extra
	/// if applicable
	///
	/// This is the same as adding the values from
	/// [`mem_use_stack`](MemoryUsage::mem_use_stack) and
	/// [`mem_use_heap`](MemoryUsage::mem_use_heap_excl_extra_capacity) together,
	/// but can be overridden if there are more efficient ways to calculate this
	/// for a specific type.
	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		self.mem_use_stack() + self.mem_use_heap_excl_extra_capacity()
	}

	/// Tells a value to remove its extra unused memory, if applicable and possible
	///
	/// This will recurse into inner types, causing all structs and nested
	/// structs to deallocate as much as possible.
	///
	/// For types where this is not applicable (ex. simple number types), this
	/// will just silently do nothing. There is a default implementation provided
	/// that is a noop.
	#[inline]
	fn shrink_extra(&mut self) {}
}

/// Trait for types that can report its total memory usage in a static context
///
/// Currently this trait is kinda... useless, but it'll become more useful once
/// traits can be `const`.
pub trait MemoryUsageStatic: MemoryUsage {
	/// Gets the static memory usage for this value
	fn mem_use_static(&self) -> usize;
}

/// Trait for types that have the same size no matter the value
///
/// For example, number types: [`u64`] will always be size 8, etc.
pub trait MemoryUsageConst: Sized + MemoryUsageStatic {
	/// The constant memory usage for this type
	const MEM_USE_CONST: usize;
}

// if possible I think `MemoryUsage` and `MemoryUsageStatic` can be
// trait objects... I'm not sure for what use case exactly, but still
const _: &dyn MemoryUsage = &0u8;
const _: &dyn MemoryUsageStatic = &0u8;
// const is not object safe

/// Provides an impl of [`MemoryUsage::mem_use_stack`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsage`].
macro_rules! mem_use_stack_size_of_impl {
	() => {
		#[inline]
		fn mem_use_stack(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MemoryUsage::mem_use_stack`] by just returning 0, for
/// implementations on unsized types directly (instead of a reference to them)
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsage`].
macro_rules! mem_use_stack_zero_impl {
	() => {
		#[inline]
		fn mem_use_stack(&self) -> usize {
			0
		}
	}
}

/// Provides an impl of [`MemoryUsage::mem_use_heap`] and
/// [`MemoryUsage::mem_use_heap_excl_extra_capacity`]
/// for types that never allocate onto the heap, returning just 0
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsage`].
macro_rules! mem_use_heap_zero_impl {
	() => {
		#[inline]
		fn mem_use_heap(&self) -> usize {
			0
		}

		#[inline]
		fn mem_use_heap_excl_extra_capacity(&self) -> usize {
			0
		}
	}
}

/// Provides an impl of [`MemoryUsageStatic::mem_use_static`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsageStatic`].
macro_rules! mem_use_static_size_of_impl {
	() => {
		#[inline]
		fn mem_use_static(&self) -> usize {
			::std::mem::size_of::<Self>()
		}
	}
}

/// Provides an impl of [`MemoryUsageConst::MEM_USE_CONST`]
/// using [`size_of`]
///
/// Use by invoking this macro within an impl block for the
/// trait [`MemoryUsageConst`].
macro_rules! mem_use_const_size_of_impl {
	() => {
		const MEM_USE_CONST: usize = ::std::mem::size_of::<Self>();
	}
}

/// Implements [`MemoryUsage`], [`MemoryUsageStatic`], and [`MemoryUsageConst`]
/// for types that only live on the stack (stack usage is using [`size_of`],
/// and heap usage is 0)
macro_rules! stack_only_impl {
	{ [$($generics:tt)*] $($type:tt)+ } => {
		impl<$($generics)*> MemoryUsage for $($type)+ {
			mem_use_stack_size_of_impl!();
			mem_use_heap_zero_impl!();
		}

		impl<$($generics)*> MemoryUsageStatic for $($type)+ {
			mem_use_static_size_of_impl!();
		}

		impl<$($generics)*> MemoryUsageConst for $($type)+ {
			mem_use_const_size_of_impl!();
		}
	}
}

stack_only_impl!([] ());

stack_only_impl!([] bool);
stack_only_impl!([] char);

stack_only_impl!([] u8);
stack_only_impl!([] u16);
stack_only_impl!([] u32);
stack_only_impl!([] u64);
stack_only_impl!([] u128);
stack_only_impl!([] usize);

stack_only_impl!([] i8);
stack_only_impl!([] i16);
stack_only_impl!([] i32);
stack_only_impl!([] i64);
stack_only_impl!([] i128);
stack_only_impl!([] isize);

// stack_only_impl!([] f16);
stack_only_impl!([] f32);
stack_only_impl!([] f64);
// stack_only_impl!([] f128);

stack_only_impl!([T: ?Sized] *const T);
stack_only_impl!([T: ?Sized] *mut T);

stack_only_impl!([] NonZero<u8>);
stack_only_impl!([] NonZero<u16>);
stack_only_impl!([] NonZero<u32>);
stack_only_impl!([] NonZero<u64>);
stack_only_impl!([] NonZero<u128>);
stack_only_impl!([] NonZero<usize>);

stack_only_impl!([] NonZero<i8>);
stack_only_impl!([] NonZero<i16>);
stack_only_impl!([] NonZero<i32>);
stack_only_impl!([] NonZero<i64>);
stack_only_impl!([] NonZero<i128>);
stack_only_impl!([] NonZero<isize>);

stack_only_impl!([] Saturating<u8>);
stack_only_impl!([] Saturating<u16>);
stack_only_impl!([] Saturating<u32>);
stack_only_impl!([] Saturating<u64>);
stack_only_impl!([] Saturating<u128>);
stack_only_impl!([] Saturating<usize>);

stack_only_impl!([] Saturating<i8>);
stack_only_impl!([] Saturating<i16>);
stack_only_impl!([] Saturating<i32>);
stack_only_impl!([] Saturating<i64>);
stack_only_impl!([] Saturating<i128>);
stack_only_impl!([] Saturating<isize>);

stack_only_impl!([] Wrapping<u8>);
stack_only_impl!([] Wrapping<u16>);
stack_only_impl!([] Wrapping<u32>);
stack_only_impl!([] Wrapping<u64>);
stack_only_impl!([] Wrapping<u128>);
stack_only_impl!([] Wrapping<usize>);

stack_only_impl!([] Wrapping<i8>);
stack_only_impl!([] Wrapping<i16>);
stack_only_impl!([] Wrapping<i32>);
stack_only_impl!([] Wrapping<i64>);
stack_only_impl!([] Wrapping<i128>);
stack_only_impl!([] Wrapping<isize>);

#[cfg(target_has_atomic = "8")]
stack_only_impl!([] AtomicBool);

#[cfg(target_has_atomic = "8")]
stack_only_impl!([] AtomicU8);
#[cfg(target_has_atomic = "16")]
stack_only_impl!([] AtomicU16);
#[cfg(target_has_atomic = "32")]
stack_only_impl!([] AtomicU32);
#[cfg(target_has_atomic = "64")]
stack_only_impl!([] AtomicU64);
// #[cfg(target_has_atomic = "128")]
// stack_only_impl!([] AtomicU128);
#[cfg(target_has_atomic = "ptr")]
stack_only_impl!([] AtomicUsize);

#[cfg(target_has_atomic = "8")]
stack_only_impl!([] AtomicI8);
#[cfg(target_has_atomic = "16")]
stack_only_impl!([] AtomicI16);
#[cfg(target_has_atomic = "32")]
stack_only_impl!([] AtomicI32);
#[cfg(target_has_atomic = "64")]
stack_only_impl!([] AtomicI64);
// #[cfg(target_has_atomic = "128")]
// stack_only_impl!([] AtomicI128);
#[cfg(target_has_atomic = "ptr")]
stack_only_impl!([] AtomicIsize);

#[cfg(target_has_atomic = "ptr")]
stack_only_impl!([T] AtomicPtr<T>);

stack_only_impl!([] Duration);

stack_only_impl!([] IpAddr);
stack_only_impl!([] Ipv4Addr);
stack_only_impl!([] Ipv6Addr);

stack_only_impl!([T: ?Sized] PhantomData<T>);
stack_only_impl!([] PhantomPinned);

stack_only_impl!([] RangeFull);

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use(*self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_excl_extra_capacity(*self)
	}

	// what to do about shrink extra? should we panic? or is no op fine?
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h T>() + T::mem_use_static(*self)
	}
}

impl<'h, T: MemoryUsageConst> MemoryUsageConst for &'h T {
	const MEM_USE_CONST: usize = size_of::<&'h T>() + T::MEM_USE_CONST;
}

impl<'h, T: ?Sized + MemoryUsage> MemoryUsage for &'h mut T {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use(*self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_excl_extra_capacity(*self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(*self)
	}
}

impl<'h, T: ?Sized + MemoryUsageStatic> MemoryUsageStatic for &'h mut T {
	#[inline]
	fn mem_use_static(&self) -> usize {
		size_of::<&'h mut T>() + T::mem_use_static(*self)
	}
}

impl<'h, T: MemoryUsageConst> MemoryUsageConst for &'h mut T {
	const MEM_USE_CONST: usize = size_of::<&'h mut T>() + T::MEM_USE_CONST;
}

impl<T: MemoryUsage> MemoryUsage for Option<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		match self {
			Some(val) => { val.mem_use_heap() }
			None => { 0 }
		}
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		match self {
			Some(val) => { val.mem_use_heap_excl_extra_capacity() }
			None => { 0 }
		}
	}

	#[inline]
	fn shrink_extra(&mut self) {
		if let Some(val) = self {
			val.shrink_extra()
		}
	}
}

macro_rules! tuple_impl {
	{ $($params:ident)* } => {
		tuple_impl! { @impl [$($params)*] [] }
	};

	{ @impl [] [$($done:ident)*] } => {
		// done
	};

	{ @impl [$first:ident $($remainder:ident)*] [$($done:ident)*] } => {
		tuple_impl! { @impl_real $($done)* $first }
		tuple_impl! { @impl [$($remainder)*] [$($done)* $first] }
	};

	{ @impl_real $($params:ident)* } => {
		impl<$($params: MemoryUsage,)*> MemoryUsage for ($($params,)*) {
			mem_use_stack_size_of_impl!();

			#[inline]
			fn mem_use_heap(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_heap
					$($params)*
				}
			}

			#[inline]
			fn mem_use_heap_excl_extra_capacity(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_heap_excl_extra_capacity
					$($params)*
				}
			}

			#[inline]
			fn mem_use(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use
					$($params)*
				}
			}

			#[inline]
			fn mem_use_excl_extra_capacity(&self) -> usize {
				tuple_impl! {
					@impl_fn
					self
					mem_use_excl_extra_capacity
					$($params)*
				}
			}
		}
	};

	{
		@impl_fn
		$self:ident
		$fn_name:ident
		$($params:ident)*
	} => {
		{
			#[allow(non_snake_case)]
			let ($($params,)*) = $self;
			let mut val = 0;
			$(val += $params.$fn_name();)*
			val
		}
	};
}

#[cfg(all(
	not(feature = "large-tuples"),
	not(feature = "omega-tuples-of-doom")
))]
tuple_impl! {
	T1 T2 T3 T4
	T5 T6 T7 T8
}

#[cfg(all(
	feature = "large-tuples",
	not(feature = "omega-tuples-of-doom")
))]
tuple_impl! {
	T1  T2  T3  T4
	T5  T6  T7  T8
	T9  T10 T11 T12
	T13 T14 T15 T16
	T17 T18 T19 T20
	T21 T22 T23 T24
	T25 T26 T27 T28
	T29 T30 T31 T32
}

#[cfg(feature = "omega-tuples-of-doom")]
tuple_impl! {
	T1 T2 T3 T4 T5 T6 T7 T8
	T9 T10 T11 T12 T13 T14 T15 T16
	T17 T18 T19 T20 T21 T22 T23 T24
	T25 T26 T27 T28 T29 T30 T31 T32
	T33 T34 T35 T36 T37 T38 T39 T40
	T41 T42 T43 T44 T45 T46 T47 T48
	T49 T50 T51 T52 T53 T54 T55 T56
	T57 T58 T59 T60 T61 T62 T63 T64
	T65 T66 T67 T68 T69 T70 T71 T72
	T73 T74 T75 T76 T77 T78 T79 T80
	T81 T82 T83 T84 T85 T86 T87 T88
	T89 T90 T91 T92 T93 T94 T95 T96
	T97 T98 T99 T100 T101 T102 T103 T104
	T105 T106 T107 T108 T109 T110 T111 T112
	T113 T114 T115 T116 T117 T118 T119 T120
	T121 T122 T123 T124 T125 T126 T127 T128
}

impl<T: MemoryUsage> MemoryUsage for [T] {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		self.iter()
			.map(T::mem_use_stack)
			.sum()
	}

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap)
			.sum()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_heap_excl_extra_capacity)
			.sum()
	}

	#[inline]
	fn mem_use(&self) -> usize {
		self.iter()
			.map(T::mem_use)
			.sum()
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		self.iter()
			.map(T::mem_use_excl_extra_capacity)
			.sum()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.iter_mut()
			.for_each(T::shrink_extra)
	}
}

impl<T: MemoryUsageStatic> MemoryUsageStatic for [T] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		let mut i = 0;
		let mut mem_use = 0;

		while i < self.len() {
			mem_use += self[i].mem_use_static();
			i += 1;
		}

		mem_use
	}
}

impl MemoryUsage for str {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		self.len()
	}

	mem_use_heap_zero_impl!();
}

impl MemoryUsageStatic for str {
	#[inline]
	fn mem_use_static(&self) -> usize {
		self.len()
	}
}

impl<T: MemoryUsage, const N: usize> MemoryUsage for [T; N] {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		<[T]>::mem_use_heap(self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use_heap_excl_extra_capacity(self)
	}

	#[inline]
	fn mem_use(&self) -> usize {
		<[T]>::mem_use(self)
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use_excl_extra_capacity(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		<[T]>::shrink_extra(self)
	}
}

impl<T: MemoryUsageStatic, const N: usize> MemoryUsageStatic for [T; N] {
	#[inline]
	fn mem_use_static(&self) -> usize {
		<[T]>::mem_use_static(self)
	}
}

impl<T: MemoryUsageConst, const N: usize> MemoryUsageConst for [T; N] {
	const MEM_USE_CONST: usize = T::MEM_USE_CONST * N;
}

impl<T: ?Sized + MemoryUsage> MemoryUsage for Box<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_heap(self)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_heap_excl_extra_capacity(self)
	}

	#[inline]
	fn mem_use(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use(self)
	}

	#[inline]
	fn mem_use_excl_extra_capacity(&self) -> usize {
		size_of::<Box<T>>() + T::mem_use_excl_extra_capacity(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(self)
	}
}

impl<T: MemoryUsage> MemoryUsage for Vec<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		let full = <[T]>::mem_use(self);
		let empty = (self.capacity() - self.len()) * size_of::<T>();

		full + empty
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		<[T]>::mem_use(self)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.iter_mut()
			.for_each(T::shrink_extra);
		self.shrink_to_fit();
	}
}

impl MemoryUsage for String {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.capacity()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.len()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.shrink_to_fit()
	}
}

impl<Idx: MemoryUsage> MemoryUsage for Range<Idx> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.start.mem_use_heap() + self.end.mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.start.mem_use_heap_excl_extra_capacity() + self.end.mem_use_heap_excl_extra_capacity()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.start.shrink_extra();
		self.end.shrink_extra();
	}
}

impl<Idx: MemoryUsage> MemoryUsage for RangeFrom<Idx> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.start.mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.start.mem_use_heap_excl_extra_capacity()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.start.shrink_extra()
	}
}

impl<Idx: MemoryUsage> MemoryUsage for RangeInclusive<Idx> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.start().mem_use_heap() + self.end().mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.start().mem_use_heap_excl_extra_capacity() + self.end().mem_use_heap_excl_extra_capacity()
	}

	// I don't think I can impl that shrinkie thing..
}

impl<Idx: MemoryUsage> MemoryUsage for RangeTo<Idx> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.end.mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.end.mem_use_heap_excl_extra_capacity()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.end.shrink_extra()
	}
}

impl<Idx: MemoryUsage> MemoryUsage for RangeToInclusive<Idx> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.end.mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.end.mem_use_heap_excl_extra_capacity()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.end.shrink_extra()
	}
}

impl<T: MemoryUsage> MemoryUsage for Bound<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		match self {
			Bound::Included(bound) => { bound.mem_use_heap() }
			Bound::Excluded(bound) => { bound.mem_use_heap() }
			Bound::Unbounded => { 0 }
		}
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		match self {
			Bound::Included(bound) => { bound.mem_use_heap_excl_extra_capacity() }
			Bound::Excluded(bound) => { bound.mem_use_heap_excl_extra_capacity() }
			Bound::Unbounded => { 0 }
		}
	}

	#[inline]
	fn shrink_extra(&mut self) {
		if let Bound::Included(bound) | Bound::Excluded(bound) = self {
			bound.shrink_extra()
		}
	}
}

impl<B: MemoryUsage, C: MemoryUsage> MemoryUsage for ControlFlow<B, C> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		match self {
			ControlFlow::Continue(cont) => { cont.mem_use_heap() }
			ControlFlow::Break(br) => { br.mem_use_heap() }
		}
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		match self {
			ControlFlow::Continue(cont) => { cont.mem_use_heap_excl_extra_capacity() }
			ControlFlow::Break(br) => { br.mem_use_heap_excl_extra_capacity() }
		}
	}

	#[inline]
	fn shrink_extra(&mut self) {
		match self {
			ControlFlow::Continue(cont) => { cont.shrink_extra() }
			ControlFlow::Break(br) => { br.shrink_extra() }
		}
	}
}

impl MemoryUsage for Path {
	mem_use_stack_zero_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.as_os_str().len()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.as_os_str().len()
	}
}

impl MemoryUsage for PathBuf {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.capacity()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.as_os_str().len()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.shrink_to_fit()
	}
}

impl<T: ?Sized + MemoryUsage> MemoryUsage for UnsafeCell<T> {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		// if `T` is `Sized`, this will probably be `size_of` impl, else it's
		// the unsized "stack" definition
		T::mem_use_stack(
			// SAFETY: we have shared access via `&self`
			unsafe { &*self.get() }
		)
	}
	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use_heap(
			// SAFETY: we have shared access via `&self`
			unsafe { &*self.get() }
		)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_heap_excl_extra_capacity(
			// SAFETY: we have shared access via `&self`
			unsafe { &*self.get() }
		)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(
			// SAFETY: we have exclusive access via `&mut self`
			unsafe { &mut *self.get() }
		)
	}
}

impl<T: ?Sized + MemoryUsage> MemoryUsage for Cell<T> {
	#[inline]
	fn mem_use_stack(&self) -> usize {
		// if `T` is `Sized`, this will probably be `size_of` impl, else it's
		// the unsized "stack" definition
		T::mem_use_stack(
			// SAFETY: `Cell<T>` has same memory layout as `T`
			unsafe { &*(self as *const Cell<T> as *const T) }
		)
	}

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use_heap(
			// SAFETY: `Cell<T>` has same memory layout as `T`
			unsafe { &*(self as *const Cell<T> as *const T) }
		)
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_heap_excl_extra_capacity(
			// SAFETY: `Cell<T>` has same memory layout as `T`
			unsafe { &*(self as *const Cell<T> as *const T) }
		)
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.get_mut().shrink_extra()
	}
}

impl<T, F> MemoryUsage for LazyCell<T, F>
where
	T: MemoryUsage,
	F: FnOnce() -> T
{
	// mem use is the same, initialised or not
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		// TODO: there doesn't seem to be a stable way to check/get uninitialised,
		// nor is there really a way to access the captured variables in closures
		// so we can really only force initialisation here
		let value_initialised = &**self;

		value_initialised.mem_use_heap()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		// TODO: see note in [`mem_use_heap`] about initialisation stuff
		let value_initialised = &**self;

		value_initialised.mem_use_heap_excl_extra_capacity()
	}

	// cannot implement shrink_extra, cannot get mut reference to inner
}

impl<T: MemoryUsage> MemoryUsage for OnceCell<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		match self.get() {
			Some(val) => { val.mem_use_heap() }
			None => { 0 }
		}
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		match self.get() {
			Some(val) => { val.mem_use_heap_excl_extra_capacity() }
			None => { 0 }
		}
	}

	#[inline]
	fn shrink_extra(&mut self) {
		if let Some(val) = self.get_mut() {
			val.shrink_extra()
		}
	}
}

// TODO: this should support `?Sized` but I dunno how to do it without relying
// on refcell internals
impl<T: MemoryUsage> MemoryUsage for RefCell<T> {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		T::mem_use_heap(&*self.borrow())
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		T::mem_use_heap_excl_extra_capacity(&*self.borrow())
	}

	#[inline]
	fn shrink_extra(&mut self) {
		T::shrink_extra(&mut *self.borrow_mut())
	}
}

impl MemoryUsage for OsStr {
	mem_use_stack_zero_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.len()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.len()
	}
}

impl MemoryUsage for OsString {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		self.capacity()
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		self.len()
	}

	#[inline]
	fn shrink_extra(&mut self) {
		self.shrink_to_fit()
	}
}

impl MemoryUsage for CStr {
	mem_use_stack_zero_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		// nul byte still uses space
		self.count_bytes() + 1
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		// nul byte still uses space
		self.count_bytes() + 1
	}
}

impl MemoryUsage for CString {
	mem_use_stack_size_of_impl!();

	#[inline]
	fn mem_use_heap(&self) -> usize {
		// nul byte still uses space
		self.count_bytes() + 1
	}

	#[inline]
	fn mem_use_heap_excl_extra_capacity(&self) -> usize {
		// nul byte still uses space
		self.count_bytes() + 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! test_stack_only_impl {
		{ $([$test_name:ident $type:ident $val:expr])* } => {
			$(
				#[test]
				fn $test_name() {
					// MemoryUsage
					assert_eq!(<$type>::mem_use_stack(&$val), size_of::<$type>());
					assert_eq!(<$type>::mem_use_heap(&$val), 0);
					assert_eq!(<$type>::mem_use_heap_excl_extra_capacity(&$val), 0);
					assert_eq!(<$type>::mem_use(&$val), size_of::<$type>());

					// MemoryUsageStatic
					assert_eq!(<$type>::mem_use_static(&$val), size_of::<$type>());

					// MemoryUsageConst
					assert_eq!(<$type>::MEM_USE_CONST, size_of::<$type>());
				}
			)*
		}
	}

	/// a
	type Unit = ();

	test_stack_only_impl! {
		[mem_use_unit Unit ()]
		[mem_use_bool bool false]
		[mem_use_char char '0']

		[mem_use_u8 u8 0]
		[mem_use_u16 u16 0]
		[mem_use_u32 u32 0]
		[mem_use_u64 u64 0]
		[mem_use_u128 u128 0]
		[mem_use_usize usize 0]

		[mem_use_i8 i8 0]
		[mem_use_i16 i16 0]
		[mem_use_i32 i32 0]
		[mem_use_i64 i64 0]
		[mem_use_i128 i128 0]
		[mem_use_isize isize 0]

		// [mem_use_f16 f16 0.0]
		[mem_use_f32 f32 0.0]
		[mem_use_f64 f64 0.0]
		// [mem_use_f128 f128 0.0]
	}

	#[test]
	fn mem_use_vec() {
		type TestVec = Vec<i32>;
		let mut vec = TestVec::new();
		let vec_size = size_of::<TestVec>();

		assert_eq!(vec.mem_use(), vec_size);
		assert_eq!(vec.mem_use_excl_extra_capacity(), vec_size);

		vec.reserve(32);

		let mem_use = vec.mem_use();
		let mem_excl = vec.mem_use_excl_extra_capacity();
		assert!(mem_use >= vec_size + (32 * i32::MEM_USE_CONST));
		assert_eq!(mem_excl, vec_size);

		// we allocated 32, then extended with 8, this should not reallocate (8 <= 32)

		vec.extend([1, 2, 3, 4, 5, 6, 7, 8]);
		assert_eq!(vec.mem_use(), mem_use);
		assert_eq!(vec.mem_use_excl_extra_capacity(), vec_size + (8 * i32::MEM_USE_CONST));
	}

	#[test]
	fn shrink_extra_vec() {
		type TestVec = Vec<Vec<i32>>;
		let mut vec = TestVec::from([
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1)
		]);

		assert!(vec.iter().all(|v| v.capacity() > 0), "test setup failed, for some reason");

		vec.shrink_extra();
		// I went in and read std's code, if shrinking
		// to 0, it'll just deallocate
		assert!(vec.iter().all(|v| v.capacity() == 0));
	}

	#[test]
	fn shrink_extra_slice() {
		type TestVec = Vec<Vec<i32>>;
		let mut vec = TestVec::from([
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1),
			Vec::with_capacity(1)
		]);

		let slice = &mut *vec;

		assert!(slice.iter().all(|v| v.capacity() > 0), "test setup failed, for some reason");

		slice.shrink_extra();
		// I went in and read std's code, if shrinking
		// to 0, it'll just deallocate
		assert!(slice.iter().all(|v| v.capacity() == 0));
	}

	#[test]
	fn shrink_extra_noop() {
		let mut value = 27478;
		let orig_value = value;
		value.shrink_extra();
		assert_eq!(value, orig_value);
		// ...
	}

	#[test]
	fn brief_tuple_impl_mem_use() {
		let tuple = (1u8, 2u32, 4u16, 8u8, 16u64);
		assert_eq!(tuple.mem_use(), 16);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), 16);
		assert_eq!(tuple.mem_use_stack(), 16);
		assert_eq!(tuple.mem_use_heap(), 0);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 0);

		let size_of_usize = size_of::<usize>();

		let tuple = (3usize, &[1u8, 2, 3, 4, 5]);
		assert_eq!(tuple.mem_use(), (size_of_usize * 2) + 5);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), (size_of_usize * 2) + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize * 2);
		assert_eq!(tuple.mem_use_heap(), 5);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);

		let tuple = (3usize, &[1u8, 2, 3, 4, 5] as &[u8]);
		assert_eq!(tuple.mem_use(), (size_of_usize * 3) + 5);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), (size_of_usize * 3) + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize * 3);
		assert_eq!(tuple.mem_use_heap(), 5);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);

		let tuple = (3usize, {
			let mut vec = Vec::with_capacity(8);
			vec.extend_from_slice(&[1u8, 2, 3, 4, 5]);
			vec
		});
		let vec_size = size_of::<Vec<u8>>();
		assert_eq!(tuple.mem_use(), size_of_usize + vec_size + 8);
		assert_eq!(tuple.mem_use_excl_extra_capacity(), size_of_usize + vec_size + 5);
		assert_eq!(tuple.mem_use_stack(), size_of_usize + vec_size);
		assert_eq!(tuple.mem_use_heap(), 8);
		assert_eq!(tuple.mem_use_heap_excl_extra_capacity(), 5);
	}
}
