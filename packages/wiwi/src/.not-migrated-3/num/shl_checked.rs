use crate::rust_std::option::{ Option, Option::Some, Option::None };
use super::{ Base, Shl };

/// Checked left shift
pub trait ShlChecked: Shl {
	fn shl_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_shl {
	{ $($num:ident)* } => {
		$(
			impl ShlChecked for $num {
				#[inline]
				fn shl_checked(self, rhs: $num) -> Option<$num> {
					// std uses u32, but we have ::BITS that is Self, so it's
					// probably more efficient to reimplement and not cast for std
					// TODO: unchecked shl
					if rhs < <$num as Base>::BITS {
						Some(self.shl_regular(rhs))
					} else {
						None
					}
				}
			}
		)*
	}
}

impl_num_trait_shl! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
