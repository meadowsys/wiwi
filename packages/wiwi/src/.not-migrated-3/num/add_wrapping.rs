use super::Add;

/// Wrapping addition
pub trait AddWrapping: Add {
	fn add_wrapping(self, rhs: Self) -> Self;
}
