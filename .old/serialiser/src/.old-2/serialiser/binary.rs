#![allow(
	dead_code,
	unused_imports,
	reason = "wip (TODO: remove me)"
)]

use crate::prelude::*;

mod int;

pub trait Serialise<'h> {
	type Serialiser: Serialiser<'h>;

	fn serialiser(&'h self) -> Self::Serialiser;
}

pub trait Serialiser<'h>: Sized {
	fn serialise<O>(&self, out: O)
	where
		O: Output;
}

pub trait Deserialise<'h>: Sized {
	type Error: std::error::Error + From<Error>;

	fn deserialise<I>(input: I) -> Result<Self, Self::Error>
	where
		I: Input<'h>;
}

pub trait Input<'h> {
	fn read_bytes(&mut self, bytes: usize) -> Option<&'h [u8]>;
}

pub trait Output {
	fn write_bytes(&mut self, bytes: &[u8]);
}

/// Error type but it's blank (it will be filled soon™)
pub struct Error {
	inner: ErrorInner
}

enum ErrorInner {
	/// Something™ happened
	Something
}

impl<'h, T> Serialise<'h> for &T
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}

impl<'h, T> Serialise<'h> for &mut T
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}

impl<'h, T> Serialise<'h> for Box<T>
where
	T: ?Sized + Serialise<'h>
{
	type Serialiser = T::Serialiser;

	#[inline]
	fn serialiser(&'h self) -> T::Serialiser {
		T::serialiser(self)
	}
}
