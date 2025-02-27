use super::Base;

/// Bitwise NOT
pub trait Not: Base + crate::rust_std::ops::Not<Output = Self> {
	#[inline]
	fn not_regular(self) -> Self {
		!self
	}
}

macro_rules! impl_num_trait_not {
	{ $($num:ident)* } => {
		$(
			impl Not for $num {}
		)*
	}
}

impl_num_trait_not! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
