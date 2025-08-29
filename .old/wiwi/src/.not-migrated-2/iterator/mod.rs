use crate::clone::Clone;
use crate::function::FnMut;
use crate::macros::debug_assert;
use crate::memory::Sized;
use crate::option::{ Option, Option::Some };

mod adapter;
pub use adapter::{
	AsStdIterator,
	AsWiwiIter,
	IntoStdIterator,
	IntoWiwiIter,
	IterAdapter
};

mod double_ended;
pub use double_ended::DoubleEndedIter;

mod empty;
pub use empty::{ empty, Empty };

mod enumerate;
pub use enumerate::Enumerate;

mod enumerate1;
pub use enumerate1::Enumerate1;

mod from_fn;
pub use from_fn::{ from_fn, FromFn };

mod into_iter;
pub use into_iter::IntoIter;

mod map;
pub use map::Map;

mod peek;
pub use peek::{ Peekable, Peek };

mod repeat_per_item;
pub use repeat_per_item::RepeatPerItem;

mod rev;
pub use rev::Rev;

mod size_hint;
pub use size_hint::{ SizeHint, SizeHintBound, SizeHintImpl, SizeHintInner, SizeHintMarker };

// TODO: there's only Vec in there
// also call it rust_impl or something?
// mod std_impl;

mod tuple;
pub use tuple::*;

// this will compiler error if something broke `Iter`'s object safety
fn __iter_object_safe(_: &mut dyn Iter<Item = ()>) {}

pub trait Iter {
	type Item;

	fn next(&mut self) -> Option<Self::Item>;

	#[inline]
	fn size_hint(&self) -> SizeHint {
		// SAFETY: `unsafe` on the impl method is to make implementors assert
		// (for hard bounds) that it is implemented correctly. We have no invariants
		// to uphold
		let imp = unsafe { self.size_hint_impl(SizeHintMarker { _private: () }) };
		SizeHint { inner: imp.inner }
	}

	/// # Safety
	///
	/// Implementors must guarantee that the iter will adhere strictly to its
	/// returned size hint. This is important for hard bounds, where the user of
	/// the iter is allowed to rely on it for soundness reasons.
	#[inline]
	unsafe fn size_hint_impl(&self, _: SizeHintMarker) -> SizeHintImpl {
		// SAFETY: `unknown` is always correct
		unsafe { SizeHintImpl::unknown() }
	}

	#[inline]
	fn for_each<F>(mut self, mut f: F)
	where
		Self: Sized,
		F: FnMut(Self::Item)
	{
		while let Some(item) = self.next() {
			f(item)
		}
	}

	#[inline]
	fn map<O, F>(self, f: F) -> Map<Self, F>
	where
		Self: Sized,
		F: FnMut(Self::Item) -> O
	{
		Map::new(self, f)
	}

	/// Consumes the iter and returns the number of items that were emitted
	///
	/// This method will try to do as little work as possible, and won't advance
	/// the iter if it doesn't have to. If the iter's [`size_hint`](Iter::size_hint)
	/// returns a [`Single`](SizeHintInner::Single) [`Hard`](SizeHintBound::Hard)
	/// bound, this method can safely return that length, since it is in the
	/// contract of `size_hint` and hard bounds that the iter must return that
	/// many items. Otherwise, it will iterate through the entire iter, counting
	/// the number of iterations.
	///
	/// # Examples
	///
	/// TODO
	fn count(mut self) -> usize
	where
		Self: Sized
	{
		use SizeHintBound::*;
		use SizeHintInner::*;

		match self.size_hint().into_inner() {
			// iter asserts it would iterate exactly this many times
			Single { bound: Hard { count } } => { count }

			Range { lower: Hard { count: lower }, upper: Hard { count: upper } } => {
				for i in 0..lower {
					let item = self.next();
					debug_assert!(item.is_some(), "iter unsafely asserts to have at least {lower} items, but actually had {i} items");
				}

				let mut count = lower;
				while self.next().is_some() { count += 1 }
				debug_assert!(count <= upper, "iter unsafely asserts to have at most {upper} items, but actually had {count} items");

				count
			}

			Lower { bound: Hard { mut count } } | Range { lower: Hard { mut count }, .. }=> {
				for i in 0..count {
					let item = self.next();
					debug_assert!(item.is_some(), "iter unsafely asserts to have at least {count} items, but actually had {i} items");
				}

				while self.next().is_some() { count += 1 }
				count
			}

			Upper { bound: Hard { count: orig_count } } | Range { upper: Hard { count: orig_count }, .. } => {
				let mut count = 0;
				while self.next().is_some() { count += 1 }
				debug_assert!(count <= orig_count, "iter unsafely asserts to have at most {orig_count} items, but actually had {count} items");
				count
			}

			// generic impl
			Single { bound: Estimate { .. } }
				| Lower { bound: Estimate { .. } }
				| Upper { bound: Estimate { .. } }
				| Range { lower: Estimate { .. }, upper: Estimate { .. } }
				| Unknown
			=> {
				let mut count = 0;
				while self.next().is_some() { count += 1 }
				count
			}
		}
	}

