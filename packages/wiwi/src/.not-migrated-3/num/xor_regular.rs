use super::Base;

/// Bitwise XOR
pub trait Xor: Base + crate::rust_std::ops::BitXor<Self, Output = Self> {
	#[inline]
	fn xor_regular(self, rhs: Self) -> Self {
		self ^ rhs
	}
}

macro_rules! impl_num_trait_xor {
	{ $($num:ident)* } => {
		$(
			impl Xor for $num {}
		)*
	}
}

impl_num_trait_xor! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
