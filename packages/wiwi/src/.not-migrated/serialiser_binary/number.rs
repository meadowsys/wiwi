use super::internal_prelude::*;
use std::{ hint, ptr };
use std::mem::MaybeUninit;

impl Serialise for u8 {
	type Serialiser<'h> = U8Serialiser;

	fn build_serialiser(&self) -> U8Serialiser {
		U8Serialiser::new(*self)
	}
}

pub struct U8Serialiser {
	byte: u8,
	needs_marker: bool
}

impl U8Serialiser {
	pub(super) fn new(val: u8) -> Self {
		Self {
			byte: val,
			needs_marker: val > MARKER_SMALLINT_RANGE_END
		}
	}
}

impl<'h> Serialiser<'h> for U8Serialiser {
	unsafe fn needed_capacity(&self) -> usize {
		self.needs_marker as usize + 1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.needs_marker { buf.write_byte(MARKER_U8) }
		buf.write_byte(self.byte)
	}
}

gen_int_deserialise! {
	u8
	unsigned_smallint_range
	DESC_EXPECTED_U8
	val => val
}

impl Serialise for u16 {
	type Serialiser<'h> = NumberSerialiserUnsigned<2>;

	fn build_serialiser(&self) -> NumberSerialiserUnsigned<2> {
		NumberSerialiserUnsigned::new(*self)
	}
}

gen_int_deserialise! {
	u16
	unsigned_smallint_range
	DESC_EXPECTED_U16
	val => val.into_u16()
}

impl Serialise for u32 {
	type Serialiser<'h> = NumberSerialiserUnsigned<4>;

	fn build_serialiser(&self) -> NumberSerialiserUnsigned<4> {
		NumberSerialiserUnsigned::new(*self)
	}
}

gen_int_deserialise! {
	u32
	unsigned_smallint_range
	DESC_EXPECTED_U32
	val => val.into_u32()
}

impl Serialise for u64 {
	type Serialiser<'h> = NumberSerialiserUnsigned<8>;

	fn build_serialiser(&self) -> NumberSerialiserUnsigned<8> {
		NumberSerialiserUnsigned::new(*self)
	}
}

gen_int_deserialise! {
	u64
	unsigned_smallint_range
	DESC_EXPECTED_U64
	val => val.into_u64()
}

impl Serialise for u128 {
	type Serialiser<'h> = NumberSerialiserUnsigned<16>;

	fn build_serialiser(&self) -> NumberSerialiserUnsigned<16> {
		NumberSerialiserUnsigned::new(*self)
	}
}

gen_int_deserialise! {
	u128
	unsigned_smallint_range
	DESC_EXPECTED_U128
	val => val.into_u128()
}

impl Serialise for usize {
	#[cfg(target_pointer_width = "64")]
	type Serialiser<'h> = NumberSerialiserUnsigned<8>;
	#[cfg(target_pointer_width = "32")]
	type Serialiser<'h> = NumberSerialiserUnsigned<4>;
	#[cfg(target_pointer_width = "16")]
	type Serialiser<'h> = NumberSerialiserUnsigned<2>;

	fn build_serialiser(&self) -> Self::Serialiser<'_> {
		NumberSerialiserUnsigned::new(*self)
	}
}

impl<'h> Deserialise<'h> for usize {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<usize> {
		#[cfg(target_pointer_width = "64")]
		let val = u64::deserialise_with_marker(buf, marker);
		#[cfg(target_pointer_width = "32")]
		let val = u32::deserialise_with_marker(buf, marker);
		#[cfg(target_pointer_width = "16")]
		let val = u16::deserialise_with_marker(buf, marker);

		Ok(use_ok!(
			val,
			val => val.into_usize(),
			#err err => err.expected(DESC_EXPECTED_USIZE).wrap()
		))
	}
}

impl Serialise for i8 {
	type Serialiser<'h> = I8Serialiser;

	fn build_serialiser(&self) -> I8Serialiser {
		I8Serialiser::new(*self)
	}
}

pub struct I8Serialiser {
	byte: u8,
	needs_marker: bool
}

