use crate::num_traits::AddCarrying;
use std::mem::MaybeUninit;

pub fn add_overflowing<I, const BYTES: usize>(
	int1: &[I; BYTES],
	int2: &[I; BYTES]
) -> ([I; BYTES], bool)
where
	I: AddCarrying
{
	unsafe {
		let int1_ptr = int1.as_ptr();
		let int2_ptr = int2.as_ptr();

		let mut result = MaybeUninit::<[I; BYTES]>::uninit();
		let result_ptr = result.as_mut_ptr().cast::<I>();

		let mut carry = false;

		for i in 0..BYTES {
			let i1 = (*int1_ptr.add(i)).clone();
			let i2 = (*int2_ptr.add(i)).clone();

			let (r, c) = I::add_carrying(i1, i2, carry);

			result_ptr.add(i).write(r);
			carry = c;
		}

		(result.assume_init(), carry)
	}
}

#[cfg(test)]
mod tests {
	use crate::num_traits::AddOverflowing;
	use super::*;
	use rand::{ RngCore, Rng, thread_rng };
	use std::mem::transmute;

	#[test]
	fn overflowing_random_u32() {
		for _ in 0..1000 {
			let orig_int1 = thread_rng().next_u32();
			let orig_int2 = thread_rng().next_u32();
			let expected = orig_int1.add_overflowing(orig_int2);

			let int1 = orig_int1.to_le_bytes();
			let int2 = orig_int2.to_le_bytes();

			let (res, overflow) = add_overflowing(&int1, &int2);
			let res = u32::from_le_bytes(res);

			assert_eq!(expected, (res, overflow));
		}
	}
}
