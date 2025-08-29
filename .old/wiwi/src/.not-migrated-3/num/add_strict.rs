use super::Add;

/// Strict addition
pub trait AddStrict: Add {
	fn add_strict(self, rhs: Self) -> Self;
}
