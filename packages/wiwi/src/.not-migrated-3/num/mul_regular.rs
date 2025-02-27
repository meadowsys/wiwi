use super::Base;

/// Multiplication
pub trait Mul: Base + crate::rust_std::ops::Mul<Self, Output = Self> {
	/// Performs plain multiplication
	///
	/// It does the same as what the `*` operator does
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	fn mul_regular(self, rhs: Self) -> Self {
		self * rhs
	}
}

macro_rules! impl_num_trait_mul {
	{ $($num:ident)* } => {
		$(
			impl Mul for $num {}
		)*
	}
}

impl_num_trait_mul! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
