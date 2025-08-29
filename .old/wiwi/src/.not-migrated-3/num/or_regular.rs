use super::Base;

/// Bitwise OR
pub trait Or: Base + crate::rust_std::ops::BitOr<Self, Output = Self> {
	#[inline]
	fn or_regular(self, rhs: Self) -> Self {
		self | rhs
	}
}

macro_rules! impl_num_trait_or {
	{ $($num:ident)* } => {
		$(
			impl Or for $num {}
		)*
	}
}

impl_num_trait_or! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
