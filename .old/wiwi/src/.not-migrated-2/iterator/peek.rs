use crate::option::{ Option, Option::None };
use super::{ Iter, SizeHintBound, SizeHintImpl, SizeHintInner, SizeHintMarker };

pub trait Peekable: Iter {
	type PeekItem;
	fn peek(&mut self) -> Option<&Self::PeekItem>;
}

pub struct Peek<I, T> {
	iter: I,
	peeked: Option<Option<T>>
}

impl<I, T> Peek<I, T>
where
	I: Iter<Item = T>
{
	#[inline]
	pub(super) fn new(iter: I) -> Self {
		Self { iter, peeked: None }
	}

	#[inline]
	pub fn into_inner(self) -> (I, Option<Option<T>>) {
		let Self { iter, peeked } = self;
		(iter, peeked)
	}
}

impl<I, T> Iter for Peek<I, T>
where
	I: Iter<Item = T>
{
	type Item = I::Item;

	#[inline]
	fn next(&mut self) -> Option<I::Item> {
		self.peeked.take().unwrap_or_else(|| self.iter.next())
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		use SizeHintBound::*;
		use SizeHintInner::*;

		let peeked = self.peeked.is_some() as usize;

		match self.iter.size_hint().into_inner() {
			Unknown => unsafe { SizeHintImpl::unknown() }

			Upper { bound: Hard { count } } => unsafe { SizeHintImpl::upper_hard(count + peeked) }
			Upper { bound: Estimate { count } } => unsafe { SizeHintImpl::upper_estimate(count + peeked) }

			Lower { bound: Hard { count } } => unsafe { SizeHintImpl::lower_hard(count + peeked) }
			Lower { bound: Estimate { count } } => unsafe { SizeHintImpl::lower_estimate(count + peeked) }

			Single { bound: Hard { count } } => unsafe { SizeHintImpl::hard(count + peeked) }
			Single { bound: Estimate { count } } => unsafe { SizeHintImpl::estimate(count + peeked) }

			Range { lower: Estimate { count: cl }, upper: Estimate { count: cu } } => unsafe { SizeHintImpl::range_estimate(cl + peeked, cu + peeked) }
			Range { lower: Estimate { count: cl }, upper: Hard { count: cu } } => unsafe { SizeHintImpl::range_lestimate_uhard(cl + peeked, cu + peeked) }
			Range { lower: Hard { count: cl }, upper: Estimate { count: cu } } => unsafe { SizeHintImpl::range_lhard_uestimate(cl + peeked, cu + peeked) }
			Range { lower: Hard { count: cl }, upper: Hard { count: cu } } => unsafe { SizeHintImpl::range_hard(cl + peeked, cu + peeked) }
		}
	}
}

impl<I, T> Peekable for Peek<I, T>
where
	I: Iter<Item = T>,
{
	type PeekItem = I::Item;

	#[inline]
	fn peek(&mut self) -> Option<&I::Item> {
		self.peeked.get_or_insert_with(|| self.iter.next()).as_ref()
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::IntoIter;
	use super::*;

	#[test]
	fn peek() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.peekable();

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.peek(), Some(&1));
		assert_eq!(iter.peek(), Some(&1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some(1));

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.peek(), Some(&2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some(2));

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		// ... perhaps a bit excessive with the peeks?
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.peek(), Some(&3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some(3));

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.peek(), None);
		assert_eq!(iter.next(), None);
	}
}
