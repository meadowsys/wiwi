use crate::convert::Into;
use crate::option::Option;
use super::{ DoubleEndedIter, Iter, SizeHintImpl, SizeHintMarker };

pub struct Rev<I> {
	iter: I
}

impl<I: DoubleEndedIter> Rev<I> {
	#[inline]
	pub(super) fn new(iter: I) -> Self {
		Self { iter }
	}

	#[inline]
	pub fn into_inner(self) -> I {
		self.iter
	}
}

impl<I: DoubleEndedIter> Iter for Rev<I> {
	type Item = I::Item;

	#[inline]
	fn next(&mut self) -> Option<I::Item> {
		self.iter.next_back()
	}

	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.iter.size_hint().into()
	}
}

impl<I: DoubleEndedIter> DoubleEndedIter for Rev<I> {
	#[inline]
	fn next_back(&mut self) -> Option<I::Item> {
		self.iter.next()
	}
}