impl I8Serialiser {
	pub(super) fn new(val: i8) -> Self {
		Self {
			byte: val.into_u8_lossy(),
			needs_marker: {
				let lower = val < MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy();
				let upper = val > MARKER_SMALLINT_RANGE_END.into_i8_lossy();
				lower || upper
			}
		}
	}
}

impl<'h> Serialiser<'h> for I8Serialiser {
	unsafe fn needed_capacity(&self) -> usize {
		self.needs_marker as usize + 1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if self.needs_marker { buf.write_byte(MARKER_I8) }
		buf.write_byte(self.byte)
	}
}

gen_int_deserialise! {
	i8
	signed_smallint_range
	DESC_EXPECTED_I8
	val => val.into_i8_lossy()
}

impl Serialise for i16 {
	type Serialiser<'h> = NumberSerialiserSigned<2>;

	fn build_serialiser(&self) -> NumberSerialiserSigned<2> {
		NumberSerialiserSigned::new(*self)
	}
}

gen_int_deserialise! {
	i16
	signed_smallint_range
	DESC_EXPECTED_I16
	val => val.into_i8_lossy().into_i16()
}

impl Serialise for i32 {
	type Serialiser<'h> = NumberSerialiserSigned<4>;

	fn build_serialiser(&self) -> NumberSerialiserSigned<4> {
		NumberSerialiserSigned::new(*self)
	}
}

gen_int_deserialise! {
	i32
	signed_smallint_range
	DESC_EXPECTED_I32
	val => val.into_i8_lossy().into_i32()
}

impl Serialise for i64 {
	type Serialiser<'h> = NumberSerialiserSigned<8>;

	fn build_serialiser(&self) -> NumberSerialiserSigned<8> {
		NumberSerialiserSigned::new(*self)
	}
}

gen_int_deserialise! {
	i64
	signed_smallint_range
	DESC_EXPECTED_I64
	val => val.into_i8_lossy().into_i64()
}

impl Serialise for i128 {
	type Serialiser<'h> = NumberSerialiserSigned<16>;

	fn build_serialiser(&self) -> NumberSerialiserSigned<16> {
		NumberSerialiserSigned::new(*self)
	}
}

gen_int_deserialise! {
	i128
	signed_smallint_range
	DESC_EXPECTED_I128
	val => val.into_i8_lossy().into_i128()
}

impl Serialise for isize {
	#[cfg(target_pointer_width = "64")]
	type Serialiser<'h> = NumberSerialiserSigned<8>;
	#[cfg(target_pointer_width = "32")]
	type Serialiser<'h> = NumberSerialiserSigned<4>;
	#[cfg(target_pointer_width = "16")]
	type Serialiser<'h> = NumberSerialiserSigned<2>;

	fn build_serialiser(&self) -> Self::Serialiser<'_> {
		NumberSerialiserSigned::new(*self)
	}
}

impl<'h> Deserialise<'h> for isize {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<isize> {
		#[cfg(target_pointer_width = "64")]
		let val = i64::deserialise_with_marker(buf, marker);
		#[cfg(target_pointer_width = "32")]
		let val = i32::deserialise_with_marker(buf, marker);
		#[cfg(target_pointer_width = "16")]
		let val = i16::deserialise_with_marker(buf, marker);

		Ok(use_ok!(
			val,
			val => val.into_isize(),
			#err err => err.expected(DESC_EXPECTED_ISIZE).wrap()
		))
	}
}

impl Serialise for f32 {
	type Serialiser<'h> = NumberSerialiserFloat<4, MARKER_F32>;

	fn build_serialiser(&self) -> NumberSerialiserFloat<4, MARKER_F32> {
		NumberSerialiserFloat::new(*self)
	}
}

impl<'h> Deserialise<'h> for f32 {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<f32> {
		match marker {
			MARKER_F32 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f32::from_le_bytes(*bytes),
					#err err => err.expected(DESC_EXPECTED_F32).wrap()
				))
			}
			_ => { expected(DESC_EXPECTED_F32).found_something_else().wrap() }
		}
	}
}

impl Serialise for f64 {
	type Serialiser<'h> = NumberSerialiserFloat<8, MARKER_F64>;

	fn build_serialiser(&self) -> NumberSerialiserFloat<8, MARKER_F64> {
		NumberSerialiserFloat::new(*self)
	}
}

