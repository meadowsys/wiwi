#[repr(transparent)]
pub struct StringMutChain<'h> {
	inner: &'h mut String
}

impl<'h> From<&'h mut String> for StringMutChain<'h> {
	fn from(value: &'h mut String) -> Self {
		Self { inner: value }
	}
}
