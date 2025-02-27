use super::Div;

/// Integer division
pub trait DivInt: Div {
	/// Performs integer division
	///
	/// This means the division operation will perform division and truncate (not
	/// round) any fractional component, as you would expect with integers
	#[inline]
	fn div_int(self, rhs: Self) -> Self {
		self / rhs
	}
}

macro_rules! impl_num_trait_div_int_ints {
	{ $($num:ident)* } => {
		$(
			impl DivInt for $num {}
		)*
	}
}

impl_num_trait_div_int_ints! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

macro_rules! impl_num_trait_div_int_floats {
	{ $($num:ident)* } => {
		$(
			impl DivInt for $num {
				#[inline]
				fn div_int(self, rhs: $num) -> $num {
					$num::floor(self / rhs)
				}
			}
		)*
	}
}

impl_num_trait_div_int_floats! {
	f32 f64
}
