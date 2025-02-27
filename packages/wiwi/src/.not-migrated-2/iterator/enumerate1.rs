use crate::convert::Into;
use crate::option::Option;
use super::{ Enumerate, Iter, SizeHintImpl, SizeHintMarker };

#[repr(transparent)]
pub struct Enumerate1<I> {
	// just sets `inner.count` to 1 and then piggy backs off of it
	inner: Enumerate<I>
}

impl<I> Enumerate1<I>
where
	I: Iter
{
	#[inline]
	pub(super) fn new(iter: I) -> Self {
		let mut inner = Enumerate::new(iter);
		inner.count = 1;
		Self { inner }
	}
}

impl<I> Iter for Enumerate1<I>
where
	I: Iter
{
	type Item = (I::Item, usize);

	#[inline]
	fn next(&mut self) -> Option<(I::Item, usize)> {
		self.inner.next()
	}

	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.inner.size_hint().into()
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::IntoIter;
	use super::*;

	#[test]
	fn enumerate1() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.enumerate1();

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some((1, 1)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some((2, 2)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some((3, 3)));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.next(), None);
	}
}
