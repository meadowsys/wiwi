use crate::convert::Into;
use crate::option::{ Option, Option::Some };
use super::{ Iter, SizeHintImpl, SizeHintMarker };

pub struct Enumerate<I> {
	iter: I,
	// pub(super) is for enumerate1
	pub(super) count: usize
}

impl<I> Enumerate<I>
where
	I: Iter
{
	#[inline]
	pub(super) fn new(iter: I) -> Self {
		Self { iter, count: 0 }
	}
}

impl<I> Iter for Enumerate<I>
where
	I: Iter
{
	type Item = (I::Item, usize);

	#[inline]
	fn next(&mut self) -> Option<(I::Item, usize)> {
		let next = self.iter.next()?;
		let next_i = self.count;

		self.count += 1;

		Some((next, next_i))
	}

	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.iter.size_hint().into()
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::IntoIter;
	use super::*;

	#[test]
	fn enumerate() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.enumerate();

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some((1, 0)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some((2, 1)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some((3, 2)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.next(), None);
	}
}
