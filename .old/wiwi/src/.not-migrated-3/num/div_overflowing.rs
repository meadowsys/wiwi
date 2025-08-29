use super::Div;

/// Overflowing division
pub trait DivOverflowing: Div {
	/// Performs overflowing division
	///
	/// # Examples
	///
	/// TODO
	fn div_overflowing(self, rhs: Self) -> (Self, bool);
}

macro_rules! impl_num_trait_div_overflowing {
	{ $($num:ident)* } => {
		$(
			impl DivOverflowing for $num {
				#[inline]
				fn div_overflowing(self, rhs: $num) -> ($num, bool) {
					$num::overflowing_div(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_div_overflowing! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
