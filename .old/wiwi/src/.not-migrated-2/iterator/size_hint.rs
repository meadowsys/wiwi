use crate::convert::From;
use crate::option::{ Option, Option::Some, Option::None };
use super::Iter;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[repr(transparent)]
pub struct SizeHint {
	pub(super) inner: SizeHintInner
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[repr(transparent)]
pub struct SizeHintImpl {
	pub(super) inner: SizeHintInner
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum SizeHintInner {
	Unknown,
	Lower { bound: SizeHintBound },
	Upper { bound: SizeHintBound },
	Single { bound: SizeHintBound },
	Range { lower: SizeHintBound, upper: SizeHintBound }
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum SizeHintBound {
	Hard { count: usize },
	Estimate { count: usize }
}

/// Iter consumers call [`size_hint`](Iter::size_hint), not [`size_hint_impl`](Iter::size_hint_impl)!
///
/// Really only just for consistency sake.
pub struct SizeHintMarker {
	pub(super) _private: ()
}

impl SizeHint {
	#[inline]
	pub fn into_inner(self) -> SizeHintInner {
		self.inner
	}
}

impl SizeHintImpl {
	#[inline]
	unsafe fn with_inner(inner: SizeHintInner) -> Self {
		SizeHintImpl { inner }
	}

	#[inline]
	unsafe fn single(bound: SizeHintBound) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::with_inner(SizeHintInner::Single { bound }) }
	}

	#[inline]
	unsafe fn lower(bound: SizeHintBound) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::with_inner(SizeHintInner::Lower { bound }) }
	}

	#[inline]
	unsafe fn upper(bound: SizeHintBound) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::with_inner(SizeHintInner::Upper { bound }) }
	}

	#[inline]
	unsafe fn range(lower: SizeHintBound, upper: SizeHintBound) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::with_inner(SizeHintInner::Range { lower, upper }) }
	}

	#[inline]
	pub unsafe fn unknown() -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::with_inner(SizeHintInner::Unknown) }
	}

	#[inline]
	pub unsafe fn hard(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::single(SizeHintBound::Hard { count }) }
	}

	#[inline]
	pub unsafe fn estimate(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::single(SizeHintBound::Estimate { count }) }
	}

	#[inline]
	pub unsafe fn lower_hard(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::lower(SizeHintBound::Hard { count }) }
	}

	#[inline]
	pub unsafe fn lower_estimate(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::lower(SizeHintBound::Estimate { count }) }
	}

	#[inline]
	pub unsafe fn upper_hard(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::upper(SizeHintBound::Hard { count }) }
	}

	#[inline]
	pub unsafe fn upper_estimate(count: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::upper(SizeHintBound::Estimate { count }) }
	}

	#[inline]
	pub unsafe fn range_hard(lower: usize, upper: usize) -> Self {
		if lower == upper {
			// SAFETY: caller promises to uphold the invariants of this type of bound
			unsafe { Self::single(SizeHintBound::Hard { count: lower }) }
		} else {
			// SAFETY: caller promises to uphold the invariants of this type of bound
			unsafe { Self::range(SizeHintBound::Hard { count: lower }, SizeHintBound::Hard { count: upper }) }
		}
	}

	#[inline]
	pub unsafe fn range_estimate(lower: usize, upper: usize) -> Self {
		if lower == upper {
			// SAFETY: caller promises to uphold the invariants of this type of bound
			unsafe { Self::single(SizeHintBound::Estimate { count: lower }) }
		} else {
			// SAFETY: caller promises to uphold the invariants of this type of bound
			unsafe { Self::range(SizeHintBound::Estimate { count: lower }, SizeHintBound::Estimate { count: upper }) }
		}
	}

	#[inline]
	pub unsafe fn range_lhard_uestimate(lower: usize, upper: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::range(SizeHintBound::Hard { count: lower }, SizeHintBound::Estimate { count: upper }) }
	}

	#[inline]
	pub unsafe fn range_lestimate_uhard(lower: usize, upper: usize) -> Self {
		// SAFETY: caller promises to uphold the invariants of this type of bound
		unsafe { Self::range(SizeHintBound::Estimate { count: lower }, SizeHintBound::Hard { count: upper }) }
	}

	#[inline]
	pub fn into_inner(self) -> SizeHintInner {
		self.inner
	}
}

impl SizeHintInner {
	fn into_std(self) -> (usize, Option<usize>) {
		use SizeHintInner::*;
		use SizeHintBound::*;

		match self {
			Unknown => { (0, None) }
			Upper { bound: Hard { count } | Estimate { count } } => { (0, Some(count)) }
			Lower { bound: Hard { count } | Estimate { count } } => { (count, None) }
			Single { bound: Hard { count } | Estimate { count } } => { (count, Some(count)) }
			Range {
				lower: Estimate { count: cl } | Hard { count: cl },
				upper: Estimate { count: cu } | Hard { count: cu }
			} => { (cl, Some(cu)) }
		}
	}
}

#[cfg(test)]
impl PartialEq<SizeHintInner> for SizeHint {
	fn eq(&self, other: &SizeHintInner) -> bool {
		self.inner.eq(other)
	}
}

#[cfg(test)]
impl PartialEq<SizeHintInner> for SizeHintImpl {
	fn eq(&self, other: &SizeHintInner) -> bool {
		self.inner.eq(other)
	}
}

#[cfg(test)]
impl PartialEq<SizeHint> for SizeHintInner {
	fn eq(&self, other: &SizeHint) -> bool {
		self.eq(&other.inner)
	}
}

#[cfg(test)]
impl PartialEq<SizeHintImpl> for SizeHintInner {
	fn eq(&self, other: &SizeHintImpl) -> bool {
		self.eq(&other.inner)
	}
}

#[cfg(test)]
impl PartialEq<SizeHint> for SizeHintImpl {
	fn eq(&self, other: &SizeHint) -> bool {
		self.inner.eq(&other.inner)
	}
}

#[cfg(test)]
impl PartialEq<SizeHintImpl> for SizeHint {
	fn eq(&self, other: &SizeHintImpl) -> bool {
		self.inner.eq(&other.inner)
	}
}

impl From<(usize, Option<usize>)> for SizeHintImpl {
	#[inline]
	fn from(value: (usize, Option<usize>)) -> Self {
		let (lower, upper) = value;

		match upper {
			Some(upper) if lower == upper => unsafe { Self::estimate(lower) }
			Some(upper) => unsafe { Self::range_estimate(lower, upper) }
			None => unsafe { Self::lower_estimate(lower) }
		}
	}
}

impl From<SizeHint> for (usize, Option<usize>) {
	#[inline]
	fn from(value: SizeHint) -> Self {
		value.inner.into_std()
	}
}

impl From<SizeHint> for SizeHintImpl {
	#[inline]
	fn from(value: SizeHint) -> Self {
		Self { inner: value.inner }
	}
}
