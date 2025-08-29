use super::Sub;

/// Overflowing subtraction
pub trait SubOverflowing: Sub {
	/// Performs overflowing subtraction
	///
	/// # Examples
	///
	/// TODO
	fn sub_overflowing(self, rhs: Self) -> (Self, bool);
}

macro_rules! impl_num_trait_sub_overflowing {
	{ $($num:ident)* } => {
		$(
			impl SubOverflowing for $num {
				#[inline]
				fn sub_overflowing(self, rhs: $num) -> ($num, bool) {
					$num::overflowing_sub(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_sub_overflowing! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
