use crate::clone::Clone;
use crate::memory::replace;
use crate::option::{ Option, Option::Some, Option::None };
use super::{ Iter, SizeHintBound, SizeHintImpl, SizeHintInner, SizeHintMarker };

pub struct RepeatPerItem<I: Iter> {
	iter: I,
	/// amount of times to emit each item
	count: usize,
	state: State<I::Item>
}

impl<I> RepeatPerItem<I>
where
	I: Iter,
	I::Item: Clone
{
	#[inline]
	pub(super) fn new(iter: I, count: usize) -> Self {
		let state = if count == 0 {
			// if it's 0, we just always return None, and don't touch inner iter
			State::Exhausted
		} else {
			State::None
		};

		Self { iter, count, state }
	}

	/// Consumes `self` and returns the underlying iter.
	#[inline]
	pub fn into_inner(self) -> (I, Option<(I::Item, usize)>) {
		let item = if let State::Some { item, remaining_count: count @ 1.. } = self.state {
			Some((item, count))
		} else {
			None
		};
		(self.iter, item)
	}
}

impl<I> Iter for RepeatPerItem<I>
where
	I: Iter,
	I::Item: Clone
{
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		let curr = replace(&mut self.state, State::None);
		let (res, next) = match curr {
			State::None => {
				if let Some(item) = self.iter.next() {
					(Some(item.clone()), State::Some {
						item,
						remaining_count: self.count - 1
					})
				} else {
					(None, State::Exhausted)
				}
			}
			State::Exhausted => {
				(None, State::Exhausted)
			}
			State::Some { item, remaining_count } => {
				match remaining_count {
					2.. => {
						(Some(item.clone()), State::Some { item, remaining_count: remaining_count - 1 })
					}
					1 => {
						(Some(item), State::None)
					}
					0 => {
						// special case for passing 1 in Iter::repeat_per_item call
						self.state = State::None;
						return self.next()
					}
				}
			}
		};

		self.state = next;
		res
	}

	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		use SizeHintBound::*;
		use SizeHintInner::*;

		let rem = if let State::Some { remaining_count, .. } = &self.state {
			*remaining_count
		} else {
			0
		};

		macro_rules! count {
			($count:ident) => { ($count * self.count) + rem }
		}

		match self.iter.size_hint().into_inner() {
			Unknown => unsafe { SizeHintImpl::unknown() }

			Upper { bound: Hard { count } } => unsafe { SizeHintImpl::upper_hard(count!(count)) }
			Upper { bound: Estimate { count } } => unsafe { SizeHintImpl::upper_estimate(count!(count)) }

			Lower { bound: Hard { count } } => unsafe { SizeHintImpl::lower_hard(count!(count)) }
			Lower { bound: Estimate { count } } => unsafe { SizeHintImpl::lower_estimate(count!(count)) }

			Single { bound: Hard { count } } => unsafe { SizeHintImpl::hard(count!(count)) }
			Single { bound: Estimate { count } } => unsafe { SizeHintImpl::estimate(count!(count)) }

			Range { lower: Estimate { count: cl }, upper: Estimate { count: cu } } => unsafe { SizeHintImpl::range_estimate(count!(cl), count!(cu)) }
			Range { lower: Estimate { count: cl }, upper: Hard { count: cu } } => unsafe { SizeHintImpl::range_lestimate_uhard(count!(cl), count!(cu)) }
			Range { lower: Hard { count: cl }, upper: Estimate { count: cu } } => unsafe { SizeHintImpl::range_lhard_uestimate(count!(cl), count!(cu)) }
			Range { lower: Hard { count: cl }, upper: Hard { count: cu } } => unsafe { SizeHintImpl::range_hard(count!(cl), count!(cu)) }
		}
	}
}

enum State<T> {
	/// Need to advance iter to get the next item
	None,
	/// The inner iter is exhausted
	Exhausted,
	/// We have an item here, and remaining iterations to do
	Some {
		item: T,
		remaining_count: usize
	}
}

#[cfg(test)]
mod tests {
	use crate::iter::{ IntoStdIterator, IntoWiwiIter };
	use super::*;

	#[test]
	fn repeat_per_item() {
		let vec = vec![1, 2, 3]
			.into_wiwi_iter()
			.map(|i| i * 9)
			.repeat_per_item(2)
			// TODO: use our own
			.convert_wiwi_into_std_iterator()
			.collect::<Vec<_>>();
		assert_eq!(vec, [9, 9, 18, 18, 27, 27]);
	}

	#[test]
	fn into_inner() {
		let iter = vec![1, 2, 3].into_wiwi_iter().repeat_per_item(2);

		let (iter, item) = iter.into_inner();
		// TODO: use our own
		let iter = iter.convert_wiwi_into_std_iterator().collect::<Vec<_>>();
		assert_eq!(item, None);
		assert_eq!(iter, [1, 2, 3]);

		let mut iter = vec![1, 2, 3].into_wiwi_iter().repeat_per_item(2);
		// 1
		let _ = iter.next();
		// 1
		let _ = iter.next();
		// 2
		let _ = iter.next();

		let (iter, item) = iter.into_inner();
		// TODO: use our own
		let iter = iter.convert_wiwi_into_std_iterator().collect::<Vec<_>>();
		assert_eq!(item, Some((2, 1)));
		assert_eq!(iter, [3]);
	}

	#[test]
	fn size_hint() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.repeat_per_item(2);

		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(6) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(5) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(4) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(3) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(2) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(1) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::hard(0) });
		assert_eq!(iter.next(), None);

		let mut iter = vec![1, 2, 3]
			.into_iter()
			.convert_std_into_wiwi_iter()
			.repeat_per_item(2);

		// same as above, but estimate only
		// (since there's an std iterator adapter in there)
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(6) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(5) });
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(4) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(3) });
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(2) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(1) });
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.size_hint(), unsafe { SizeHintImpl::estimate(0) });
		assert_eq!(iter.next(), None);
	}
}
