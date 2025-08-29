use crate::prelude::*;
use self::private::Sealed;

macro_rules! from {
	{
		$losslessness:literal

		$(#[$trait_meta:meta])*
		trait $trait_name:ident

		$(#[$fn_meta:meta])*
		fn $fn_name:ident($type_name:ident)
	} => {
		#[doc = concat!(
			"Trait for number types that can be converted from [`",
			stringify!($type_name),
			"`], ",
			$losslessness
		)]
		#[doc = ""]
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			#[doc = concat!("Convert from a [`", stringify!($type_name), "`] value")]
			#[doc = ""]
			$(#[$fn_meta])*
			fn $fn_name(value: $type_name) -> Self;
		}
	};

	{
		impl $trait_name:ident::$fn_name:ident($type_name:ident)
		$(
			$(#[$meta:meta])*
			$impl_type:ident
		)*
	} => {
		$(
			$(#[$meta])*
			impl $trait_name for $impl_type {
				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn $fn_name(value: $type_name) -> $impl_type { value as _ }
			}
		)*
	};
}

macro_rules! into {
	{
		$losslessness:literal

		$(#[$trait_meta:meta])*
		trait $trait_name:ident

		$(#[$fn_meta:meta])*
		fn $fn_name:ident() -> $type_name:ident
	} => {
		#[doc = concat!(
			"Trait for number types that can be converted into [`",
			stringify!($type_name),
			"`], ",
			$losslessness
		)]
		#[doc = ""]
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			#[doc = concat!("Convert into a [`", stringify!($type_name), "`] value")]
			#[doc = ""]
			$(#[$fn_meta])*
			fn $fn_name(self) -> $type_name;
		}
	};

	{
		impl $trait_name:ident::$fn_name:ident() -> $type_name:ident
		$(
			$(#[$meta:meta])*
			$impl_type:ident
		)*
	} => {
		$(
			$(#[$meta])*
			impl $trait_name for $impl_type {
				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn $fn_name(self) -> $type_name { self as _ }
			}
		)*
	};
}

from! { "losslessly" trait FromU8 fn from_u8(u8) }
from! { "losslessly" trait FromU16 fn from_u16(u16) }
from! { "losslessly" trait FromU32 fn from_u32(u32) }
from! { "losslessly" trait FromU64 fn from_u64(u64) }
from! { "losslessly" trait FromU128 fn from_u128(u128) }
from! { "losslessly" trait FromUsize fn from_usize(usize) }
from! { "losslessly" trait FromI8 fn from_i8(i8) }
from! { "losslessly" trait FromI16 fn from_i16(i16) }
from! { "losslessly" trait FromI32 fn from_i32(i32) }
from! { "losslessly" trait FromI64 fn from_i64(i64) }
from! { "losslessly" trait FromI128 fn from_i128(i128) }
from! { "losslessly" trait FromIsize fn from_isize(isize) }
from! { "losslessly" trait FromF32 fn from_f32(f32) }
from! { "losslessly" trait FromF64 fn from_f64(f64) }
from! { "losslessly" trait FromBool fn from_bool(bool) }

from! { "potentially lossily" trait FromU8Lossy fn from_u8_lossy(u8) }
from! { "potentially lossily" trait FromU16Lossy fn from_u16_lossy(u16) }
from! { "potentially lossily" trait FromU32Lossy fn from_u32_lossy(u32) }
from! { "potentially lossily" trait FromU64Lossy fn from_u64_lossy(u64) }
from! { "potentially lossily" trait FromU128Lossy fn from_u128_lossy(u128) }
from! { "potentially lossily" trait FromUsizeLossy fn from_usize_lossy(usize) }
from! { "potentially lossily" trait FromI8Lossy fn from_i8_lossy(i8) }
from! { "potentially lossily" trait FromI16Lossy fn from_i16_lossy(i16) }
from! { "potentially lossily" trait FromI32Lossy fn from_i32_lossy(i32) }
from! { "potentially lossily" trait FromI64Lossy fn from_i64_lossy(i64) }
from! { "potentially lossily" trait FromI128Lossy fn from_i128_lossy(i128) }
from! { "potentially lossily" trait FromIsizeLossy fn from_isize_lossy(isize) }
from! { "potentially lossily" trait FromF32Lossy fn from_f32_lossy(f32) }
from! { "potentially lossily" trait FromF64Lossy fn from_f64_lossy(f64) }
from! { "potentially lossily" trait FromBoolLossy fn from_bool_lossy(bool) }

into! { "losslessly" trait IntoU8 fn into_u8() -> u8 }
into! { "losslessly" trait IntoU16 fn into_u16() -> u16 }
into! { "losslessly" trait IntoU32 fn into_u32() -> u32 }
into! { "losslessly" trait IntoU64 fn into_u64() -> u64 }
into! { "losslessly" trait IntoU128 fn into_u128() -> u128 }
into! { "losslessly" trait IntoUsize fn into_usize() -> usize }
into! { "losslessly" trait IntoI8 fn into_i8() -> i8 }
into! { "losslessly" trait IntoI16 fn into_i16() -> i16 }
into! { "losslessly" trait IntoI32 fn into_i32() -> i32 }
into! { "losslessly" trait IntoI64 fn into_i64() -> i64 }
into! { "losslessly" trait IntoI128 fn into_i128() -> i128 }
into! { "losslessly" trait IntoIsize fn into_isize() -> isize }
into! { "losslessly" trait IntoF32 fn into_f32() -> f32 }
into! { "losslessly" trait IntoF64 fn into_f64() -> f64 }

into! { "potentially lossily" trait IntoU8Lossy fn into_u8_lossy() -> u8 }
into! { "potentially lossily" trait IntoU16Lossy fn into_u16_lossy() -> u16 }
into! { "potentially lossily" trait IntoU32Lossy fn into_u32_lossy() -> u32 }
into! { "potentially lossily" trait IntoU64Lossy fn into_u64_lossy() -> u64 }
into! { "potentially lossily" trait IntoU128Lossy fn into_u128_lossy() -> u128 }
into! { "potentially lossily" trait IntoUsizeLossy fn into_usize_lossy() -> usize }
into! { "potentially lossily" trait IntoI8Lossy fn into_i8_lossy() -> i8 }
into! { "potentially lossily" trait IntoI16Lossy fn into_i16_lossy() -> i16 }
into! { "potentially lossily" trait IntoI32Lossy fn into_i32_lossy() -> i32 }
into! { "potentially lossily" trait IntoI64Lossy fn into_i64_lossy() -> i64 }
into! { "potentially lossily" trait IntoI128Lossy fn into_i128_lossy() -> i128 }
into! { "potentially lossily" trait IntoIsizeLossy fn into_isize_lossy() -> isize }
into! { "potentially lossily" trait IntoF32Lossy fn into_f32_lossy() -> f32 }
into! { "potentially lossily" trait IntoF64Lossy fn into_f64_lossy() -> f64 }

from! {
	impl FromU8::from_u8(u8)
	u8
	u16 u32 u64 u128 usize
	i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU16::from_u16(u16)
	u16
	u32 u64 u128 usize
	i32 i64 i128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	isize
	f32 f64
}

from! {
	impl FromU32::from_u32(u32)
	u32
	u64 u128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	usize
	i64 i128
	#[cfg(target_pointer_width = "64")]
	isize
	f64
}

from! {
	impl FromU64::from_u64(u64)
	u64
	u128
	#[cfg(target_pointer_width = "64")]
	usize
	i128
}

from! {
	impl FromU128::from_u128(u128)
	u128
}

from! {
	impl FromUsize::from_usize(usize)
	#[cfg(target_pointer_width = "16")]
	u16
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	u32
	u64 u128 usize
	#[cfg(target_pointer_width = "16")]
	i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	i64
	i128
	#[cfg(target_pointer_width = "16")]
	f32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	f64
}

from! {
	impl FromI8::from_i8(i8)
	i8
	i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI16::from_i16(i16)
	i16
	i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI32::from_i32(i32)
	i32
	i64 i128
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	isize
	f64
}

from! {
	impl FromI64::from_i64(i64)
	i64
	i128
	#[cfg(target_pointer_width = "64")]
	isize
}

from! {
	impl FromI128::from_i128(i128)
	i128
}

from! {
	impl FromIsize::from_isize(isize)
	#[cfg(target_pointer_width = "16")]
	i16
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	i32
	i64 i128 isize
	#[cfg(target_pointer_width = "16")]
	f32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	f64
}

from! {
	impl FromF32::from_f32(f32)
	f32 f64
}

from! {
	impl FromF64::from_f64(f64)
	f64
}

from! {
	impl FromBool::from_bool(bool)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl FromBool for f32 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool(value: bool) -> f32 { value as u32 as _ }
}

impl FromBool for f64 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool(value: bool) -> f64 { value as u64 as _ }
}

from! {
	impl FromU8Lossy::from_u8_lossy(u8)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU16Lossy::from_u16_lossy(u16)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU32Lossy::from_u32_lossy(u32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU64Lossy::from_u64_lossy(u64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromU128Lossy::from_u128_lossy(u128)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromUsizeLossy::from_usize_lossy(usize)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI8Lossy::from_i8_lossy(i8)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI16Lossy::from_i16_lossy(i16)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI32Lossy::from_i32_lossy(i32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI64Lossy::from_i64_lossy(i64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromI128Lossy::from_i128_lossy(i128)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromIsizeLossy::from_isize_lossy(isize)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromF32Lossy::from_f32_lossy(f32)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromF64Lossy::from_f64_lossy(f64)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

from! {
	impl FromBoolLossy::from_bool_lossy(bool)
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl FromBoolLossy for f32 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool_lossy(value: bool) -> f32 { value as u32 as _ }
}

impl FromBoolLossy for f64 {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn from_bool_lossy(value: bool) -> f64 { value as u64 as _ }
}

into! {
	impl IntoU8::into_u8() -> u8
	u8
	bool
}

into! {
	impl IntoU16::into_u16() -> u16
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	bool
}

into! {
	impl IntoU32::into_u32() -> u32
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	bool
}

into! {
	impl IntoU64::into_u64() -> u64
	u8 u16 u32 u64 usize
	bool
}

into! {
	impl IntoU128::into_u128() -> u128
	u8 u16 u32 u64 u128 usize
	bool
}

into! {
	impl IntoUsize::into_usize() -> usize
	u8 u16
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u32
	#[cfg(target_pointer_width = "64")]
	u64
	usize
	bool
}

into! {
	impl IntoI8::into_i8() -> i8
	i8
	bool
}

into! {
	impl IntoI16::into_i16() -> i16
	u8
	i8 i16
	#[cfg(target_pointer_width = "16")]
	isize
	bool
}

into! {
	impl IntoI32::into_i32() -> i32
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	i8 i16 i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	isize
	bool
}

into! {
	impl IntoI64::into_i64() -> i64
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	i8 i16 i32 i64 isize
	bool
}

into! {
	impl IntoI128::into_i128() -> i128
	u8 u16 u32 u64 usize
	i8 i16 i32 i64 i128 isize
	bool
}

into! {
	impl IntoIsize::into_isize() -> isize
	u8
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u16
	#[cfg(target_pointer_width = "64")]
	u32
	i8 i16
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	i32
	#[cfg(target_pointer_width = "64")]
	i64
	isize
	bool
}

into! {
	impl IntoF32::into_f32() -> f32
	u8 u16
	#[cfg(target_pointer_width = "16")]
	usize
	i8 i16
	#[cfg(target_pointer_width = "16")]
	isize
	f32
}

impl IntoF32 for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	fn into_f32(self) -> f32 { self as u32 as _ }
}

into! {
	impl IntoF64::into_f64() -> f64
	u8 u16 u32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize
	i8 i16 i32
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	isize
	f32 f64
}

impl IntoF64 for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn into_f64(self) -> f64 { self as u64 as _ }
}

into! {
	impl IntoU8Lossy::into_u8_lossy() -> u8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU16Lossy::into_u16_lossy() -> u16
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU32Lossy::into_u32_lossy() -> u32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU64Lossy::into_u64_lossy() -> u64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoU128Lossy::into_u128_lossy() -> u128
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoUsizeLossy::into_usize_lossy() -> usize
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI8Lossy::into_i8_lossy() -> i8
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI16Lossy::into_i16_lossy() -> i16
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI32Lossy::into_i32_lossy() -> i32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI64Lossy::into_i64_lossy() -> i64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoI128Lossy::into_i128_lossy() -> i128
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoIsizeLossy::into_isize_lossy() -> isize
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
	bool
}

into! {
	impl IntoF32Lossy::into_f32_lossy() -> f32
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

impl IntoF32Lossy for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	// there is possibility for (micro)optimisation here...
	// figure out which intermediate int type is best to cast to
	fn into_f32_lossy(self) -> f32 { self as u32 as _ }
}

into! {
	impl IntoF64Lossy::into_f64_lossy() -> f64
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}

impl IntoF64Lossy for bool {
	#[expect(
		clippy::as_conversions,
		reason = "implementation detail of a more restrictive API"
	)]
	#[expect(
		clippy::inline_always,
		reason = "simple cast op"
	)]
	#[inline(always)]
	fn into_f64_lossy(self) -> f64 { self as u64 as _ }
}

pub trait FromNum<Num>
where
	Self: Sealed
{
	fn from_num(num: Num) -> Self;
}

pub trait FromNumLossy<Num>
where
	Self: Sealed
{
	fn from_num_lossy(num: Num) -> Self;
}

pub trait IntoNum<Num>
where
	Self: Sealed
{
	fn into_num(self) -> Num;
}

pub trait IntoNumLossy<Num>
where
	Self: Sealed
{
	fn into_num_lossy(self) -> Num;
}

impl<NFrom, NInto> FromNumLossy<NFrom> for NInto
where
	NInto: FromNum<NFrom>
{
	#[inline]
	fn from_num_lossy(num: NFrom) -> Self {
		NInto::from_num(num)
	}
}

impl<NFrom, NInto> IntoNum<NInto> for NFrom
where
	NFrom: Sealed,
	NInto: FromNum<NFrom>
{
	#[inline]
	fn into_num(self) -> NInto {
		NInto::from_num(self)
	}
}

impl<NFrom, NInto> IntoNumLossy<NInto> for NFrom
where
	NFrom: Sealed,
	NInto: FromNumLossy<NFrom>
{
	#[inline]
	fn into_num_lossy(self) -> NInto {
		NInto::from_num_lossy(self)
	}
}

macro_rules! impl_generic_num_conversions {
	{
		for $to:ident
		lossless $trait_lossless:ident::$fn_lossless:ident
		lossy $trait_lossy:ident::$fn_lossy:ident
		$($(#[$meta:meta])* $from:ident $lossness:ident)*
	} => {
		$(
			impl_generic_num_conversions! {
				@impl
				$trait_lossless $fn_lossless
				$trait_lossy $fn_lossy
				$(#[$meta])* $from $to $lossness
			}
		)*
	};

	{
		@impl
		$trait_lossless:ident $fn_lossless:ident
		$trait_lossy:ident $fn_lossy:ident
		$(#[$meta:meta])* $from:ident $to:ident lossless
	} => {
		impl_generic_num_conversions! {
			@impl
			$(#[$meta])* $from $to
			FromNum from_num
			$trait_lossless $fn_lossless
		}
	};

	{
		@impl
		$trait_lossless:ident $fn_lossless:ident
		$trait_lossy:ident $fn_lossy:ident
		$(#[$meta:meta])* $from:ident $to:ident lossy
	} => {
		impl_generic_num_conversions! {
			@impl
			$(#[$meta])* $from $to
			FromNumLossy from_num_lossy
			$trait_lossy $fn_lossy
		}
	};

	{
		@impl
		$(#[$meta:meta])*
		$from:ident $to:ident
		$generic_trait:ident $generic_fn:ident
		$trait_name:ident $fn_name:ident
	} => {
		$(#[$meta])*
		impl $generic_trait<$from> for $to {
			#[inline(always)]
			fn $generic_fn(num: $from) -> $to {
				<$from as $trait_name>::$fn_name(num)
			}
		}
	};
}

impl_generic_num_conversions! {
	for u8
	lossless IntoU8::into_u8
	lossy IntoU8Lossy::into_u8_lossy

	u8 lossless
	u16 lossy
	u32 lossy
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for u16
	lossless IntoU16::into_u16
	lossy IntoU16Lossy::into_u16_lossy

	u8 lossless
	u16 lossless
	u32 lossy
	u64 lossy
	u128 lossy
	#[cfg(target_pointer_width = "16")]
	usize lossless
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	usize lossy
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for u32
	lossless IntoU32::into_u32
	lossy IntoU32Lossy::into_u32_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossy
	u128 lossy
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize lossless
	#[cfg(target_pointer_width = "64")]
	usize lossy
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for u64
	lossless IntoU64::into_u64
	lossy IntoU64Lossy::into_u64_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossless
	u128 lossy
	usize lossless
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for u128
	lossless IntoU128::into_u128
	lossy IntoU128Lossy::into_u128_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossless
	u128 lossless
	usize lossless
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for usize
	lossless IntoUsize::into_usize
	lossy IntoUsizeLossy::into_usize_lossy

	u8 lossless
	u16 lossless
	#[cfg(target_pointer_width = "16")]
	u32 lossy
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u32 lossless
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	u64 lossy
	#[cfg(target_pointer_width = "64")]
	u64 lossless
	u128 lossy
	usize lossless
	i8 lossy
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for i8
	lossless IntoI8::into_i8
	lossy IntoI8Lossy::into_i8_lossy

	u8 lossy
	u16 lossy
	u32 lossy
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossless
	i16 lossy
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for i16
	lossless IntoI16::into_i16
	lossy IntoI16Lossy::into_i16_lossy

	u8 lossless
	u16 lossy
	u32 lossy
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossless
	i16 lossless
	i32 lossy
	i64 lossy
	i128 lossy
	#[cfg(target_pointer_width = "16")]
	isize lossless
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for i32
	lossless IntoI32::into_i32
	lossy IntoI32Lossy::into_i32_lossy

	u8 lossless
	u16 lossless
	u32 lossy
	u64 lossy
	u128 lossy
	#[cfg(target_pointer_width = "16")]
	usize lossless
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	usize lossy
	i8 lossless
	i16 lossless
	i32 lossless
	i64 lossy
	i128 lossy
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	isize lossless
	#[cfg(target_pointer_width = "64")]
	isize lossy
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for i64
	lossless IntoI64::into_i64
	lossy IntoI64Lossy::into_i64_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossy
	u128 lossy
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	usize lossless
	#[cfg(target_pointer_width = "64")]
	usize lossy
	i8 lossless
	i16 lossless
	i32 lossless
	i64 lossless
	i128 lossy
	isize lossless
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for i128
	lossless IntoI128::into_i128
	lossy IntoI128Lossy::into_i128_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossless
	u128 lossy
	usize lossless
	i8 lossless
	i16 lossless
	i32 lossless
	i64 lossless
	i128 lossless
	isize lossless
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for isize
	lossless IntoIsize::into_isize
	lossy IntoIsizeLossy::into_isize_lossy

	u8 lossless
	#[cfg(target_pointer_width = "16")]
	u16 lossy
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	u16 lossless
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	u32 lossy
	#[cfg(target_pointer_width = "64")]
	u32 lossless
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossless
	i16 lossless
	#[cfg(target_pointer_width = "16")]
	i32 lossy
	#[cfg(any(
		target_pointer_width = "32",
		target_pointer_width = "64"
	))]
	i32 lossless
	#[cfg(any(
		target_pointer_width = "16",
		target_pointer_width = "32"
	))]
	i64 lossy
	#[cfg(target_pointer_width = "64")]
	i64 lossless
	i128 lossy
	isize lossless
	f32 lossy
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for f32
	lossless IntoF32::into_f32
	lossy IntoF32Lossy::into_f32_lossy

	u8 lossless
	u16 lossless
	u32 lossy
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossless
	i16 lossless
	i32 lossy
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossless
	f64 lossy
	bool lossless
}

impl_generic_num_conversions! {
	for f64
	lossless IntoF64::into_f64
	lossy IntoF64Lossy::into_f64_lossy

	u8 lossless
	u16 lossless
	u32 lossless
	u64 lossy
	u128 lossy
	usize lossy
	i8 lossless
	i16 lossless
	i32 lossless
	i64 lossy
	i128 lossy
	isize lossy
	f32 lossless
	f64 lossless
	bool lossless
}

pub trait ArrayConversions<const N: usize>
where
	Self: Sealed
{
	fn into_le_bytes(self) -> [u8; N];
	fn into_be_bytes(self) -> [u8; N];
	fn into_ne_bytes(self) -> [u8; N];
	fn from_le_bytes(bytes: [u8; N]) -> Self;
	fn from_be_bytes(bytes: [u8; N]) -> Self;
	fn from_ne_bytes(bytes: [u8; N]) -> Self;
}

macro_rules! impl_array_conversions {
	{ $($(#[$meta:meta])* $num:ident $bytes:literal)* } => {
		$(
			$(#[$meta])*
			impl ArrayConversions<$bytes> for $num {
				#[inline(always)]
				fn into_le_bytes(self) -> [u8; $bytes] { $num::to_le_bytes(self) }

				#[inline(always)]
				fn into_be_bytes(self) -> [u8; $bytes] { $num::to_be_bytes(self) }

				#[inline(always)]
				fn into_ne_bytes(self) -> [u8; $bytes] { $num::to_ne_bytes(self) }

				#[inline(always)]
				fn from_le_bytes(bytes: [u8; $bytes]) -> $num { $num::from_le_bytes(bytes) }

				#[inline(always)]
				fn from_be_bytes(bytes: [u8; $bytes]) -> $num { $num::from_be_bytes(bytes) }

				#[inline(always)]
				fn from_ne_bytes(bytes: [u8; $bytes]) -> $num { $num::from_ne_bytes(bytes) }
			}
		)*
	}
}

impl_array_conversions! {
	u8 1
	u16 2
	u32 4
	u64 8
	u128 16

	i8 1
	i16 2
	i32 4
	i64 8
	i128 16

	f32 4
	f64 8

	#[cfg(target_pointer_width = "16")]
	usize 2
	#[cfg(target_pointer_width = "16")]
	isize 2

	#[cfg(target_pointer_width = "32")]
	usize 4
	#[cfg(target_pointer_width = "32")]
	isize 4

	#[cfg(target_pointer_width = "64")]
	usize 8
	#[cfg(target_pointer_width = "64")]
	isize 8
}

pub trait Endian<Num, const N: usize>
where
	Self: Sealed,
	Num: ArrayConversions<N>
{
	fn from_bytes(bytes: [u8; N]) -> Num;
	fn into_bytes(num: Num) -> [u8; N];
}

macro_rules! decl_endian_structs {
	{ $($endian_str:literal $struct:ident $from:ident $into:ident)* } => {
		$(
			/// Implements [`Endian`] trait and provides
			#[doc = $endian_str]
			/// conversions between numbers and arrays
			pub struct $struct {
				__private: ()
			}

			impl Sealed for $struct {}

			impl<Num, const N: usize> Endian<Num, N> for $struct
			where
				Num: ArrayConversions<N>
			{
				#[inline]
				fn from_bytes(bytes: [u8; N]) -> Num {
					Num::$from(bytes)
				}

				#[inline]
				fn into_bytes(num: Num) -> [u8; N] {
					Num::$into(num)
				}
			}
		)*
	}
}

decl_endian_structs! {
	"little endian" EndianLittle from_le_bytes into_le_bytes
	"big endian" EndianBig from_be_bytes into_be_bytes
	"native endian" EndianNative from_ne_bytes into_ne_bytes
}

pub trait IntSignedness
where
	Self: Sealed
{
	const SIGNED: bool;
}

macro_rules! impl_int_signedness {
	{ $($int:ident $signed:literal)* } => {
		$(
			impl IntSignedness for $int {
				const SIGNED: bool = $signed;
			}
		)*
	}
}

impl_int_signedness! {
	u8 false
	u16 false
	u32 false
	u64 false
	u128 false
	usize false
	i8 true
	i16 true
	i32 true
	i64 true
	i128 true
	isize true
}

pub trait IntUnsigned
where
	Self: Sealed,
	Self::Signed: IntSigned<Unsigned = Self>
{
	type Signed;

	fn into_signed(self) -> Self::Signed;
}

pub trait IntSigned
where
	Self: Sealed,
	Self::Unsigned: IntUnsigned<Signed = Self>
{
	type Unsigned;

	fn into_unsigned(self) -> Self::Unsigned;
}

macro_rules! impl_int_signed_and_unsigned {
	{ $($unsigned:ident $signed:ident)* } => {
		$(
			impl IntUnsigned for $unsigned {
				type Signed = $signed;

				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn into_signed(self) -> $signed { self as _ }
			}

			impl IntSigned for $signed {
				type Unsigned = $unsigned;

				#[expect(
					clippy::as_conversions,
					reason = "implementation detail of a more restrictive API"
				)]
				#[inline(always)]
				fn into_unsigned(self) -> $unsigned { self as _ }
			}
		)*
	}
}

impl_int_signed_and_unsigned! {
	u8 i8
	u16 i16
	u32 i32
	u64 i64
	u128 i128
	usize isize
}

macro_rules! op_trait {
	{
		$(#[$trait_meta:meta])*
		trait $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident(rhs)
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R>
		where
			Self: Sealed,
			R: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(option) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident(rhs)
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R>
		where
			Self: Sealed,
			R: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> Option<Self::Output>;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(option) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self) -> Option<Self::Output>;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(tuple) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident(rhs)
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R>
		where
			Self: Sealed,
			R: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self, rhs: R) -> (Self::Output, bool);
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(tuple) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			fn $fn_name(self) -> (Self::Output, bool);
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(unsafe) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident(rhs)
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name<R>
		where
			Self: Sealed,
			R: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			/// # Safety
			///
			/// TODO (also remove me from here (definition)
			/// and write it in the macro invocation)
			unsafe fn $fn_name(self, rhs: R) -> Self::Output;
		}
	};

	{
		$(#[$trait_meta:meta])*
		trait(unsafe) $trait_name:ident
		$(#[$fn_meta:meta])*
		fn $fn_name:ident()
		$(
			$(#[$output_meta:meta])*
			type Output
		)?
	} => {
		$(#[$trait_meta])*
		pub trait $trait_name
		where
			Self: Sealed
		{
			$($(#[$output_meta])*)?
			type Output;

			$(#[$fn_meta])*
			/// # Safety
			///
			/// TODO (also remove me from here (definition)
			/// and write it in the macro invocation)
			unsafe fn $fn_name(self) -> Self::Output;
		}
	};

	{
		impl
		$(
			$lhs:ident: $lhs_ty:ident { $($lhs_stuff:tt)* }
		)*
	} => {
		$(
			op_trait! {
				impl(lhs_stuff)
				$lhs $lhs_ty $($lhs_stuff)*
			}
		)*
	};

	{
		impl(lhs_stuff)
		$lhs:ident $lhs_ty:ident
		$rhs:ident: $rhs_ty:ident { $($rhs_stuff:tt)* }
		$($rest:tt)*
	} => {
		op_trait! {
			impl(rhs_stuff)
			$lhs $lhs_ty
			$rhs $rhs_ty
			$($rhs_stuff)*
		}
		op_trait! {
			impl(lhs_stuff)
			$lhs $lhs_ty
			$($rest)*
		}
	};

	{
		impl(lhs_stuff)
		$lhs:ident $lhs_ty:ident
		$(#[$meta:meta])*
		$op:ident -> $output_ty:ident { $($body:tt)* }
		$($rest:tt)*
	} => {
		op_trait! {
			impl(helper_lhs)
			$op
			$(#[$meta])*
			$lhs $lhs_ty
			$output_ty
			$($body)*
		}
		op_trait! {
			impl(lhs_stuff)
			$lhs $lhs_ty
			$($rest)*
		}
	};

	{
		impl(lhs_stuff)
		$lhs:ident $lhs_ty:ident
	} => {};

	{
		impl(rhs_stuff)
		$lhs:ident $lhs_ty:ident
		$rhs:ident $rhs_ty:ident
		$(#[$meta:meta])*
		$op:ident -> $output_ty:ident { $($body:tt)* }
		$($rest:tt)*
	} => {
		op_trait! {
			impl(helper_rhs)
			$op
			$(#[$meta])*
			$lhs $lhs_ty
			$rhs $rhs_ty
			$output_ty
			$($body)*
		}
		op_trait! {
			impl(rhs_stuff)
			$lhs $lhs_ty
			$rhs $rhs_ty
			$($rest)*
		}
	};

	{
		impl(rhs_stuff)
		$lhs:ident $lhs_ty:ident
		$rhs:ident $rhs_ty:ident
	} => {};

	{
		impl(normal)
		$trait_name:ident $fn_name:ident
		$(#[$meta:meta])*
		$lhs:ident $lhs_ty:ident
		$rhs:ident $rhs_ty:ident
		$output_ty:ident
		$($body:tt)*
	} => {
		$(#[$meta])*
		impl $trait_name<$rhs_ty> for $lhs_ty {
			type Output = $output_ty;

			#[inline(always)]
			fn $fn_name(self, rhs: $rhs_ty) -> $output_ty {
				#[inline(always)]
				fn inner($lhs: $lhs_ty, $rhs: $rhs_ty) -> $output_ty {
					$($body)*
				}

				inner(self, rhs)
			}
		}
	};

	{
		impl(normal1)
		$trait_name:ident $fn_name:ident
		$(#[$meta:meta])*
		$lhs:ident $lhs_ty:ident
		$output_ty:ident
		$($body:tt)*
	} => {
		$(#[$meta])*
		impl $trait_name for $lhs_ty {
			type Output = $output_ty;

			#[inline(always)]
			fn $fn_name(self) -> $output_ty {
				#[inline(always)]
				fn inner($lhs: $lhs_ty) -> $output_ty {
					$($body)*
				}

				inner(self)
			}
		}
	};

	{ impl(helper_rhs) add $($stuff:tt)* } => { op_trait! { impl(normal) Add add $($stuff)* } };
	{ impl(helper_rhs) sub $($stuff:tt)* } => { op_trait! { impl(normal) Sub sub $($stuff)* } };
	{ impl(helper_rhs) mul $($stuff:tt)* } => { op_trait! { impl(normal) Mul mul $($stuff)* } };
	{ impl(helper_rhs) div $($stuff:tt)* } => { op_trait! { impl(normal) Div div $($stuff)* } };
	{ impl(helper_lhs) neg $($stuff:tt)* } => { op_trait! { impl(normal1) Neg neg $($stuff)* } };
}

op_trait! { trait Add fn add(rhs) }
op_trait! { trait Sub fn sub(rhs) }
op_trait! { trait Mul fn mul(rhs) }
op_trait! { trait Div fn div(rhs) }
op_trait! { trait Neg fn neg() }

op_trait! { trait(option) AddChecked fn add_checked(rhs) }
op_trait! { trait(option) SubChecked fn sub_checked(rhs) }
op_trait! { trait(option) MulChecked fn mul_checked(rhs) }
op_trait! { trait(option) DivChecked fn div_checked(rhs) }
op_trait! { trait(option) NegChecked fn neg_checked() }

op_trait! { trait(tuple) AddOverflowing fn add_overflowing(rhs) }
op_trait! { trait(tuple) SubOverflowing fn sub_overflowing(rhs) }
op_trait! { trait(tuple) MulOverflowing fn mul_overflowing(rhs) }
op_trait! { trait(tuple) DivOverflowing fn div_overflowing(rhs) }
op_trait! { trait(tuple) NegOverflowing fn neg_overflowing() }

op_trait! { trait AddSaturating fn add_saturating(rhs) }
op_trait! { trait SubSaturating fn sub_saturating(rhs) }
op_trait! { trait MulSaturating fn mul_saturating(rhs) }
op_trait! { trait DivSaturating fn div_saturating(rhs) }
op_trait! { trait NegSaturating fn neg_saturating() }

// // TODO: strict op?
// // TODO: unbounded shifts?

op_trait! { trait(unsafe) AddUnchecked fn add_unchecked(rhs) }
op_trait! { trait(unsafe) SubUnchecked fn sub_unchecked(rhs) }
op_trait! { trait(unsafe) MulUnchecked fn mul_unchecked(rhs) }
op_trait! { trait(unsafe) DivUnchecked fn div_unchecked(rhs) }
op_trait! { trait(unsafe) NegUnchecked fn neg_unchecked() }

op_trait! { trait AddWrapping fn add_wrapping(rhs) }
op_trait! { trait SubWrapping fn sub_wrapping(rhs) }
op_trait! { trait MulWrapping fn mul_wrapping(rhs) }
op_trait! { trait DivWrapping fn div_wrapping(rhs) }
op_trait! { trait NegWrapping fn neg_wrapping() }

op_trait! {
	impl

	lhs: u8 {
		rhs: u8 {
			add -> u8 { lhs + rhs }
			sub -> u8 { lhs - rhs }
			mul -> u8 { lhs * rhs }
			div -> u8 { lhs / rhs }
		}

		rhs: u16 {
			add -> u16 { lhs.into_u16() + rhs }
			sub -> u16 { lhs.into_u16() - rhs }
			mul -> u16 { lhs.into_u16() * rhs }
			div -> u16 { lhs.into_u16() / rhs }
		}

		rhs: u32 {
			add -> u32 { lhs.into_u32() + rhs }
			sub -> u32 { lhs.into_u32() - rhs }
			mul -> u32 { lhs.into_u32() * rhs }
			div -> u32 { lhs.into_u32() / rhs }
		}

		rhs: u64 {
			add -> u64 { lhs.into_u64() + rhs }
			sub -> u64 { lhs.into_u64() - rhs }
			mul -> u64 { lhs.into_u64() * rhs }
			div -> u64 { lhs.into_u64() / rhs }
		}

		rhs: u128 {
			add -> u128 { lhs.into_u128() + rhs }
			sub -> u128 { lhs.into_u128() - rhs }
			mul -> u128 { lhs.into_u128() * rhs }
			div -> u128 { lhs.into_u128() / rhs }
		}

		rhs: usize {
			add -> usize { lhs.into_usize() + rhs }
			sub -> usize { lhs.into_usize() - rhs }
			mul -> usize { lhs.into_usize() * rhs }
			div -> usize { lhs.into_usize() / rhs }
		}
	}

	lhs: u16 {}

	lhs: u32 {}

	lhs: u64 {}

	lhs: u128 {}

	lhs: usize {}

	lhs: i8 {
		neg -> i8 { -lhs }
		rhs: i8 {
			add -> i8 { lhs + rhs }
		}
	}

	lhs: i16 {}

	lhs: i32 {}

	lhs: i64 {}

	lhs: i128 {}

	lhs: isize {}

	lhs: f32 {}

	lhs: f64 {}
}

/*
macro_rules! op_trait {
	{ impl(helper) add $($stuff:tt)* } => { op_trait! { impl(normal) Add::add $($stuff)* } };
	{ impl(helper) sub $($stuff:tt)* } => { op_trait! { impl(normal) Sub::sub $($stuff)* } };
	{ impl(helper) mul $($stuff:tt)* } => { op_trait! { impl(normal) Mul::mul $($stuff)* } };
	{ impl(helper) div $($stuff:tt)* } => { op_trait! { impl(normal) Div::div $($stuff)* } };
	{ impl(helper) neg $($stuff:tt)* } => { op_trait! { impl(normal) Neg::neg $($stuff)* } };

	{ impl(helper) add_checked $($stuff:tt)* } => { op_trait! { impl(checked) AddChecked::add_checked $($stuff)* } };
	{ impl(helper) sub_checked $($stuff:tt)* } => { op_trait! { impl(checked) SubChecked::sub_checked $($stuff)* } };
	{ impl(helper) mul_checked $($stuff:tt)* } => { op_trait! { impl(checked) MulChecked::mul_checked $($stuff)* } };
	{ impl(helper) div_checked $($stuff:tt)* } => { op_trait! { impl(checked) DivChecked::div_checked $($stuff)* } };
	{ impl(helper) neg_checked $($stuff:tt)* } => { op_trait! { impl(checked) NegChecked::neg_checked $($stuff)* } };

	{ impl(helper) add_overflowing $($stuff:tt)* } => { op_trait! { impl(overflowing) AddOverflowing::add_overflowing $($stuff)* } };
	{ impl(helper) sub_overflowing $($stuff:tt)* } => { op_trait! { impl(overflowing) SubOverflowing::sub_overflowing $($stuff)* } };
	{ impl(helper) mul_overflowing $($stuff:tt)* } => { op_trait! { impl(overflowing) MulOverflowing::mul_overflowing $($stuff)* } };
	{ impl(helper) div_overflowing $($stuff:tt)* } => { op_trait! { impl(overflowing) DivOverflowing::div_overflowing $($stuff)* } };
	{ impl(helper) neg_overflowing $($stuff:tt)* } => { op_trait! { impl(overflowing) NegOverflowing::neg_overflowing $($stuff)* } };

	{ impl(helper) add_saturating $($stuff:tt)* } => { op_trait! { impl(saturating) AddSaturating::add_saturating $($stuff)* } };
	{ impl(helper) sub_saturating $($stuff:tt)* } => { op_trait! { impl(saturating) SubSaturating::sub_saturating $($stuff)* } };
	{ impl(helper) mul_saturating $($stuff:tt)* } => { op_trait! { impl(saturating) MulSaturating::mul_saturating $($stuff)* } };
	{ impl(helper) div_saturating $($stuff:tt)* } => { op_trait! { impl(saturating) DivSaturating::div_saturating $($stuff)* } };
	{ impl(helper) neg_saturating $($stuff:tt)* } => { op_trait! { impl(saturating) NegSaturating::neg_saturating $($stuff)* } };

	{ impl(helper) add_unchecked $($stuff:tt)* } => { op_trait! { impl(unchecked) AddUnchecked::add_unchecked $($stuff)* } };
	{ impl(helper) sub_unchecked $($stuff:tt)* } => { op_trait! { impl(unchecked) SubUnchecked::sub_unchecked $($stuff)* } };
	{ impl(helper) mul_unchecked $($stuff:tt)* } => { op_trait! { impl(unchecked) MulUnchecked::mul_unchecked $($stuff)* } };
	{ impl(helper) div_unchecked $($stuff:tt)* } => { op_trait! { impl(unchecked) DivUnchecked::div_unchecked $($stuff)* } };
	{ impl(helper) neg_unchecked $($stuff:tt)* } => { op_trait! { impl(unchecked) NegUnchecked::neg_unchecked $($stuff)* } };

	{ impl(helper) add_wrapping $($stuff:tt)* } => { op_trait! { impl(wrapping) AddWrapping::add_wrapping $($stuff)* } };
	{ impl(helper) sub_wrapping $($stuff:tt)* } => { op_trait! { impl(wrapping) SubWrapping::sub_wrapping $($stuff)* } };
	{ impl(helper) mul_wrapping $($stuff:tt)* } => { op_trait! { impl(wrapping) MulWrapping::mul_wrapping $($stuff)* } };
	{ impl(helper) div_wrapping $($stuff:tt)* } => { op_trait! { impl(wrapping) DivWrapping::div_wrapping $($stuff)* } };
	{ impl(helper) neg_wrapping $($stuff:tt)* } => { op_trait! { impl(wrapping) NegWrapping::neg_wrapping $($stuff)* } };
}

op_trait! {
	impl

	lhs: u8 {
		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: u8 {
			add() -> u8 { lhs + rhs }
			sub() -> u8 { lhs - rhs }
			mul() -> u8 { lhs * rhs }
			div() -> u8 { lhs / rhs }

			add_checked() -> u8 { lhs.checked_add(rhs) }
			sub_checked() -> u8 { lhs.checked_sub(rhs) }
			mul_checked() -> u8 { lhs.checked_mul(rhs) }
			div_checked() -> u8 { lhs.checked_div(rhs) }

			add_overflowing() -> u8 { lhs.overflowing_add(rhs) }
			sub_overflowing() -> u8 { lhs.overflowing_sub(rhs) }
			mul_overflowing() -> u8 { lhs.overflowing_mul(rhs) }
			div_overflowing() -> u8 { lhs.overflowing_div(rhs) }

			add_saturating() -> u8 { lhs.saturating_add(rhs) }
			sub_saturating() -> u8 { lhs.saturating_sub(rhs) }
			mul_saturating() -> u8 { lhs.saturating_mul(rhs) }
			div_saturating() -> u8 { lhs.saturating_div(rhs) }

			// SAFETY: invariants upheld by caller
			add_unchecked() -> u8 { unsafe { lhs.unchecked_add(rhs) } }
			// SAFETY: invariants upheld by caller
			sub_unchecked() -> u8 { unsafe { lhs.unchecked_sub(rhs) } }
			// SAFETY: invariants upheld by caller
			mul_unchecked() -> u8 { unsafe { lhs.unchecked_mul(rhs) } }
			div_unchecked() -> u8 { lhs / rhs }

			add_wrapping() -> u8 { lhs.wrapping_add(rhs) }
			sub_wrapping() -> u8 { lhs.wrapping_sub(rhs) }
			mul_wrapping() -> u8 { lhs.wrapping_mul(rhs) }
			div_wrapping() -> u8 { lhs.wrapping_div(rhs) }
		}

		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: u16 {
			add() -> u16 { lhs.into_u16() + rhs }
			sub() -> u16 { lhs.into_u16() - rhs }
			mul() -> u16 { lhs.into_u16() * rhs }
			div() -> u16 { lhs.into_u16() / rhs }

			add_checked() -> u16 { lhs.into_u16().checked_add(rhs) }
			sub_checked() -> u16 { lhs.into_u16().checked_sub(rhs) }
			mul_checked() -> u16 { lhs.into_u16().checked_mul(rhs) }
			div_checked() -> u16 { lhs.into_u16().checked_div(rhs) }
		}

		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: u32 {
			add() -> u32 { lhs.into_u32() + rhs }
			sub() -> u32 { lhs.into_u32() - rhs }
			mul() -> u32 { lhs.into_u32() * rhs }
			div() -> u32 { lhs.into_u32() / rhs }

			add_checked() -> u32 { lhs.into_u32().checked_add(rhs) }
			sub_checked() -> u32 { lhs.into_u32().checked_sub(rhs) }
			mul_checked() -> u32 { lhs.into_u32().checked_mul(rhs) }
			div_checked() -> u32 { lhs.into_u32().checked_div(rhs) }
		}

		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: u64 {
			add() -> u64 { lhs.into_u64() + rhs }
			sub() -> u64 { lhs.into_u64() - rhs }
			mul() -> u64 { lhs.into_u64() * rhs }
			div() -> u64 { lhs.into_u64() / rhs }

			add_checked() -> u64 { lhs.into_u64().checked_add(rhs) }
			sub_checked() -> u64 { lhs.into_u64().checked_sub(rhs) }
			mul_checked() -> u64 { lhs.into_u64().checked_mul(rhs) }
			div_checked() -> u64 { lhs.into_u64().checked_div(rhs) }
		}

		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: u128 {
			add() -> u128 { lhs.into_u128() + rhs }
			sub() -> u128 { lhs.into_u128() - rhs }
			mul() -> u128 { lhs.into_u128() * rhs }
			div() -> u128 { lhs.into_u128() / rhs }

			add_checked() -> u128 { lhs.into_u128().checked_add(rhs) }
			sub_checked() -> u128 { lhs.into_u128().checked_sub(rhs) }
			mul_checked() -> u128 { lhs.into_u128().checked_mul(rhs) }
			div_checked() -> u128 { lhs.into_u128().checked_div(rhs) }
		}
	}

	lhs: i8 {
		#[cfg(any(
			target_pointer_width = "16",
			target_pointer_width = "32",
			target_pointer_width = "64"
		))]
		rhs: i8 {
			add() -> i8 { lhs + rhs }
			sub() -> i8 { lhs - rhs }
			mul() -> i8 { lhs * rhs }
			div() -> i8 { lhs / rhs }

			add_checked() -> i8 { lhs.checked_add(rhs) }
			sub_checked() -> i8 { lhs.checked_sub(rhs) }
			mul_checked() -> i8 { lhs.checked_mul(rhs) }
			div_checked() -> i8 { lhs.checked_div(rhs) }

			add_overflowing() -> i8 { lhs.overflowing_add(rhs) }
			sub_overflowing() -> i8 { lhs.overflowing_sub(rhs) }
			mul_overflowing() -> i8 { lhs.overflowing_mul(rhs) }
			div_overflowing() -> i8 { lhs.overflowing_div(rhs) }

			add_saturating() -> i8 { lhs.saturating_add(rhs) }
			sub_saturating() -> i8 { lhs.saturating_sub(rhs) }
			mul_saturating() -> i8 { lhs.saturating_mul(rhs) }
			div_saturating() -> i8 { lhs.saturating_div(rhs) }

			// SAFETY: invariants upheld by caller
			add_unchecked() -> i8 { unsafe { lhs.unchecked_add(rhs) } }
			// SAFETY: invariants upheld by caller
			sub_unchecked() -> i8 { unsafe { lhs.unchecked_sub(rhs) } }
			// SAFETY: invariants upheld by caller
			mul_unchecked() -> i8 { unsafe { lhs.unchecked_mul(rhs) } }
			div_unchecked() -> i8 { lhs / rhs }

			add_wrapping() -> i8 { lhs.wrapping_add(rhs) }
			sub_wrapping() -> i8 { lhs.wrapping_sub(rhs) }
			mul_wrapping() -> i8 { lhs.wrapping_mul(rhs) }
			div_wrapping() -> i8 { lhs.wrapping_div(rhs) }
		}
	}
}
*/

/// notouchie
mod private {
	use super::*;

	/// notouchie
	pub trait Sealed
	where
		Self: Sized
	{}

	macro_rules! impl_sealed {
		{ $($type:ident)* } => {
			$(
				impl Sealed for $type {}
			)*
		}
	}

	impl_sealed! {
		u8 u16 u32 u64 u128 usize
		i8 i16 i32 i64 i128 isize
		f32 f64
		bool
	}

	impl<T> Sealed for &T
	where
		T: Sealed
	{}

	impl<T> Sealed for &mut T
	where
		T: Sealed
	{}
}
