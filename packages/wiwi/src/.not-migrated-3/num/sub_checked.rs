use crate::rust_std::option::Option;
use super::Sub;

pub trait SubChecked: Sub {
	fn sub_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_sub_checked {
	{ $($num:ident)* } => {
		$(
			impl SubChecked for $num {
				#[inline]
				fn sub_checked(self, rhs: $num) -> Option<$num> {
					$num::checked_sub(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_sub_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
