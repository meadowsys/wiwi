use crate::rust_std::option::Option;
use super::Mul;

pub trait MulChecked: Mul {
	fn mul_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_mul_checked {
	{ $($num:ident)* } => {
		$(
			impl MulChecked for $num {
				#[inline]
				fn mul_checked(self, rhs: $num) -> Option<$num> {
					$num::checked_mul(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_mul_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
