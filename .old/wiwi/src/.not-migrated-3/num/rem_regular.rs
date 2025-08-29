use super::Base;

/// Modulo (the remainder operator)
pub trait Rem: Base + crate::rust_std::ops::Rem<Self, Output = Self> {
	#[inline]
	fn rem_regular(self, rhs: Self) -> Self {
		self % rhs
	}
}

macro_rules! impl_num_trait_rem {
	{ $($num:ident)* } => {
		$(
			impl Rem for $num {}
		)*
	}
}

impl_num_trait_rem! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
