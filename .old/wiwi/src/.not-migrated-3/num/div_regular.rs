use super::Base;

/// Division
pub trait Div: Base + crate::rust_std::ops::Div<Self, Output = Self> {
	/// Performs regular division
	///
	/// It does the same as what the `/` operator does
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	fn div_regular(self, rhs: Self) -> Self {
		self / rhs
	}
}

macro_rules! impl_num_trait_div {
	{ $($num:ident)* } => {
		$(
			impl Div for $num {}
		)*
	}
}

impl_num_trait_div! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
	f32 f64
}