impl<'h> Deserialise<'h> for f64 {
	type Error = Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<f64> {
		match marker {
			MARKER_F32 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f32::from_le_bytes(*bytes).into_f64(),
					#err err => err.expected(DESC_EXPECTED_F32).wrap()
				))
			}
			MARKER_F64 => {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => f64::from_le_bytes(*bytes),
					#err err => err.expected(DESC_EXPECTED_F64).wrap()
				))
			}
			_ => { expected(DESC_EXPECTED_F64).found_something_else().wrap() }
		}
	}
}

pub struct NumberSerialiserUnsigned<const N: usize> {
	le_bytes: [u8; N],
	byte_count: u8
}

impl<const N: usize> NumberSerialiserUnsigned<N> {
	pub(super) fn new<T>(val: T) -> Self
	where
		T: IntUnsigned + ArrayConversions<N> + FromU8Lossless + Ord
	{
		let le_bytes = val.clone().into_le_bytes();
		let byte_count = if val <= T::from_u8(MARKER_SMALLINT_RANGE_END) {
			0
		} else {
			unsafe { get_byte_count_unsigned_le(le_bytes) }
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h, const N: usize> Serialiser<'h> for NumberSerialiserUnsigned<N> {
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		let byte_count = self.byte_count.into_usize();
		if byte_count > N { hint::unreachable_unchecked() }

		if byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			buf.write_byte(get_marker_for_unsigned(self.byte_count));
			buf.write_bytes(&self.le_bytes[..byte_count])
		}
	}
}

pub struct NumberSerialiserSigned<const N: usize> {
	le_bytes: [u8; N],
	byte_count: u8
}

impl<const N: usize> NumberSerialiserSigned<N> {
	pub(super) fn new<T>(val: T) -> Self
	where
		T: IntSigned + ArrayConversions<N> + FromI8Lossless + Ord
	{
		let le_bytes = val.clone().into_le_bytes();
		let byte_count = {
			let lower = val >= T::from_i8(MARKER_SMALLINT_NEGATIVE_RANGE_START.into_i8_lossy());
			let upper = val <= T::from_i8(MARKER_SMALLINT_RANGE_END.into_i8_lossy());

			if lower && upper {
				0
			} else {
				unsafe { get_byte_count_signed_le(le_bytes) }
			}
		};

		Self { le_bytes, byte_count }
	}
}

impl<'h, const N: usize> Serialiser<'h> for NumberSerialiserSigned<N> {
	unsafe fn needed_capacity(&self) -> usize {
		self.byte_count.into_usize() + 1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		let byte_count = self.byte_count.into_usize();
		if byte_count > N { hint::unreachable_unchecked() }

		if byte_count == 0 {
			buf.write_byte(self.le_bytes[0]);
		} else {
			buf.write_byte(get_marker_for_signed(self.byte_count));
			buf.write_bytes(&self.le_bytes[..byte_count]);
		}
	}
}

pub struct NumberSerialiserFloat<const N: usize, const MARKER: u8> {
	le_bytes: [u8; N],
}

impl<const N: usize, const MARKER: u8> NumberSerialiserFloat<N, MARKER> {
	pub(super) fn new<T>(val: T) -> Self
	where
		T: ArrayConversions<N>
	{
		Self { le_bytes: val.into_le_bytes() }
	}
}

impl<'h, const N: usize, const MARKER: u8> Serialiser<'h> for NumberSerialiserFloat<N, MARKER> {
	unsafe fn needed_capacity(&self) -> usize {
		N + 1
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		buf.write_byte(MARKER);
		buf.write_bytes(&self.le_bytes);
	}
}

pub(super) const unsafe fn get_marker_for_unsigned(byte_count: u8) -> u8 {
	((byte_count - 1) << 1) | 0x80
}

pub(super) const unsafe fn get_marker_for_signed(byte_count: u8) -> u8 {
	((byte_count - 1) << 1) | 0x81
}

