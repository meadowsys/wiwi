use super::Base;

/// Right shift
pub trait Shr: Base + crate::rust_std::ops::Shr<Self, Output = Self> {
	#[inline]
	fn shr_regular(self, rhs: Self) -> Self {
		self >> rhs
	}
}

macro_rules! impl_num_trait_shr {
	{ $($num:ident)* } => {
		$(
			impl Shr for $num {}
		)*
	}
}

impl_num_trait_shr! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
