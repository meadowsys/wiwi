use crate::rust_std::option::Option;
use super::Base;

/// Checked negation
pub trait NegChecked: Base {
	fn neg_checked(self) -> Option<Self>;
}

macro_rules! impl_num_trait_neg_checked {
	{ $($num:ident)* } => {
		$(
			impl NegChecked for $num {
				#[inline]
				fn neg_checked(self) -> Option<$num> {
					$num::checked_neg(self)
				}
			}
		)*
	}
}

impl_num_trait_neg_checked! {
	i8 i16 i32 i64 i128 isize
	u8 u16 u32 u64 u128 usize
}
