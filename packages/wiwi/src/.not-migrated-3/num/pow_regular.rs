use super::Base;

/// Exponentiation
pub trait Pow: Base {
	fn pow_regular(self, exp: Self) -> Self;
}

macro_rules! impl_num_trait_pow {
	{ @pow $($num:ident)* } => {
		$(
			impl Pow for $num {
				#[inline]
				fn pow_regular(self, exp: $num) -> $num {
					// TODO: use saturating conversion
					$num::pow(self, exp as _)
				}
			}
		)*
	};

	{ @powf $($num:ident)* } => {
		$(
			impl Pow for $num {
				#[inline]
				fn pow_regular(self, exp: $num) -> $num {
					$num::powf(self, exp)
				}
			}
		)*
	};
}

impl_num_trait_pow! {
	@pow
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl_num_trait_pow! {
	@powf
	f32 f64
}
