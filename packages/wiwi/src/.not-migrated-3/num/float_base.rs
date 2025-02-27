use super::Base;

/// Additional base for all floating point numbers
pub trait FloatBase: Base {
	const RADIX: Self;
	const MANTISSA_DIGITS: Self;
	const DIGITS: Self;
	const EPSILON: Self;
	const MIN_POSITIVE: Self;
	const MIN_EXP: Self;
	const MAX_EXP: Self;
	const MIN_10_EXP: Self;
	const MAX_10_EXP: Self;
	const NAN: Self;
	const INFINITY: Self;
	const NEG_INFINITY: Self;
}

macro_rules! impl_num_trait_float_base {
	{ $($num:ident)* } => {
		$(
			impl FloatBase for $num {
				const RADIX: $num = $num::RADIX as _;
				const MANTISSA_DIGITS: $num = $num::MANTISSA_DIGITS as _;
				const DIGITS: $num = $num::DIGITS as _;
				const EPSILON: $num = $num::EPSILON;
				const MIN_POSITIVE: $num = $num::MIN_POSITIVE;
				const MIN_EXP: $num = $num::MIN_EXP as _;
				const MAX_EXP: $num = $num::MAX_EXP as _;
				const MIN_10_EXP: $num = $num::MIN_10_EXP as _;
				const MAX_10_EXP: $num = $num::MAX_10_EXP as _;
				const NAN: $num = $num::NAN;
				const INFINITY: $num = $num::INFINITY;
				const NEG_INFINITY: $num = $num::NEG_INFINITY;
			}
		)*
	}
}

impl_num_trait_float_base! {
	f32 f64
}
