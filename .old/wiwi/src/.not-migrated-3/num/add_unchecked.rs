use super::Add;

/// Unchecked addition
pub trait AddUnchecked: Add {
	unsafe fn add_unchecked(self, rhs: Self) -> Self;
}
