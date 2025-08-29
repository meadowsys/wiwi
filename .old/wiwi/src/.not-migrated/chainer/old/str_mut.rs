#[repr(transparent)]
pub struct StrMutChain<'h> {
	inner: &'h mut str
}

impl<'h> From<&'h mut str> for StrMutChain<'h> {
	fn from(value: &'h mut str) -> Self {
		Self { inner: value }
	}
}
