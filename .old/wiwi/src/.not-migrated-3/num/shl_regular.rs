use super::Base;

/// Left shift
pub trait Shl: Base + crate::rust_std::ops::Shl<Self, Output = Self> {
	#[inline]
	fn shl_regular(self, rhs: Self) -> Self {
		self << rhs
	}
}

macro_rules! impl_num_trait_shl {
	{ $($num:ident)* } => {
		$(
			impl Shl for $num {}
		)*
	}
}

impl_num_trait_shl! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
