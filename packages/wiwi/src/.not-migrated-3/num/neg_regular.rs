use super::Base;

/// Negation
pub trait Neg: Base + crate::rust_std::ops::Neg<Output = Self> {
	#[inline]
	fn neg_regular(self) -> Self {
		-self
	}
}

macro_rules! impl_num_trait_neg {
	{ $($num:ident)* } => {
		$(
			impl Neg for $num {}
		)*
	}
}

impl_num_trait_neg! {
	i8 i16 i32 i64 i128 isize
	f32 f64
}
