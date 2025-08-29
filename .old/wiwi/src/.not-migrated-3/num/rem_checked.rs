use crate::rust_std::option::Option;
use super::Rem;

pub trait RemChecked: Rem {
	fn rem_checked(self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_num_trait_rem_checked {
	{ $($num:ident)* } => {
		$(
			impl RemChecked for $num {
				#[inline]
				fn rem_checked(self, rhs: $num) -> Option<$num> {
					$num::checked_rem(self, rhs)
				}
			}
		)*
	}
}

impl_num_trait_rem_checked! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
