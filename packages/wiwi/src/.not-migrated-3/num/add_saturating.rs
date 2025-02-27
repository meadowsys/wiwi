use super::Add;

/// Saturating addition
pub trait AddSaturating: Add {
	fn add_saturating(self, rhs: Self) -> Self;
}
