use super::internal_prelude::*;
use super::NumberSerialiserUnsigned;
use std::borrow::Cow;
use std::str;

impl Serialise for str {
	type Serialiser<'h> = StrSerialiser<'h>;

	fn build_serialiser(&self) -> StrSerialiser<'_> {
		StrSerialiser::new(self)
	}
}

impl Serialise for String {
	type Serialiser<'h> = StrSerialiser<'h>;

	fn build_serialiser(&self) -> StrSerialiser<'_> {
		StrSerialiser::new(self)
	}
}

/// Serialiser for strings (including [`prim@str`], [`String`], [`Cow<str>`] etc)
pub struct StrSerialiser<'h> {
	/// The [`prim@str`] to serialise
	val: &'h str,

	/// If `val.len() > u8::MAX`, this will be `Some`, containing
	/// the [`USizeSerialiser`] whose job is to serialise the length
	len_ser: Option<<usize as Serialise>::Serialiser<'h>>
}

impl<'h> StrSerialiser<'h> {
	fn new(val: &'h str) -> Self {
		let len_ser = if val.len() > u8::MAX.into_usize() {
			Some(NumberSerialiserUnsigned::new(val.len()))
		} else {
			None
		};

		Self { val, len_ser }
	}
}

impl<'h> Serialiser<'h> for StrSerialiser<'h> {
	unsafe fn needed_capacity(&self) -> usize {
		let meta = if let Some(len_ser) = self.len_ser.as_ref() {
			// marker + length serialised
			1 + len_ser.needed_capacity()
		} else {
			// marker + one byte for len
			2
		};

		meta + self.val.len()
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = self.len_ser.as_ref() {
			buf.write_byte(MARKER_STR_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_STR_8);
			buf.write_byte(self.val.len().into_u8_lossy());
		}

		buf.write_bytes(self.val.as_bytes());
	}
}

impl<'h> Deserialise<'h> for &'h str {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<&'h str> {
		let len = match marker {
			MARKER_STR_8 => {
				use_ok!(
					buf.read_byte(),
					len => len.into_usize(),
					#err err => err.expected(DESC_EXPECTED_STR).wrap()
				)
			}
			MARKER_STR_XL => {
				use_ok!(
					usize::deserialise(buf),
					#err err => err.expected(DESC_EXPECTED_STR).wrap()
				)
			}
			_ => {
				return expected(DESC_EXPECTED_STR)
					.found_something_else()
					.wrap()
			}
		};

		let slice = use_ok!(
			buf.read_bytes(len),
			#err err => err.expected(DESC_EXPECTED_STR).wrap()
		);

		let str = use_ok!(
			str::from_utf8(slice),
			#err _err => expected(DESC_EXPECTED_STR)
				.found(DESC_FOUND_INVALID_UTF_8)
				.wrap()
		);

		Ok(str)
	}
}

impl<'h> Deserialise<'h> for String {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<String> {
		Ok(use_ok!(
			<&str>::deserialise_with_marker(buf, marker),
			str => String::from(str)
		))
	}
}

impl<'h> Deserialise<'h> for Cow<'h, str> {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Cow<'h, str>> {
		Ok(use_ok!(
			<&str>::deserialise_with_marker(buf, marker),
			str => Cow::Borrowed(str)
		))
	}
}
