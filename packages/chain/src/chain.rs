#[must_use = "a chain takes ownership of itself, performs the action, then returns itself again"]
pub struct Chain<T> {
	inner: T
}

impl<T> Chain<T> {
	#[inline]
	pub fn from_inner(inner: T) -> Self {
		Self { inner }
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.inner
	}

	#[inline]
	pub fn as_inner(&self) -> &T {
		&self.inner
	}

	#[inline]
	pub fn as_inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	#[inline]
	pub fn as_inner_chain(&self) -> Chain<&T> {
		self.as_inner().into_chain()
	}

	#[inline]
	pub fn as_inner_mut_chain(&mut self) -> Chain<&mut T> {
		self.as_inner_mut().into_chain()
	}

	#[inline]
	pub fn with_inner<F>(self, f: F) -> Self
	where
		F: FnOnce(&T)
	{
		f(self.as_inner());
		self
	}

	#[inline]
	pub fn with_inner_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut T)
	{
		f(self.as_inner_mut());
		self
	}
}

pub trait ChainInner: Sized {
	#[inline]
	fn from_chain(chain: Chain<Self>) -> Self {
		chain.into_inner()
	}

	#[inline]
	fn into_chain(self) -> Chain<Self> {
		Chain::from_inner(self)
	}
}

impl<T> ChainInner for T {}

pub(crate) trait ChainInnerType {
	type Inner;
}

impl<T> ChainInnerType for Chain<T> {
	type Inner = T;
}
