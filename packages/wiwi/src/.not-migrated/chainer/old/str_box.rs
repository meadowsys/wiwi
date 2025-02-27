#[repr(transparent)]
pub struct StrBoxChain {
	inner: Box<str>
}

impl From<Box<str>> for StrBoxChain {
	fn from(value: Box<str>) -> Self {
		Self { inner: value }
	}
}
