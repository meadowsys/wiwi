#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage {
	// currently the 3 branches matching logic is virtually the same except for
	// const and dyn keywords

	// normal impl always (for types that know they don't support const impl)
	{
		$(
			{ $($generics:tt)* }
			$({ $($const_generics:tt)* })?
		)?
		dyn $type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
		$(
			compacted |$self_compacted:ident| $impl_compacted:expr;
			compact |$self_compact:ident| $impl_compact:expr;
		)?
		$(
			ref compacted |$self_compacted_ref:ident| $impl_compacted_ref:expr;
			ref compact |$self_compact_ref:ident| $impl_compact_ref:expr;
		)?
	} => {
		impl$(<$($generics)*>)? $crate::MemoryUsage for $type
		$(where $($where)*)?
		{
			#[inline]
			fn memory_usage(&$self) -> ::core::primitive::usize {
				$impl
			}
		}

		impl$(<$($generics)*>)? $crate::MemoryCompaction for $type
		$(where $($where)*)?
		{
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

			$(
				#[inline]
				fn memory_usage_compacted_ref(&$self_compacted_ref) -> ::core::primitive::usize {
					$impl_compacted_ref
				}

				#[inline]
				fn compact_memory_ref(&$self_compact_ref) -> ::core::primitive::usize {
					$impl_compact_ref
				}
			)?
		}
	};

	// impl const always (with no generics or same generics for both const or not)
	// will error without nightly feature enabled
	{
		$(
			{ $($generics:tt)* }
		)?
		const $type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
		$(
			compacted |$self_compacted:ident| $impl_compacted:expr;
			compact |$self_compact:ident| $impl_compact:expr;
		)?
		$(
			ref compacted |$self_compacted_ref:ident| $impl_compacted_ref:expr;
			ref compact |$self_compact_ref:ident| $impl_compact_ref:expr;
		)?
	} => {
		const impl$(<$($generics)*>)? $crate::MemoryUsage for $type
		$(where $($where)*)?
		{
			#[inline]
			fn memory_usage(&$self) -> ::core::primitive::usize {
				$impl
			}
		}

		impl$(<$($generics)*>)? $crate::MemoryCompaction for $type
		$(where $($where)*)?
		{
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

			$(
				#[inline]
				fn memory_usage_compacted_ref(&$self_compacted_ref) -> ::core::primitive::usize {
					$impl_compacted_ref
				}

				#[inline]
				fn compact_memory_ref(&$self_compact_ref) -> ::core::primitive::usize {
					$impl_compact_ref
				}
			)?
		}
	};

	// impl const always (with special const generic bounds)
	{
		{ $($generics:tt)* }
		{ $($const_generics:tt)* }
		const $type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
		$(
			compacted |$self_compacted:ident| $impl_compacted:expr;
			compact |$self_compact:ident| $impl_compact:expr;
		)?
		$(
			ref compacted |$self_compacted_ref:ident| $impl_compacted_ref:expr;
			ref compact |$self_compact_ref:ident| $impl_compact_ref:expr;
		)?
	} => {
		const impl<$($const_generics)*> $crate::MemoryUsage for $type
		$(where $($where)*)?
		{
			#[inline]
			fn memory_usage(&$self) -> ::core::primitive::usize {
				$impl
			}
		}

		impl<$($generics)*> $crate::MemoryCompaction for $type
		$(where $($where)*)?
		{
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

			$(
				#[inline]
				fn memory_usage_compacted_ref(&$self_compacted_ref) -> ::core::primitive::usize {
					$impl_compacted_ref
				}

				#[inline]
				fn compact_memory_ref(&$self_compact_ref) -> ::core::primitive::usize {
					$impl_compact_ref
				}
			)?
		}
	};

	// impl const when nightly is enabled, normal impl otherwise
	{
		$(
			{ $($generics:tt)* }
			$({ $($const_generics:tt)* })?
		)?
		$type:ty
		$(where { $($where:tt)* })?
		|$self:ident| $impl:expr;
		$(
			compacted |$self_compacted:ident| $impl_compacted:expr;
			compact |$self_compact:ident| $impl_compact:expr;
		)?
		$(
			ref compacted |$self_compacted_ref:ident| $impl_compacted_ref:expr;
			ref compact |$self_compact_ref:ident| $impl_compact_ref:expr;
		)?
	} => {
		#[$crate::__macro_support::cfg_not_nightly]
		$crate::__macro_support::impl_memory_usage! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			dyn $type
			$(where { $($where)* })?
			|$self| $impl;
			$(
				compacted |$self_compacted| $impl_compacted;
				compact |$self_compact| $impl_compact;
			)?
			$(
				ref compacted |$self_compacted_ref| $impl_compacted_ref;
				ref compact |$self_compact_ref| $impl_compact_ref;
			)?
		}

		#[$crate::__macro_support::cfg_nightly]
		$crate::__macro_support::impl_memory_usage! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			const $type
			$(where { $($where)* })?
			|$self| $impl;
			$(
				compacted |$self_compacted| $impl_compacted;
				compact |$self_compact| $impl_compact;
			)?
			$(
				ref compacted |$self_compacted_ref| $impl_compacted_ref;
				ref compact |$self_compact_ref| $impl_compact_ref;
			)?
		}
	};
}
pub(crate) use impl_memory_usage;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_only_static {
	// todo we should not allow dyn impls at all (unless not nightly)
	// maybe achieve this by making dyn inner like @dyn $type
	// and mark it as not public api
	{
		$(
			{ $($generics:tt)* }
			$({ $($const_generics:tt)* })?
		)?
		dyn $type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		impl$(<$($generics)*>)? $crate::MemoryUsageStatic for $type
		$(where $($where)*)?
		{
			const MEMORY_USAGE: ::core::primitive::usize = $impl;
		}
	};

	{
		$(
			{ $($generics:tt)* }
		)?
		const $type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		const impl$(<$($generics)*>)? $crate::MemoryUsageStatic for $type
		$(where $($where)*)?
		{
			const MEMORY_USAGE: ::core::primitive::usize = $impl;
		}
	};

	{
		{ $($generics:tt)* }
		{ $($const_generics:tt)* }
		const $type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		const impl<$($const_generics)*> $crate::MemoryUsageStatic for $type
		$(where $($where)*)?
		{
			const MEMORY_USAGE: ::core::primitive::usize = $impl;
		}
	};

	{
		$(
			{ $($generics:tt)* }
			$({ $($const_generics:tt)* })?
		)?
		$type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		#[$crate::__macro_support::cfg_not_nightly]
		$crate::__macro_support::impl_memory_usage_only_static! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			dyn $type
			$(where { $($where)* })?
			=> $impl;
		}

		#[$crate::__macro_support::cfg_nightly]
		$crate::__macro_support::impl_memory_usage_only_static! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			const $type
			$(where { $($where)* })?
			=> $impl;
		}
	};
}
pub(crate) use impl_memory_usage_only_static;

#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_static {
	{
		$(
			{ $($generics:tt)* }
			$({ $($const_generics:tt)* })?
		)?
		$type:ty
		$(where { $($where:tt)* })?
		=> $impl:expr;
	} => {
		$crate::__macro_support::impl_memory_usage! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			$type
			$(where { $($where)* })?
			|self| $impl;
		}

		$crate::__macro_support::impl_memory_usage_only_static! {
			$(
				{ $($generics)* }
				$({ $($const_generics)* })?
			)?
			$type
			$(where { $($where)* })?
			=> $impl;
		}
	}
}
pub(crate) use impl_memory_usage_static;

/// Implement [`const MemoryUsage`] and [`MemoryUsageStatic`] for a type only
/// occupying the stack, using it's stack size as given by [`size_of`]
///
/// [`const MemoryUsage`]: crate::MemoryUsage
/// [`MemoryUsageStatic`]: crate::MemoryUsageStatic
#[cfg_attr(feature = "macros", macro_export)]
macro_rules! impl_memory_usage_size_of {
	{ $($ty:ty)* } => {
		$(
			$crate::__macro_support::impl_memory_usage_static! {
				$ty => size_of::<$ty>();
			}
		)*
	}
}
pub(crate) use impl_memory_usage_size_of;
