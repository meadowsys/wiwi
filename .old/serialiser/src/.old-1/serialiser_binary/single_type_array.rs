use super::internal_prelude::*;
use super::NumberSerialiserUnsigned;
use super::number::{
	get_byte_count_signed_le,
	get_byte_count_unsigned_le,
	get_marker_for_signed,
	get_marker_for_unsigned
};
use std::{ hint, slice };

#[repr(transparent)]
pub struct SingleTypeArray<T>(pub T);

impl<T> Serialise for SingleTypeArray<Vec<T>>
where
	T: SingleTypeArraySerialise
{
	type Serialiser<'h> = SingleTypeArraySerialiserImpl<'h, T> where T: 'h;

	fn build_serialiser(&self) -> SingleTypeArraySerialiserImpl<'_, T> {
		SingleTypeArraySerialiserImpl::new(&self.0)
	}
}

impl<'s, T> Serialise for SingleTypeArray<&'s [T]>
where
	T: SingleTypeArraySerialise + 's
{
	type Serialiser<'h> = SingleTypeArraySerialiserImpl<'h, T> where Self: 'h;

	fn build_serialiser(&self) -> SingleTypeArraySerialiserImpl<'_, T> {
		SingleTypeArraySerialiserImpl::new(&self.0)
	}
}

impl<T> Serialise for SingleTypeArray<Box<[T]>>
where
	T: SingleTypeArraySerialise
{
	type Serialiser<'h> = SingleTypeArraySerialiserImpl<'h, T> where T: 'h;

	fn build_serialiser(&self) -> SingleTypeArraySerialiserImpl<'_, T> {
		SingleTypeArraySerialiserImpl::new(&self.0)
	}
}

pub struct SingleTypeArraySerialiserImpl<'h, T>
where
	T: SingleTypeArraySerialise + 'h
{
	serialisers: Vec<T::Serialiser<'h>>,
	len_ser: Option<<usize as Serialise>::Serialiser<'h>>
}

impl<'h, T> SingleTypeArraySerialiserImpl<'h, T>
where
	T: SingleTypeArraySerialise + 'h
{
	fn new(slice: &'h [T]) -> Self {
		let len_ser = if slice.len() > u8::MAX.into_usize() {
			Some(NumberSerialiserUnsigned::new(slice.len()))
		} else {
			None
		};

		let serialisers = slice.iter()
			.map(|val| val.build_serialiser())
			.collect();

		Self { serialisers, len_ser }
	}
}

impl<'h, T> Serialiser<'h> for SingleTypeArraySerialiserImpl<'h, T>
where
	T: SingleTypeArraySerialise + 'h
{
	unsafe fn needed_capacity(&self) -> usize {
		let len = if let Some(len_ser) = &self.len_ser {
			len_ser.needed_capacity()
		} else {
			// just one u8 value
			1
		};

		let contents = self.serialisers.iter()
			.map(|ser| ser.needed_capacity())
			.sum::<usize>();

		// marker (1) + len + contents
		1 + len + contents
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = &self.len_ser {
			buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_SINGLE_TYPE_ARRAY_8);
			buf.write_byte(self.serialisers.len().into_u8_lossy());
		}

		buf.write_byte(T::marker());
		self.serialisers
			.iter()
			.for_each(|ser| ser.serialise(buf));
	}
}

pub trait SingleTypeArraySerialise {
	type Serialiser<'h>: SingleTypeArraySerialiser<'h> where Self: 'h;
	unsafe fn marker() -> u8;
	fn build_serialiser(&self) -> Self::Serialiser<'_>;
}

pub trait SingleTypeArraySerialiser<'h> {
	unsafe fn needed_capacity(&self) -> usize;
	unsafe fn serialise<O: Output>(&self, buf: &mut O);
}