pub(super) unsafe fn get_byte_count_unsigned_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	let ptr = bytes.as_ptr();

	for i in (1..BYTES).rev() {
		// simple! just return the first byte (including the byte) where
		// its not all 0s. Iter stops after offset 1 because we'll always
		// return at least 1 byte, so no point in checking last byte.

		// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
		// within `0..BYTES`, so this offset is in bounds and valid for reads
		let ptr = unsafe { ptr.add(i) };

		// SAFETY: as established above `ptr` is valid for reads
		let byte = unsafe { *ptr };
		if byte != 0 { return (i + 1) as _ }
	}

	// everything is empty? just return the minimum
	1
}

pub(super) unsafe fn get_byte_count_signed_le<const BYTES: usize>(bytes: [u8; BYTES]) -> u8 {
	debug_assert!(BYTES > 0);

	let ptr = bytes.as_ptr();

	let sign_bit = {
		// SAFETY: `ptr` is to an array of size `BYTES`, where `BYTES` is gt 0,
		// so offsetting ptr by `BYTES - 1` is in bounds and valid for reads
		let ptr = unsafe { ptr.add(BYTES - 1) };

		// SAFETY: `ptr` is valid for reads as established above
		let byte = unsafe { *ptr };

		byte >> 7
	};

	// byte that has empty data and can (probably) be safely discarded.
	// if negative, all bits 1, if positive, all bits 0
	let empty_byte = if sign_bit == 0 { 0 } else { u8::MAX };

	for i in (0..BYTES).rev() {
		let byte = {
			// SAFETY: `ptr` is to an array of size `BYTES` and `i` from loop is
			// within `0..BYTES`, so this offset is in bounds and valid for reads
			let ptr = unsafe { ptr.add(i) };

			// SAFETY: as established above this offset ptr is valid for reads
			unsafe { *ptr }
		};

		// the following could be shortened to a one liner...
		// but this absolutely sucks for readability/maintainability, so nah
		// if byte != empty_byte { return (i + 1) as u8 + (byte >> 7 != sign_bit) as u8 }

		if byte == empty_byte {
			// byte is full of 1s if negative, or full of 0s if positive
			// this byte can (probably) be safely discarded; continue
		} else if byte >> 7 == sign_bit {
			// sign bit is the same, return up to / including this byte
			// iter range is 0 to BYTES - 1 (inclusive), so this return range
			// will be 1 to BYTES (inclusive), which is correct
			return (i + 1).into_u8_lossy()
		} else {
			// sign bit is different, return this byte and one more after it.
			// if the next byte would have the wrong sign, it would have returned
			// already in the previous branch. This won't ever overflow because
			// the first byte will not have a different sign (as... itself),
			// so will never reach here
			return (i + 2).into_u8_lossy()
		}
	}

	// everything is empty? just return the minimum
	1
}

macro_rules! gen_int_deserialise {
	{
		$int:ident
		$smallint_macro:ident
		$expected:ident
		$smallint:ident => $smallint_cast:expr
	} => {
		impl<'h> Deserialise<'h> for $int {
			type Error = Error;

			fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<$int> {
				match marker {
					$smallint @ $smallint_macro!() => { Ok($smallint_cast) }

					MARKER_U8 => { int_try_into(read_u8(buf), $expected) }
					MARKER_I8 => { int_try_into(read_i8(buf), $expected) }

					MARKER_U16 => { int_try_into(read_u16(buf), $expected) }
					MARKER_I16 => { int_try_into(read_i16(buf), $expected) }

					MARKER_U24 => { int_try_into(read_u24_to_u32(buf), $expected) }
					MARKER_I24 => { int_try_into(read_i24_to_i32(buf), $expected) }

					MARKER_U32 => { int_try_into(read_u32(buf), $expected) }
					MARKER_I32 => { int_try_into(read_i32(buf), $expected) }

					MARKER_U40 => { int_try_into(read_u40_to_u64(buf), $expected) }
					MARKER_I40 => { int_try_into(read_i40_to_i64(buf), $expected) }

					MARKER_U48 => { int_try_into(read_u48_to_u64(buf), $expected) }
					MARKER_I48 => { int_try_into(read_i48_to_i64(buf), $expected) }

					MARKER_U56 => { int_try_into(read_u56_to_u64(buf), $expected) }
					MARKER_I56 => { int_try_into(read_i56_to_i64(buf), $expected) }

					MARKER_U64 => { int_try_into(read_u64(buf), $expected) }
					MARKER_I64 => { int_try_into(read_i64(buf), $expected) }

					MARKER_U72 => { int_try_into(read_u72_to_u128(buf), $expected) }
					MARKER_I72 => { int_try_into(read_i72_to_i128(buf), $expected) }

					MARKER_U80 => { int_try_into(read_u80_to_u128(buf), $expected) }
					MARKER_I80 => { int_try_into(read_i80_to_i128(buf), $expected) }

					MARKER_U88 => { int_try_into(read_u88_to_u128(buf), $expected) }
					MARKER_I88 => { int_try_into(read_i88_to_i128(buf), $expected) }

					MARKER_U96 => { int_try_into(read_u96_to_u128(buf), $expected) }
					MARKER_I96 => { int_try_into(read_i96_to_i128(buf), $expected) }

					MARKER_U104 => { int_try_into(read_u104_to_u128(buf), $expected) }
					MARKER_I104 => { int_try_into(read_i104_to_i128(buf), $expected) }

					MARKER_U112 => { int_try_into(read_u112_to_u128(buf), $expected) }
					MARKER_I112 => { int_try_into(read_i112_to_i128(buf), $expected) }

					MARKER_U120 => { int_try_into(read_u120_to_u128(buf), $expected) }
					MARKER_I120 => { int_try_into(read_i120_to_i128(buf), $expected) }

					MARKER_U128 => { int_try_into(read_u128(buf), $expected) }
					MARKER_I128 => { int_try_into(read_i128(buf), $expected) }

					_ => { expected($expected).found_something_else().wrap() }
				}
			}
		}
	}
}
use gen_int_deserialise;

