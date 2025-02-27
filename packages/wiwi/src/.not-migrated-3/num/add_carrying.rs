use super::AddOverflowing;

/// Carrying addition
pub trait AddCarrying: AddOverflowing {
	/// Performs carrying add
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	fn add_carrying(self, rhs: Self, carry: bool) -> (Self, bool) {
		let (res, carry1) = self.add_overflowing(rhs);
		let (res, carry2) = res.add_overflowing(Self::from_bool(carry));
		(res, carry1 || carry2)
	}
}

macro_rules! impl_num_trait_add_carrying {
	{ $($num:ident)* } => {
		$(
			impl AddCarrying for $num {}
		)*
	}
}

impl_num_trait_add_carrying! {
	u8 u16 u32 u64 u128 usize
	// TODO: look in std, it's carrying impl is different for signed ints
	// i8 i16 i32 i64 i128 isize
}
