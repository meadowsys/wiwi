#![allow(
	stable_features,
	reason = "new_range_api, new_range_from_api, new_range_to_inclusive_api is not stable in msrv"
)]
#![cfg_attr(
	feature = "nightly",
	feature(
		const_trait_impl,
		f16,
		f128,
		new_range_api,
		new_range_from_api,
		new_range_to_inclusive_api
	)
)]
#![cfg_attr(
	not(feature = "std"),
	no_std
)]
#![deny(
	unconditional_recursion,
	clippy::unconditional_recursion
)]

mod macros;

/// Types that know their own memory usage
#[diagnostic::on_unimplemented(
	message = "Type doesn't know its memory usage"
)]
pub trait MemoryUsage {
	/// Get memory usage (including direct, indirect, and compactible memory)
	fn memory_usage(&self) -> usize;

	/// Get expected memory usage after compaction
	///
	/// Compaction means to remove unnecessary memory usage. This would include,
	/// for example, removing the unused capacity of a [`Vec`], by calling
	/// [`shrink_to_fit()`](Vec::shrink_to_fit).
	///
	/// Types like [`Vec`] that support memory compaction _should_ reimplement
	/// this function.
	#[inline]
	fn memory_usage_compacted(&self) -> usize {
		// default is delegate to ref version of this method, so if someone
		// implements ref, they get this one for free if they want it
		// otherwise, no-op
		self.memory_usage_compacted_ref()
	}

	/// Perform memory compaction, saving as much memory as possible,
	/// returning the amount of bytes saved
	#[inline]
	fn compact_memory(&mut self) -> usize {
		// default is delegate to ref version of this method, so if someone
		// implements ref, they get this one for free if they want it
		// otherwise, no-op
		self.compact_memory_ref()
	}

	/// Get expected memory usage after compaction behind a shared reference
	///
	/// This is useful for types with interior mutability. Types that have or
	/// could have (ie. with generics) should implement this method and
	/// [`compact_memory_ref()`](MemoryUsage::compact_memory_ref).
	/// For types that don't, the default implementation is a no-op.
	///
	/// See [`MemoryUsage::memory_usage_compacted`] for more details.
	#[inline]
	fn memory_usage_compacted_ref(&self) -> usize {
		// default is no-op, and no bytes saved
		self.memory_usage()
	}

	/// Perform memory compaction behind a shared reference, saving as much
	/// memory as possible, returning the amount of bytes saved
	///
	/// This is useful for types with interior mutability. Types that have or
	/// could have (ie. with generics) should implement this method and
	/// [`memory_usage_compacted_ref()`](MemoryUsage::memory_usage_compacted_ref).
	/// For types that don't, the default implementation is a no-op.
	///
	/// See [`MemoryUsage::compact_memory`] for more details.
	#[inline]
	fn compact_memory_ref(&self) -> usize {
		// default is no-op, and no bytes saved
		0
	}
}

// the "const" syntax is nightly too, so we can't just
// use a feature gate on normal code
macro_rules! const_trait {
	() => {
		/// Types that know their own memory usage at compile time
		#[diagnostic::on_unimplemented(
			message = "Type doesn't know its memory usage at compile time"
		)]
		pub trait MemoryUsageConst: MemoryUsage {
			fn memory_usage_const(&self) -> usize;
		}

		/// Types that always have the same memory usage, regardless of value
		/// (notably different than [`Sized`])
		#[diagnostic::on_unimplemented(
			message = "Type doesn't know its static memory usage"
		)]
		pub trait MemoryUsageStatic: MemoryUsageConst {
			const MEMORY_USAGE: usize;
		}
	};

	(const) => {
		/// Types that know their own memory usage at compile time
		#[diagnostic::on_unimplemented(
			message = "Type doesn't know its memory usage at compile time"
		)]
		pub const trait MemoryUsageConst: MemoryUsage {
			fn memory_usage_const(&self) -> usize;
		}

		/// Types that always have the same memory usage, regardless of value
		/// (notably different than [`Sized`])
		#[diagnostic::on_unimplemented(
			message = "Type doesn't know its static memory usage"
		)]
		pub const trait MemoryUsageStatic: MemoryUsageConst {
			const MEMORY_USAGE: usize;
		}
	};
}

#[cfg(not(feature = "nightly"))]
const_trait!();

#[cfg(feature = "nightly")]
const_trait!(const);

mod impls {
	// todo primitive types, fn (pointer), closure, tuple (to 16)

	mod array;
	mod bool;
	mod char;
	mod number;
	mod pointer;
	mod range;
	mod reference;
	mod slice;
	mod string;
	mod unit;
	mod vec;
}

#[doc(hidden)]
pub mod __macro_support {
	pub(crate) use crate::macros::{
		impl_memory_usage,
		impl_memory_usage_only_const,
		impl_memory_usage_const,
		impl_memory_usage_only_static,
		impl_memory_usage_static
	};
	pub use wiwi_macro_proc::{
		memory_usage_cfg_nightly as cfg_nightly,
		memory_usage_cfg_not_nightly as cfg_not_nightly
	};
}