macro_rules! read_int_fn_impl {
	{ $(
		$fn_name:ident $int:ident $expected:ident
	)* } => {
		$(
			fn $fn_name<'h, I: Input<'h>>(buf: &mut I) -> Result<$int> {
				Ok(use_ok!(
					buf.read_bytes_const(),
					bytes => $int::from_le_bytes(*bytes),
					#err err => err.expected($expected).wrap()
				))
			}
		)*
	}
}

read_int_fn_impl! {
	read_u8 u8 DESC_EXPECTED_U8
	read_i8 i8 DESC_EXPECTED_I8

	read_u16 u16 DESC_EXPECTED_U16
	read_i16 i16 DESC_EXPECTED_I16

	read_u32 u32 DESC_EXPECTED_U32
	read_i32 i32 DESC_EXPECTED_I32

	read_u64 u64 DESC_EXPECTED_U64
	read_i64 i64 DESC_EXPECTED_I64

	read_u128 u128 DESC_EXPECTED_U128
	read_i128 i128 DESC_EXPECTED_I128
}

macro_rules! read_int_extending_fn_impl {
	{ $(
		$fn_name:ident $in_bytes:literal $extend_fn:ident $target_int:ident $expected:ident
	)* } => {
		$(
			fn $fn_name<'h, I: Input<'h>>(buf: &mut I) -> Result<$target_int> {
				Ok(use_ok!(
					buf.read_bytes_const::<$in_bytes>(),
					bytes => $target_int::from_le_bytes(unsafe { $extend_fn(*bytes) }),
					#err err => err.expected($expected).wrap()
				))
			}
		)*
	}
}

