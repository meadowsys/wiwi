use crate::prelude::*;

pub trait Input: Sized {
	type ConstSize<const N: usize>: Sized;
	type ConstSizeOwned<const N: usize>: Sized + 'static;

	fn len(&self) -> usize;

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	#[inline]
	fn starts_with<N>(&self, needle: &N) -> bool
	where
		N: Needle<Self>
	{
		needle.input_starts_with(self)
	}

	fn take_first(&self, i: usize) -> Option<(Self, Self)>;
	fn take_first_const<const N: usize>(&self) -> Option<(Self::ConstSize<N>, Self)>;
	fn take_first_const_owned<const N: usize>(&self) -> Option<(Self::ConstSizeOwned<N>, Self)>;
}

pub trait Needle<I>
where
	I: Input
{
	fn len(&self) -> usize;

	#[inline]
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn input_starts_with(&self, input: &I) -> bool;
}

impl<'h> Input for &'h [u8] {
	type ConstSize<const N: usize> = &'h [u8; N];
	type ConstSizeOwned<const N: usize> = [u8; N];

	#[inline]
	fn len(&self) -> usize {
		(**self).len()
	}

	#[inline]
	fn take_first(&self, i: usize) -> Option<(&'h [u8], &'h [u8])> {
		self.split_at_checked(i)
	}

	#[inline]
	fn take_first_const<const N: usize>(&self) -> Option<(Self::ConstSize<N>, Self)> {
		self.split_at_checked(N).map(|(output, remaining_input)| (
			// SAFETY: ptr derived from `output`, which is a slice of len `N`
			unsafe { &*output.as_ptr().cast() },
			remaining_input
		))
	}

	#[inline]
	fn take_first_const_owned<const N: usize>(&self) -> Option<(Self::ConstSizeOwned<N>, Self)> {
		self.take_first_const().map(|(output, remaining_input)| (*output, remaining_input))
	}
}

impl<'h> Needle<&'h [u8]> for &[u8] {
	#[inline]
	fn len(&self) -> usize {
		(**self).len()
	}

	#[inline]
	fn input_starts_with(&self, input: &&'h [u8]) -> bool {
		input.starts_with(self)
	}
}
