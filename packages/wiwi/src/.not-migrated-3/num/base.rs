use crate::rust_std::clone::Clone;
use crate::rust_std::marker::Sized;
use crate::rust_std::mem::{ align_of, size_of };

/// Common base for all numbers
pub trait Base: Sized + Clone + private::Sealed {
	/// Min (finite) value of this number type
	const MIN: Self;
	/// Max (finite) value of this number type
	const MAX: Self;
	/// `0`
	const ZERO: Self;
	/// `1`
	const ONE: Self;
	/// Size of this number in bits
	const BITS: Self;
	/// Size of this number type in bytes
	const BYTES: Self;
	/// Align of this number type in bytes
	const ALIGN: Self;
	/// Size of this number in bits, type `usize`
	const BITS_USIZE: usize;
	/// Size of this number type in bytes, type `usize`
	const BYTES_USIZE: usize;
	/// Align of this number type in bytes, type `usize`
	const ALIGN_USIZE: usize;

	fn from_bool(b: bool) -> Self;
}

mod private {
	pub trait Sealed {}
}

macro_rules! impl_num_trait_base {
	{
		int: unsigned $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0;
			one: 1;
			bits: {
				// attributes on expressions are experimental, so. we do this
				#[expect(clippy::as_conversions)]
				let val = $num::BITS as _;
				val
			};
			from_bool: b => {
				// attributes on expressions are experimental, so. we do this
				#[expect(clippy::as_conversions)]
				let val = b as _;
				val
			};
		}
	};

	{
		int: signed $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0;
			one: 1;
			bits: {
				// attributes on expressions are experimental, so. we do this
				#[expect(clippy::as_conversions)]
				let val = $num::BITS as _;
				val
			};
			from_bool: b => {
				// attributes on expressions are experimental, so. we do this
				#[expect(clippy::as_conversions)]
				let val = b as _;
				val
			};
		}
	};

	{
		float: $num:ident
	} => {
		impl_num_trait_base! {
			base: $num;
			zero: 0.0;
			one: 1.0;
			bits: {
				#[expect(clippy::as_conversions)]
				let thing = (size_of::<$num>() * 8) as _;
				thing
			};
			from_bool: b => if b { 1.0 } else { 0.0 };
		}
	};

	{
		base: $num:ident;
		zero: $zero:literal;
		one: $one:literal;
		bits: $bits:expr;
		from_bool: $b:ident => $from_bool:expr;
	} => {
		impl private::Sealed for $num {}
		impl Base for $num {
			const MIN: $num = $num::MIN;
			const MAX: $num = $num::MAX;
			const ZERO: $num = $zero;
			const ONE: $num = $one;
			const BITS: $num = $bits;
			#[expect(clippy::as_conversions)]
			const BYTES: $num = size_of::<$num>() as _;
			#[expect(clippy::as_conversions)]
			const ALIGN: $num = align_of::<$num>() as _;
			#[expect(clippy::as_conversions)]
			const BITS_USIZE: usize = Self::BITS as _;
			#[expect(clippy::as_conversions)]
			const BYTES_USIZE: usize = Self::BYTES as _;
			#[expect(clippy::as_conversions)]
			const ALIGN_USIZE: usize = Self::ALIGN as _;

			#[inline(always)]
			fn from_bool($b: bool) -> $num { $from_bool }
		}
	};
}

impl_num_trait_base! {
	int: unsigned u8
}

impl_num_trait_base! {
	int: unsigned u16
}

impl_num_trait_base! {
	int: unsigned u32
}

impl_num_trait_base! {
	int: unsigned u64
}

impl_num_trait_base! {
	int: unsigned u128
}

impl_num_trait_base! {
	int: unsigned usize
}

impl_num_trait_base! {
	int: signed i8
}

impl_num_trait_base! {
	int: signed i16
}

impl_num_trait_base! {
	int: signed i32
}

impl_num_trait_base! {
	int: signed i64
}

impl_num_trait_base! {
	int: signed i128
}

impl_num_trait_base! {
	int: signed isize
}

impl_num_trait_base! {
	float: f32
}

impl_num_trait_base! {
	float: f64
}

#[cfg(test)]
mod tests {
	use crate::rust_std::assert_eq;
	use crate::rust_std::cmp::Eq;
	use crate::rust_std::fmt::Debug;
	use super::*;

	#[test]
	fn bits_and_bytes() {
		fn check<I: Base + Debug + Eq>(expected_bits: I, expected_bytes: I) {
			assert_eq!(I::BITS, expected_bits);
			assert_eq!(I::BYTES, expected_bytes);
		}

		check(8u8, 1);
		check(16u16, 2);
		check(32u32, 4);
		check(64u64, 8);
		check(128u128, 16);
		check(8i8, 1);
		check(16i16, 2);
		check(32i32, 4);
		check(64i64, 8);
		check(128i128, 16);
	}
}
