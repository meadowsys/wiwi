use super::{ Base, Widening };

/// Numbers that can be "chopped in half" to two of another number type that's
/// half its size (the inverse of [`Widening`])
pub trait Narrowing<Narrow>: Base {
	/// Extend `Narrow` out into `Self` (without changing its value)
	fn widen(narrow: Narrow) -> Self;

	/// Splits `self` into its lower and upper narrow parts
	///
	/// The returned value is (low, high), ie. first element is the lower half,
	/// and the second element is the upper half, ie. little endian order.
	fn split(self) -> (Narrow, Narrow);

	/// Join the narrow lower and upper parts together to get `Self`
	fn join(n_low: Narrow, n_high: Narrow) -> Self;
}

macro_rules! impl_num_trait_narrowing {
	{ $num:ident $narrow:ident $($next_narrow:ident $($rest:ident)*)? } => {
		impl Narrowing<$narrow> for $num {
			#[inline]
			fn split(self) -> ($narrow, $narrow) {
				<$narrow as Widening<$num>>::split(self)
			}

			#[inline(always)]
			fn widen(narrow: $narrow) -> $num {
				<$narrow as Widening<$num>>::widen(narrow)
			}

			#[inline]
			fn join(n_low: $narrow, n_high: $narrow) -> $num {
				<$narrow as Widening<$num>>::join(n_low, n_high)
			}
		}

		$(impl_num_trait_narrowing! { $narrow $next_narrow $($rest)* })?
	}
}

impl_num_trait_narrowing! { u128 u64 u32 u16 u8 }

#[cfg(target_pointer_width = "64")]
impl_num_trait_narrowing! { u128 usize u32 }

#[cfg(target_pointer_width = "32")]
impl_num_trait_narrowing! { u64 usize u16 }

#[cfg(target_pointer_width = "16")]
impl_num_trait_narrowing! { u32 usize u8 }
