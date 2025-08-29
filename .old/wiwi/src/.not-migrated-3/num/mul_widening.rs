use super::{ Mul, MulUnchecked, Widening };

/// Widening multiplication
pub trait MulWidening: Mul {
	/// Performs widening multiplication, returning the result in two parts:
	/// `(low, high)`
	///
	/// This will never overflow
	///
	/// # Examples
	///
	/// TODO
	fn mul_widening(self, rhs: Self) -> (Self, Self);
}

macro_rules! impl_num_trait_mul_widening {
	{ $num:ident $wide:ident $($next_wide:ident $($rest:ident)*)? } => {
		impl MulWidening for $num {
			#[inline]
			fn mul_widening(self, rhs: $num) -> ($num, $num) {
				// SAFETY: we widen the numbers (eg. u64 -> u128), then multiply,
				// so they have twice the bit space to multiply into, should the
				// original multiplication have overflowed
				let widened = unsafe {
					$wide::mul_unchecked(
						<$num as Widening<$wide>>::widen(self),
						<$num as Widening<$wide>>::widen(rhs)
					)
				};
				<$num as Widening<$wide>>::split(widened)
			}
		}

		$(impl_num_trait_mul_widening! { $wide $next_wide $($rest)* })?
	}
}

impl_num_trait_mul_widening! {
	u8 u16 u32 u64 u128
}

#[cfg(target_pointer_width = "64")]
impl_num_trait_mul_widening! { usize u128 }

#[cfg(target_pointer_width = "32")]
impl_num_trait_mul_widening! { usize u64 }

#[cfg(target_pointer_width = "16")]
impl_num_trait_mul_widening! { usize u32 }

// TODO: needs the big num stuff I think
// /// Widening mul is done by splitting the u128 values into two u64's each, then
// /// multiplying those together using wiwi's `int` feature widening multiplication
// /// implementation
// impl MulWidening for u128 {
// 	#[inline]
// 	fn mul_widening(self, rhs: u128) -> (u128, u128) {
// 		// yeah, the widening_mul impl of int does indeed depend on MulWidening trait,
// 		// but it won't cause an infinite recursion loop, since it calls
// 		// int::widening_mul with [u64; 2], and MulWidening for u64 doesn't depend
// 		// on int::widening_mul
//
// 		let (lhs1, lhs2) = <u128 as Narrowing<u64>>::split(self);
// 		let (rhs1, rhs2) = <u128 as Narrowing<u64>>::split(rhs);
//
// 		let [[lhs1, lhs2], [rhs1, rhs2]] = crate::int::mul::mul_widening(
// 			&[lhs1, lhs2],
// 			&[rhs1, rhs2]
// 		);
//
// 		let lhs = <u128 as Narrowing<u64>>::join(lhs1, lhs2);
// 		let rhs = <u128 as Narrowing<u64>>::join(rhs1, rhs2);
//
// 		(lhs, rhs)
// 	}
// }

#[cfg(test)]
mod tests {
	// use super::*;
	//
	// #[test]
	// fn u128_widening_mul() {
	// 	// effectively just testing that we split/reassemble everything properly
	// 	// (like, in the right order etc), since widening_mul from int mod is
	// 	// tested already
	// 	let i1 = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffffu128;
	// 	let i2 = 0x1_0000_0000u128;
	// 	let expected_res = (0xffff_ffff_ffff_ffff_ffff_ffff_0000_0000u128, 0xffff_ffffu128);
	//
	// 	let res = i1.mul_widening(i2);
	// 	assert_eq!(res, expected_res);
	// }
}
