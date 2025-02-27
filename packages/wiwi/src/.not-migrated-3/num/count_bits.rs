use super::Base;

pub trait CountBits: Base {
	fn count_zeros(self) -> Self;
	fn count_ones(self) -> Self;
	fn leading_zeros(self) -> Self;
	fn leading_ones(self) -> Self;
	fn trailing_zeros(self) -> Self;
	fn trailing_ones(self) -> Self;
}

macro_rules! impl_num_trait_count_bits {
	{ $($num:ident)* } => {
		$(
			impl CountBits for $num {
				#[expect(clippy::as_conversions)]
				#[inline]
				fn count_zeros(self) -> $num {
					$num::count_zeros(self) as _
				}

				#[expect(clippy::as_conversions)]
				#[inline]
				fn count_ones(self) -> $num {
					$num::count_ones(self) as _
				}

				#[expect(clippy::as_conversions)]
				#[inline]
				fn leading_zeros(self) -> $num {
					$num::leading_zeros(self) as _
				}

				#[expect(clippy::as_conversions)]
				#[inline]
				fn leading_ones(self) -> $num {
					$num::leading_ones(self) as _
				}

				#[expect(clippy::as_conversions)]
				#[inline]
				fn trailing_zeros(self) -> $num {
					$num::trailing_zeros(self) as _
				}

				#[expect(clippy::as_conversions)]
				#[inline]
				fn trailing_ones(self) -> $num {
					$num::trailing_ones(self) as _
				}
			}
		)*
	}
}

impl_num_trait_count_bits! {
	u8 u16 u32 u64 u128 usize
	i8 i16 i32 i64 i128 isize
}