	#[inline]
	fn enumerate(self) -> Enumerate<Self>
	where
		Self: Sized
	{
		Enumerate::new(self)
	}

	#[inline]
	fn enumerate1(self) -> Enumerate1<Self>
	where
		Self: Sized
	{
		Enumerate1::new(self)
	}

	#[inline]
	fn peekable(self) -> Peek<Self, Self::Item>
	where
		Self: Sized
	{
		Peek::new(self)
	}

	/// Takes every element emitted by the underlying iter, and returns it
	/// `count` times via cloning.
	///
	/// If the count provided is 1, elements may be needlessly cloned.
	//  side note: I'm purposefully not specifying the exact behaviour
	///
	/// If the count provided is 0, the iter will never be advanced, and the
	/// returned iter will yield no elements.
	//  another side note: this... feels less expensive than checking for 0 every
	//  iteration and does match the behaviour that one may expect of asking to
	//  repeat every item 0 times lol (if everything comes out 0 times... nothing
	//  comes out), and feels better to me than panicking. I mean, technically
	//  it's not wrong to ask repeat every element 0 times...
	///
	/// This iter is fused, meaning it will always yield `None`
	/// once the underlying iter does, and will never advance it again.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::iter::{ IntoWiwiIter, Iter };
	/// # // TODO: replace `convert_std_into_wiwi_iter` with native IntoIter method
	/// let mut iter = [1, 2, 3]
	///    .convert_std_into_wiwi_iter()
	///    .repeat_per_item(3);
	///
	/// assert_eq!(iter.next(), Some(1));
	/// assert_eq!(iter.next(), Some(1));
	/// assert_eq!(iter.next(), Some(1));
	///
	/// assert_eq!(iter.next(), Some(2));
	/// assert_eq!(iter.next(), Some(2));
	/// assert_eq!(iter.next(), Some(2));
	///
	/// assert_eq!(iter.next(), Some(3));
	/// assert_eq!(iter.next(), Some(3));
	/// assert_eq!(iter.next(), Some(3));
	///
	/// assert_eq!(iter.next(), None);
	/// ```
	///
	/// Passing in `0` for the count:
	///
	/// ```
	/// # use wiwi::iter::{ IntoWiwiIter, Iter };
	/// # // TODO: replace `convert_std_into_wiwi_iter` with native IntoIter method
	/// let mut iter = [1, 2, 3]
	///    .convert_std_into_wiwi_iter()
	///    .repeat_per_item(0);
	///
	/// // Never going to get anything out of it...
	/// assert_eq!(iter.next(), None);
	/// assert_eq!(iter.next(), None);
	/// assert_eq!(iter.next(), None);
	///
	/// // ...but the original iter is indeed intact
	/// let (mut orig_iter, item) = iter.into_inner();
	/// assert_eq!(orig_iter.next(), Some(1));
	/// assert_eq!(orig_iter.next(), Some(2));
	/// assert_eq!(orig_iter.next(), Some(3));
	/// assert_eq!(orig_iter.next(), None);
	/// assert_eq!(item, None);
	/// ```
	#[inline]
	fn repeat_per_item(self, count: usize) -> RepeatPerItem<Self>
	where
		Self: Sized,
		Self::Item: Clone
	{
		RepeatPerItem::new(self, count)
	}

	/*
	next_chunk
	last
	advance_by
	nth
	step_by
	chain
	zip
	intersperse
	intersperse_with
	map
	filter
	filter_map
	peekable
	skip_while
	take_while
	map_while
	skip
	take
	scan
	flat_map
	flatten
	map_windows
	fuse
	inspect
	by_ref
	collect
	try_collect
	collect_into
	partition
	partition_in_place
	is_partitioned
	try_fold
	try_for_each
	fold
	reduce
	try_reduce
	all
	any
	find
	find_map
	try_find
	position
	rposition
	max
	min
	max_by_key
	max_by
	min_by_key
	min_by
	rev
	unzip
	copied
	cloned
	cycle
	array_chunks
	sum
	product
	cmp
	cmp_by
	partial_cmp
	partial_cmp_by
	eq
	eq_by
	ne
	lt
	le
	gt
	ge
	is_sorted/by/key

	traits
	product, sum
	extend, from/intoiter
	double ended iter
	exact size iter
	iter
	fused iter, trusted len
	in place iterable
	trusted fused/step
	unchecked iter
	trusted/step

	fns
	empty
	from_fn
	once/with
	repeat/with
	successors
	zip
	from_coroutine
	repeat_n
	*/
}

impl<I: Iter> Iter for &mut I {
	type Item = I::Item;

	#[inline]
	fn next(&mut self) -> Option<I::Item> {
		(**self).next()
	}
}
