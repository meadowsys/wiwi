extern crate core;

use crate::function::FnMut;
use crate::memory::Sized;
use crate::option::Option;
use super::{ Iter, Rev };

pub trait DoubleEndedIter: Iter {
	fn next_back(&mut self) -> Option<Self::Item>;

	#[inline]
	fn rev(self) -> Rev<Self>
	where
		Self: Sized
	{
		Rev::new(self)
	}

	#[inline]
	fn for_each_back<F>(mut self, mut f: F)
	where
		Self: Sized,
		F: FnMut(Self::Item)
	{
		self.rev().for_each(f)
	}
}

impl<I: DoubleEndedIter> DoubleEndedIter for &mut I {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		(**self).next_back()
	}
}
