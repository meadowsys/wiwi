use super::Add;

/// Overflowing addition
pub trait AddOverflowing: Add {
	/// Performs overflowing add
	///
	/// # Examples
	///
	/// TODO
	fn add_overflowing(self, rhs: Self) -> (Self, bool);
}

macro_rules! impl_num_trait_add_overflowing {
	{ $($num:ident)* } => {
		$(
			impl AddOverflowing for $num {
				#[inline]
				fn add_overflowing(self, rhs: $num) -> ($num, bool) {
					$num::overflowing_add(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_add_overflowing! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
