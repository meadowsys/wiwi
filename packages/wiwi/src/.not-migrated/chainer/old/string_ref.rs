#[repr(transparent)]
pub struct StringRefChain<'h> {
	inner: &'h String
}

impl<'h> From<&'h String> for StringRefChain<'h> {
	fn from(value: &'h String) -> Self {
		Self { inner: value }
	}
}
