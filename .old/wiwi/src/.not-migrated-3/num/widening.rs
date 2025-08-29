use super::Base;

/// Numbers that can be "widened" to another number type that's double its size
pub trait Widening<Wide>: Base {
	/// Extend `self` out into the widened type (without changing its value)
	fn widen(self) -> Wide;

	/// Splits the wide integer into its lower and upper `Self` parts
	///
	/// The returned value is (low, high), ie. first element is the lower half,
	/// and the second element is the upper half, ie. little endian order.
	fn split(wide: Wide) -> (Self, Self);

	/// Joins `self` with an upper value, to give a widened value
	fn join(self, n_high: Self) -> Wide;
}

macro_rules! impl_num_trait_widening {
	{ $num:ident $wide:ident $($next_wide:ident $($rest:ident)*)? } => {
		impl Widening<$wide> for $num {
			#[expect(clippy::as_conversions)]
			#[inline(always)]
			fn widen(self) -> $wide { self as _ }

			#[expect(clippy::as_conversions)]
			#[inline]
			fn split(wide: $wide) -> ($num, $num) {
				(wide as _, (wide >> <$num as Base>::BITS) as _)
			}

			#[inline]
			fn join(self, n_high: $num) -> $wide {
				let l = <$num as Widening<$wide>>::widen(self);
				let h = <$num as Widening<$wide>>::widen(n_high);
				l | (h << <$num as Base>::BITS)
			}
		}

		$(impl_num_trait_widening! { $wide $next_wide $($rest)* })?
	}
}

impl_num_trait_widening! { u8 u16 u32 u64 u128 }

#[cfg(target_pointer_width = "64")]
impl_num_trait_widening! { u32 usize u128 }

#[cfg(target_pointer_width = "32")]
impl_num_trait_widening! { u16 usize u64 }

#[cfg(target_pointer_width = "16")]
impl_num_trait_widening! { u8 usize u32 }
