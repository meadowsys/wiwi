use super::Pow;

/// Floating point exponentiation
pub trait PowFloat: Pow {
	fn pow_float(self, exp: Self) -> Self;
}
