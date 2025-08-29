use super::Base;

/// Bitwise AND
pub trait And: Base + crate::rust_std::ops::BitAnd<Self, Output = Self> {
	#[inline]
	fn and_regular(self, rhs: Self) -> Self {
		self & rhs
	}
}

macro_rules! impl_num_trait_and {
	{ $($num:ident)* } => {
		$(
			impl And for $num {}
		)*
	}
}

impl_num_trait_and! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
