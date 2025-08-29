use crate::rust_std::option::{ Option, Option::Some, Option::None };
use super::{ Base, Shr };

/// Checked right shift
pub trait ShrChecked: Shr {
	fn shr_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_shr {
	{ $($num:ident)* } => {
		$(
			impl ShrChecked for $num {
				#[inline]
				fn shr_checked(self, rhs: $num) -> Option<$num> {
					if rhs < <$num as Base>::BITS {
						// TODO: use unchecked
						Some(self.shr_regular(rhs))
					} else {
						None
					}
				}
			}
		)*
	}
}

impl_num_trait_shr! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
