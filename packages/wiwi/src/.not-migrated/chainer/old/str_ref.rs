#[repr(transparent)]
pub struct StrRefChain<'h> {
	inner: &'h str
}

impl<'h> From<&'h str> for StrRefChain<'h> {
	fn from(value: &'h str) -> Self {
		Self { inner: value }
	}
}
