use super::Mul;

/// Unchecked multiplication
pub trait MulUnchecked: Mul {
	/// Performs unchecked multiplication
	///
	/// # Safety
	///
	/// The result of the multiplication should not overflow.
	///
	/// # Examples
	///
	/// TODO
	unsafe fn mul_unchecked(self, rhs: Self) -> Self;
}

macro_rules! impl_num_trait_mul_unchecked {
	{ $($num:ident)* } => {
		$(
			impl MulUnchecked for $num {
				#[inline]
				unsafe fn mul_unchecked(self, rhs: $num) -> $num {
					// TODO: use actually unchecked mul when it's been longer in stable
					self * rhs
				}
			}
		)*
	}
}

impl_num_trait_mul_unchecked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
