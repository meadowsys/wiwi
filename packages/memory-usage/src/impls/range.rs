use crate::{ MemoryUsage, MemoryUsageConst, MemoryUsageStatic };
use crate::macros::{
	impl_memory_usage,
	impl_memory_usage_only_const,
	impl_memory_usage_only_static,
	impl_memory_usage_size_of
};
use core::ops::{
	Range,
	RangeFrom,
	RangeFull,
	RangeInclusive,
	RangeTo,
	RangeToInclusive
};
#[cfg(feature = "nightly")]
use core::range::{
	Range as RangeNew,
	RangeFrom as RangeFromNew,
	// this is the same type as from core::ops, for some reason
	// RangeFull as RangeFullNew,
	RangeInclusive as RangeInclusiveNew,
	// RangeTo is same as RangeFull, for some reason
	// RangeTo as RangeToNew,
	RangeToInclusive as RangeToInclusiveNew
};

impl_memory_usage_size_of! {
	RangeFull
}

impl_memory_usage! {
	{ Idx: MemoryUsage }
	Range<Idx>
	|self| self.start.memory_usage() + self.end.memory_usage();
	compacted |self| self.start.memory_usage_compacted() + self.end.memory_usage_compacted();
	compact |self| self.start.compact_memory() + self.end.compact_memory();
	ref compacted |self| self.start.memory_usage_compacted_ref() + self.end.memory_usage_compacted_ref();
	ref compact |self| self.start.compact_memory_ref() + self.end.compact_memory_ref();
}

impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	Range<Idx>
	|self| self.start.memory_usage_const() + self.end.memory_usage_const();
}

impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	Range<Idx>
	=> Idx::MEMORY_USAGE * 2;
}

#[cfg(feature = "nightly")]
impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeNew<Idx>
	|self| self.start.memory_usage() + self.end.memory_usage();
	compacted |self| self.start.memory_usage_compacted() + self.end.memory_usage_compacted();
	compact |self| self.start.compact_memory() + self.end.compact_memory();
	ref compacted |self| self.start.memory_usage_compacted_ref() + self.end.memory_usage_compacted_ref();
	ref compact |self| self.start.compact_memory_ref() + self.end.compact_memory_ref();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeNew<Idx>
	|self| self.start.memory_usage_const() + self.end.memory_usage_const();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeNew<Idx>
	=> Idx::MEMORY_USAGE * 2;
}

impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeInclusive<Idx>
	|self| {
		let start = self.start().memory_usage();
		let end = self.end().memory_usage();
		// could just take size of bool, but that's not guaranteed to be there, so
		// roundabout calculating it instead
		let remaining = size_of::<RangeInclusive<Idx>>() - (2 * size_of::<Idx>());
		start + end + remaining
	};
	ref compacted |self| {
		let start = self.start().memory_usage_compacted_ref();
		let end = self.end().memory_usage_compacted_ref();
		let remaining = size_of::<RangeInclusive<Idx>>() - (2 * size_of::<Idx>());
		start + end + remaining
	};
	ref compact |self| self.start().compact_memory_ref() + self.end().compact_memory_ref();
}

impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeInclusive<Idx>
	|self| {
		let start = self.start().memory_usage_const();
		let end = self.end().memory_usage_const();
		// could just take size of bool, but that's not guaranteed to be there, so
		// roundabout calculating it instead
		let remaining = size_of::<RangeInclusive<Idx>>() - (2 * size_of::<Idx>());
		start + end + remaining
	};
}

impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeInclusive<Idx>
	=> {
		let start = Idx::MEMORY_USAGE;
		let end = Idx::MEMORY_USAGE;
		// could just take size of bool, but that's not guaranteed to be there, so
		// roundabout calculating it instead
		let remaining = size_of::<RangeInclusive<Idx>>() - (2 * size_of::<Idx>());
		start + end + remaining
	};
}

#[cfg(feature = "nightly")]
impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeInclusiveNew<Idx>
	|self| self.start.memory_usage() + self.last.memory_usage();
	compacted |self| self.start.memory_usage_compacted() + self.last.memory_usage_compacted();
	compact |self| self.start.compact_memory() + self.last.compact_memory();
	ref compacted |self| self.start.memory_usage_compacted_ref() + self.last.memory_usage_compacted_ref();
	ref compact |self| self.start.compact_memory_ref() + self.last.compact_memory_ref();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeInclusiveNew<Idx>
	|self| self.start.memory_usage_const() + self.last.memory_usage_const();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeInclusiveNew<Idx>
	=> Idx::MEMORY_USAGE * 2;
}

impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeFrom<Idx>
	|self| self.start.memory_usage();
	compacted |self| self.start.memory_usage_compacted();
	compact |self| self.start.compact_memory();
	ref compacted |self| self.start.memory_usage_compacted_ref();
	ref compact |self| self.start.compact_memory_ref();
}

impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeFrom<Idx>
	|self| self.start.memory_usage_const();
}

impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeFrom<Idx>
	=> Idx::MEMORY_USAGE;
}

#[cfg(feature = "nightly")]
impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeFromNew<Idx>
	|self| self.start.memory_usage();
	compacted |self| self.start.memory_usage_compacted();
	compact |self| self.start.compact_memory();
	ref compacted |self| self.start.memory_usage_compacted_ref();
	ref compact |self| self.start.compact_memory_ref();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeFromNew<Idx>
	|self| self.start.memory_usage_const();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeFromNew<Idx>
	=> Idx::MEMORY_USAGE;
}

impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeTo<Idx>
	|self| self.end.memory_usage();
	compacted |self| self.end.memory_usage_compacted();
	compact |self| self.end.compact_memory();
	ref compacted |self| self.end.memory_usage_compacted_ref();
	ref compact |self| self.end.compact_memory_ref();
}

impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeTo<Idx>
	|self| self.end.memory_usage_const();
}

impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeTo<Idx>
	=> Idx::MEMORY_USAGE;
}

impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeToInclusive<Idx>
	|self| self.end.memory_usage();
	compacted |self| self.end.memory_usage_compacted();
	compact |self| self.end.compact_memory();
	ref compacted |self| self.end.memory_usage_compacted_ref();
	ref compact |self| self.end.compact_memory_ref();
}

impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeToInclusive<Idx>
	|self| self.end.memory_usage_const();
}

impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeToInclusive<Idx>
	=> Idx::MEMORY_USAGE;
}

#[cfg(feature = "nightly")]
impl_memory_usage! {
	{ Idx: MemoryUsage }
	RangeToInclusiveNew<Idx>
	|self| self.last.memory_usage();
	compacted |self| self.last.memory_usage_compacted();
	compact |self| self.last.compact_memory();
	ref compacted |self| self.last.memory_usage_compacted_ref();
	ref compact |self| self.last.compact_memory_ref();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_const! {
	{ Idx: MemoryUsageConst }
	{ Idx: [const] MemoryUsageConst }
	RangeToInclusiveNew<Idx>
	|self| self.last.memory_usage_const();
}

#[cfg(feature = "nightly")]
impl_memory_usage_only_static! {
	{ Idx: MemoryUsageStatic }
	{ Idx: [const] MemoryUsageStatic }
	RangeToInclusiveNew<Idx>
	=> Idx::MEMORY_USAGE;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn range_full_usage() {
		let range: RangeFull = ..;

		assert_eq!(range.memory_usage(), 0);
		assert_eq!(range.memory_usage_const(), 0);
		assert_eq!(RangeFull::MEMORY_USAGE, 0);
	}

	#[test]
	fn range_usage() {
		let range: Range<usize> = 0..10;

		assert_eq!(range.memory_usage(), 16);
		assert_eq!(range.memory_usage_const(), 16);
		assert_eq!(<Range<usize>>::MEMORY_USAGE, 16);
	}

	#[cfg(feature = "nightly")]
	#[test]
	fn range_new_usage() {
		let range: RangeNew<usize> = (0..10).into();

		assert_eq!(range.memory_usage(), 16);
		assert_eq!(range.memory_usage_const(), 16);
		assert_eq!(<Range<usize>>::MEMORY_USAGE, 16);
	}

	#[test]
	fn range_inclusive_usage() {
		let range: RangeInclusive<usize> = 0..=10;

		assert_eq!(range.memory_usage(), 24);
		assert_eq!(range.memory_usage_const(), 24);
		assert_eq!(<RangeInclusive<usize>>::MEMORY_USAGE, 24);
	}

	#[cfg(feature = "nightly")]
	#[test]
	fn range_inclusive_new_usage() {
		let range: RangeInclusiveNew<usize> = (0..=10).into();

		assert_eq!(range.memory_usage(), 16);
		assert_eq!(range.memory_usage_const(), 16);
		assert_eq!(<RangeInclusiveNew<usize>>::MEMORY_USAGE, 16);
	}

	#[test]
	fn range_from_usage() {
		let range: RangeFrom<usize> = 0..;

		assert_eq!(range.memory_usage(), 8);
		assert_eq!(range.memory_usage_const(), 8);
		assert_eq!(<RangeFrom<usize>>::MEMORY_USAGE, 8);
	}

	#[cfg(feature = "nightly")]
	#[test]
	fn range_from_new_usage() {
		let range: RangeFromNew<usize> = (0..).into();

		assert_eq!(range.memory_usage(), 8);
		assert_eq!(range.memory_usage_const(), 8);
		assert_eq!(<RangeFrom<usize>>::MEMORY_USAGE, 8);
	}

	#[test]
	fn range_to_usage() {
		let range: RangeTo<usize> = ..10;

		assert_eq!(range.memory_usage(), 8);
		assert_eq!(range.memory_usage_const(), 8);
		assert_eq!(<RangeTo<usize>>::MEMORY_USAGE, 8);
	}

	#[test]
	fn range_to_inclusive_usage() {
		let range: RangeToInclusive<usize> = ..=10;

		assert_eq!(range.memory_usage(), 8);
		assert_eq!(range.memory_usage_const(), 8);
		assert_eq!(<RangeToInclusive<usize>>::MEMORY_USAGE, 8);
	}

	#[cfg(feature = "nightly")]
	#[test]
	fn range_to_inclusive_new_usage() {
		let range: RangeToInclusiveNew<usize> = (..=10).into();

		assert_eq!(range.memory_usage(), 8);
		assert_eq!(range.memory_usage_const(), 8);
		assert_eq!(<RangeToInclusiveNew<usize>>::MEMORY_USAGE, 8);
	}
}
