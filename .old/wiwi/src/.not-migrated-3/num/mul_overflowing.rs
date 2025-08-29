use super::Mul;

/// Overflowing multiplication
pub trait MulOverflowing: Mul {
	/// Performs overflowing multiplication
	///
	/// # Examples
	///
	/// TODO
	fn mul_overflowing(self, rhs: Self) -> (Self, bool);
}

macro_rules! impl_num_trait_mul_overflowing {
	{ $($num:ident)* } => {
		$(
			impl MulOverflowing for $num {
				#[inline]
				fn mul_overflowing(self, rhs: $num) -> ($num, bool) {
					$num::overflowing_mul(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_mul_overflowing! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