read_int_extending_fn_impl! {
	read_u24_to_u32 3 zero_extend_array_le u32 DESC_EXPECTED_U24
	read_i24_to_i32 3 sign_extend_array_le i32 DESC_EXPECTED_I24

	read_u40_to_u64 5 zero_extend_array_le u64 DESC_EXPECTED_U40
	read_i40_to_i64 5 sign_extend_array_le i64 DESC_EXPECTED_I40

	read_u48_to_u64 6 zero_extend_array_le u64 DESC_EXPECTED_U48
	read_i48_to_i64 6 sign_extend_array_le i64 DESC_EXPECTED_I48

	read_u56_to_u64 7 zero_extend_array_le u64 DESC_EXPECTED_U56
	read_i56_to_i64 7 sign_extend_array_le i64 DESC_EXPECTED_I56

	read_u72_to_u128 9 zero_extend_array_le u128 DESC_EXPECTED_U72
	read_i72_to_i128 9 sign_extend_array_le i128 DESC_EXPECTED_I72

	read_u80_to_u128 10 zero_extend_array_le u128 DESC_EXPECTED_U80
	read_i80_to_i128 10 sign_extend_array_le i128 DESC_EXPECTED_I80

	read_u88_to_u128 11 zero_extend_array_le u128 DESC_EXPECTED_U88
	read_i88_to_i128 11 sign_extend_array_le i128 DESC_EXPECTED_I88

	read_u96_to_u128 12 zero_extend_array_le u128 DESC_EXPECTED_U96
	read_i96_to_i128 12 sign_extend_array_le i128 DESC_EXPECTED_I96

	read_u104_to_u128 13 zero_extend_array_le u128 DESC_EXPECTED_U104
	read_i104_to_i128 13 sign_extend_array_le i128 DESC_EXPECTED_I104

	read_u112_to_u128 14 zero_extend_array_le u128 DESC_EXPECTED_U112
	read_i112_to_i128 14 sign_extend_array_le i128 DESC_EXPECTED_I112

	read_u120_to_u128 15 zero_extend_array_le u128 DESC_EXPECTED_U120
	read_i120_to_i128 15 sign_extend_array_le i128 DESC_EXPECTED_I120
}

fn int_try_into<Into, From: TryInto<Into>>(
	from: Result<From>,
	expected_msg: &'static str
) -> Result<Into> {
	Ok(use_ok!(
		use_ok!(from).try_into(),
		#err _err => expected(expected_msg)
			.found(DESC_FOUND_OVERFLOWING_INT)
			.wrap()
	))
}

unsafe fn zero_extend_array_le<
	const IN_BYTES: usize,
	const OUT_BYTES: usize
>(bytes: [u8; IN_BYTES]) -> [u8; OUT_BYTES] {
	debug_assert!(0 < IN_BYTES);
	debug_assert!(IN_BYTES < OUT_BYTES);

	let mut out = MaybeUninit::<[u8; OUT_BYTES]>::uninit();

	ptr::copy_nonoverlapping(bytes.as_ptr(), out.as_mut_ptr().cast::<u8>(), IN_BYTES);
	ptr::write_bytes(out.as_mut_ptr().cast::<u8>().add(IN_BYTES), 0, OUT_BYTES - IN_BYTES);

	out.assume_init()
}

unsafe fn sign_extend_array_le<
	const IN_BYTES: usize,
	const OUT_BYTES: usize
>(bytes: [u8; IN_BYTES]) -> [u8; OUT_BYTES] {
	debug_assert!(0 < IN_BYTES);
	debug_assert!(IN_BYTES < OUT_BYTES);

	let sign = bytes[IN_BYTES - 1] >> 7;
	let fill_byte = if sign == 0 { 0 } else { u8::MAX };

	let mut out = MaybeUninit::<[u8; OUT_BYTES]>::uninit();

	ptr::copy_nonoverlapping(bytes.as_ptr(), out.as_mut_ptr().cast::<u8>(), IN_BYTES);
	ptr::write_bytes(out.as_mut_ptr().cast::<u8>().add(IN_BYTES), fill_byte, OUT_BYTES - IN_BYTES);

	out.assume_init()
}

#[cfg(test)]
mod tests {
	use crate::serialiser_binary::{ deserialise, serialise };
	use super::*;
	use rand::{ Rng, thread_rng };

	macro_rules! gen_test {
		{ $($num:ident $fn_name:ident)* } => {
			$(
				#[test]
				fn $fn_name() {
					for _ in 0..1000 {
						let num = thread_rng().gen::<$num>();
						let serialised = serialise(&num);
						let deserialised = deserialise::<$num>(&serialised).unwrap();
						assert_eq!(deserialised, num);
					}
				}
			)*
		}
	}

	gen_test! {
		u8 roundtrip_u8
		u16 roundtrip_u16
		u32 roundtrip_u32
		u64 roundtrip_u64
		u128 roundtrip_u128
		usize roundtrip_usize
		i8 roundtrip_i8
		i16 roundtrip_i16
		i32 roundtrip_i32
		i64 roundtrip_i64
		i128 roundtrip_i128
		isize roundtrip_isize
		f32 roundtrip_f32
		f64 roundtrip_f64
	}
}
