use crate::rust_std::option::Option;
use super::Add;

/// Checked addition
pub trait AddChecked: Add {
	fn add_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_add_checked {
	{ $($num:ident)* } => {
		$(
			impl AddChecked for $num {
				#[inline]
				fn add_checked(self, rhs: $num) -> Option<$num> {
					$num::checked_add(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_add_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
