use crate::convert::Into;
use crate::function::FnMut;
use crate::option::Option;
use super::{ Iter, SizeHintImpl, SizeHintMarker };

pub struct Map<I, F> {
	iter: I,
	f: F
}

impl<I, F, O> Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	#[inline]
	pub(super) fn new(iter: I, f: F) -> Self {
		Self { iter, f }
	}

	#[inline]
	pub fn into_inner(self) -> (I, F) {
		let Self { iter, f } = self;
		(iter, f)
	}
}

impl<I, F, O> Iter for Map<I, F>
where
	I: Iter,
	F: FnMut(I::Item) -> O
{
	type Item = O;

	#[inline]
	fn next(&mut self) -> Option<O> {
		self.iter.next().map(&mut self.f)
	}

	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		self.iter.size_hint().into()
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::{ IntoIter, IntoStdIterator };
	use super::*;

	#[test]
	fn map() {
		let vec = vec![1, 2, 3, 4, 5]
			.into_wiwi_iter()
			.map(|i| i + 8)
			.convert_wiwi_into_std_iterator()
			.collect::<Vec<_>>();
		assert_eq!(vec, [9, 10, 11, 12, 13]);
	}

	#[test]
	fn size_hint() {
		let iter = vec![1, 2, 3, 4, 5]
			.into_wiwi_iter();
		let hint = iter.size_hint();

		let iter = iter.map(|i| i + 8);
		let map_hint = iter.size_hint();
		assert_eq!(hint, map_hint);
		assert_eq!(hint, unsafe { SizeHintImpl::hard(5) });
	}
}
