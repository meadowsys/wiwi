use super::SubOverflowing;

/// Borrowing subtraction
pub trait SubBorrowing: SubOverflowing {
	/// Performs borrowing subtraction
	///
	/// # Examples
	///
	/// TODO
	#[inline]
	fn sub_borrowing(self, rhs: Self, borrow: bool) -> (Self, bool) {
		let (res, borrow1) = self.sub_overflowing(rhs);
		let (res, borrow2) = res.sub_overflowing(Self::from_bool(borrow));
		(res, borrow1 || borrow2)
	}
}

macro_rules! impl_num_trait_sub_borrowing {
	{ $($num:ident)* } => {
		$(
			impl SubBorrowing for $num {}
		)*
	}
}

impl_num_trait_sub_borrowing! {
	u8 u16 u32 u64 u128 usize
	// TODO: look in std, it's borrowing impl is different for signed ints
	// i8 i16 i32 i64 i128 isize
}
