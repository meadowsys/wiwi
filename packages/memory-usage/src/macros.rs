#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage {
	{
		$({ $($generics:tt)* })? $type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
		$(
			compacted |$self_compacted:ident| $impl_compacted:expr;
			compact |$self_compact:ident| $impl_compact:expr;
		)?
	} => {
		impl$(<$($generics)*>)? $crate::MemoryUsage for $type
		$(where $($where)*)?
		{
			#[inline]
			fn memory_usage(&$self) -> ::core::primitive::usize {
				$impl
			}

			$(
				#[inline]
				fn memory_usage_compacted(&$self_compacted) -> ::core::primitive::usize {
					$impl_compacted
				}

				#[inline]
				fn compact_memory(&mut $self_compact) -> ::core::primitive::usize {
					$impl_compact
				}
			)?
		}
	}
}
pub(crate) use impl_memory_usage;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_only_const {
	{
		$(
			$({ $($generics:tt)* } { $($const_generics:tt)* })? $type:ty
			$(where { $($where:tt)* })?
			|$self:ident| $impl:expr;
		)*
	} => {
		$(
			#[$crate::__macro_support::cfg_not_nightly]
			impl$(<$($generics)*>)? $crate::MemoryUsageConst for $type
			$(where $($where)*)?
			{
				#[inline]
				fn memory_usage_const(&$self) -> ::core::primitive::usize {
					$impl
				}
			}

			#[$crate::__macro_support::cfg_nightly]
			$crate::__macro_support::impl_memory_usage_only_const! {
				const $({ $($const_generics)* })? $type
				$(where { $($where)* })?
				|$self| $impl;
			}
		)*
	};

	{
		const $({ $($generics:tt)* })? $type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
	} => {
		const impl$(<$($generics)*>)? $crate::MemoryUsageConst for $type
		$(where $($where)*)?
		{
			#[inline]
			fn memory_usage_const(&$self) -> ::core::primitive::usize {
				$impl
			}
		}
	};
}
pub(crate) use impl_memory_usage_only_const;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_const {
	{
		$(
			$({ $($generics:tt)* } { $($const_generics:tt)* })? $type:ty
			$(where { $($where:tt)* })?
			|$self:ident| $impl:expr;
		)*
	} => {
		$(
			$crate::__macro_support::impl_memory_usage! {
				$({ $($generics)* })? $type
				$(where { $($where)* })?
				|$self| $impl;
			}

			$crate::__macro_support::impl_memory_usage_only_const! {
				$({ $($generics)* } { $($const_generics)* })? $type
				$(where { $($where)* })?
				|$self| $impl;
			}
		)*
	};
}
pub(crate) use impl_memory_usage_const;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_only_static {
	{
		$(
			$({ $($generics:tt)* } { $($const_generics:tt)* })? $type:ty
			$(where { $($where:tt)* })?
			=> $impl:expr;
		)*
	} => {
		$(
			#[$crate::__macro_support::cfg_not_nightly]
			impl$(<$($generics)*>)? $crate::MemoryUsageStatic for $type
			$(where $($where)*)?
			{
				const MEMORY_USAGE: ::core::primitive::usize = $impl;
			}

			#[$crate::__macro_support::cfg_nightly]
			$crate::__macro_support::impl_memory_usage_only_static! {
				const $({ $($const_generics)* })? $type
				$(where { $($where)* })?
				=> $impl;
			}
		)*
	};

	{
		const $({ $($generics:tt)* })? $type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		const impl$(<$($generics)*>)? $crate::MemoryUsageStatic for $type
		$(where $($where)*)?
		{
			const MEMORY_USAGE: ::core::primitive::usize = $impl;
		}
	}
}
pub(crate) use impl_memory_usage_only_static;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_static {
	{
		$(
			$({ $($generics:tt)* } { $($const_generics:tt)* })? $type:ty
			$(where { $($where:tt)* })?
			=> $impl:expr;
		)*
	} => {
		$(
			$crate::__macro_support::impl_memory_usage_const! {
				$({ $($generics)* } { $($const_generics)* })? $type
				$(where { $($where)* })?
				|self| $impl;
			}

			$crate::__macro_support::impl_memory_usage_only_static! {
				$({ $($generics)* } { $($const_generics)* })? $type
				$(where { $($where)* })?
				=> $impl;
			}
		)*
	}
}
pub(crate) use impl_memory_usage_static;

/// Implement [`MemoryUsage`], [`MemoryUsageConst`], and [`MemoryUsageStatic`] for
/// a type only occupying the stack, using it's stack size as given by [`size_of`]
///
/// [`MemoryUsage`]: crate::MemoryUsage
/// [`MemoryUsageConst`]: crate::MemoryUsageConst
/// [`MemoryUsageStatic`]: crate::MemoryUsageStatic
#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_size_of {
	{ $($ty:ty)* } => {
		$crate::__macro_support::impl_memory_usage_static! {
			$($ty => size_of::<$ty>();)*
		}
	}
}
pub(crate) use impl_memory_usage_size_of;
