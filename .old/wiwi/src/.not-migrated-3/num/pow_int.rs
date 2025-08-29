use super::Pow;

/// Integer exponentiation
pub trait PowI: Pow {
	fn pow_int(self, exp: u32) -> Self;
}

macro_rules! impl_num_trait_powi {
	{ @pow $($num:ident)* } => {
		$(
			impl PowI for $num {
				#[inline]
				fn pow_int(self, exp: u32) -> $num {
					// TODO: use saturating conversion
					$num::pow(self, exp as _)
				}
			}
		)*
	};

	{ @powi $($num:ident)* } => {
		$(
			impl PowI for $num {
				#[inline]
				fn pow_int(self, exp: u32) -> $num {
					$num::powi(self, exp)
				}
			}
		)*
	};
}

impl_num_trait_powi! {
	@pow
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}

impl_num_trait_powi! {
	@powi
	f32 f64
}
