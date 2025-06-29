//! Implementations for slices/Vecs/arrays
//!
//! This implementation is quite inefficient right now, requiring 3 iterations.
//! However, specialised wrappers are available for specific types to use a
//! different, more efficient implementation for that specific slice type.
//!
//! RUST!!! SPECIALISATION WHEN!?!?!?

use crate::defer::OnDrop;
use super::internal_prelude::*;
use super::NumberSerialiserUnsigned;
use std::mem::MaybeUninit;
use std::ptr;

impl<T: Serialise> Serialise for [T] {
	type Serialiser<'h> = SliceSerialiser<'h, T> where Self: 'h;

	fn build_serialiser(&self) -> SliceSerialiser<'_, T> {
		SliceSerialiser::new(self)
	}
}
impl<T: Serialise, const N: usize> Serialise for [T; N] {
	type Serialiser<'h> = SliceSerialiser<'h, T> where Self: 'h;

	fn build_serialiser(&self) -> SliceSerialiser<'_, T> {
		SliceSerialiser::new(self)
	}
}

impl<T: Serialise> Serialise for Vec<T> {
	type Serialiser<'h> = SliceSerialiser<'h, T> where Self: 'h;

	fn build_serialiser(&self) -> SliceSerialiser<'_, T> {
		SliceSerialiser::new(self)
	}
}

pub struct SliceSerialiser<'h, T: Serialise + 'h> {
	inner: Vec<T::Serialiser<'h>>,
	/// If `inner.len() > u8::MAX`, this will be `Some`, containing
	/// the [`USizeSerialiser`] whose job is to serialise the length
	len_ser: Option<<usize as Serialise>::Serialiser<'h>>
}

impl<'h, T: Serialise> SliceSerialiser<'h, T> {
	fn new(slice: &'h [T]) -> Self {
		let len_ser = if slice.len() > u8::MAX.into_usize() {
			Some(NumberSerialiserUnsigned::new(slice.len()))
		} else {
			None
		};

		let inner = slice.iter()
			.map(|val| val.build_serialiser())
			.collect();

		Self { inner, len_ser }
	}
}

impl<'h, T: Serialise> Serialiser<'h> for SliceSerialiser<'h, T> {
	unsafe fn needed_capacity(&self) -> usize {
		let len_ser = if let Some(len_ser) = &self.len_ser {
			// marker + length serialised
			1 + len_ser.needed_capacity()
		} else {
			// marker + one byte for len
			2
		};

		let inner = self.inner.iter()
			.map(|val| val.needed_capacity())
			.sum::<usize>();

		len_ser + inner
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = &self.len_ser {
			buf.write_byte(MARKER_ARRAY_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_ARRAY_8);
			buf.write_byte(self.inner.len().into_u8_lossy());
		}

		for item in &self.inner {
			item.serialise(buf);
		}
	}
}

impl<'h, T: Deserialise<'h>, const N: usize> Deserialise<'h> for [T; N] {
	type Error = T::Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<[T; N], T::Error> {
		let len = match marker {
			MARKER_ARRAY_8 => {
				use_ok!(
					buf.read_byte(),
					val => val.into_usize(),
					#err err => err.expected(DESC_EXPECTED_ARRAY).wrap_foreign()
				)
			}
			MARKER_ARRAY_XL => {
				use_ok!(
					usize::deserialise(buf),
					#err err => err.expected(DESC_EXPECTED_ARRAY).wrap_foreign()
				)
			}
			_ => {
				return expected(DESC_EXPECTED_ARRAY)
					.found_something_else()
					.wrap_foreign()
			}
		};

		if len != N {
			return expected(DESC_EXPECTED_ARRAY_CONST)
				.found(DESC_FOUND_WRONG_SIZE_ARRAY)
				.wrap_foreign()
		}

		let arr = MaybeUninit::<[T; N]>::uninit();
		let count = 0;

		let mut dest = (arr, count).on_drop(|(mut arr, count)| {
			// drop what elements have been deserialised already
			// if we early exit for whatever reason
			let ptr = arr.as_mut_ptr().cast::<T>();
			for i in 0..count {
				let value = unsafe{ ptr::read(ptr.add(i)) };
				unsafe { drop(value) }
			}
		});
		let ptr = dest.0.as_mut_ptr().cast::<T>();

		for i in 0..N {
			let val = use_ok!(T::deserialise(buf));
			unsafe { ptr.add(i).write(val) }
			dest.1 = i;
		}

		unsafe { Ok(dest.into_inner().0.0.assume_init()) }
	}
}

impl<'h, T: Deserialise<'h>> Deserialise<'h> for Vec<T> {
	type Error = T::Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Vec<T>, T::Error> {
		let len = match marker {
			MARKER_ARRAY_8 => {
				use_ok!(
					buf.read_byte(),
					val => val.into_usize(),
					#err err => err.expected(DESC_EXPECTED_ARRAY).wrap_foreign()
				)
			}
			MARKER_ARRAY_XL => {
				use_ok!(
					usize::deserialise(buf),
					#err err => err.expected(DESC_EXPECTED_ARRAY).wrap_foreign()
				)
			}
			_ => {
				return expected(DESC_EXPECTED_ARRAY)
					.found_something_else()
					.wrap_foreign()
			}
		};

		let mut vec = Vec::<T>::with_capacity(len);
		let mut ptr = vec.as_mut_ptr();

		for _ in 0..len {
			unsafe { ptr.write(use_ok!(T::deserialise(buf))) }

			ptr = unsafe { ptr.add(1) };
			let len = vec.len();
			unsafe { vec.set_len(len + 1) }
		}

		Ok(vec)
	}
}

impl<'h, T: Deserialise<'h>> Deserialise<'h> for Box<[T]> {
	type Error = T::Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Box<[T]>, T::Error> {
		Ok(use_ok!(
			Vec::deserialise_with_marker(buf, marker),
			vec => vec.into_boxed_slice()
		))
	}
}
