use core::hash::{ BuildHasher, Hash, Hasher };

pub struct DefaultHashBuilder {
	inner: ahash::RandomState
}

impl DefaultHashBuilder {
	#[inline]
	pub fn new() -> Self {
		Self { inner: ahash::RandomState::new() }
	}
}

impl BuildHasher for DefaultHashBuilder {
	type Hasher = DefaultHasher;

	#[inline]
	fn build_hasher(&self) -> DefaultHasher {
		DefaultHasher { inner: self.inner.build_hasher() }
	}

	#[expect(
		clippy::manual_hash_one,
		reason = "inner might have overridden it for whatever reason, we should use it"
	)]
	#[inline]
	fn hash_one<T: Hash>(&self, x: T) -> u64
	where
		Self: Sized
	{
		let mut hasher = self.build_hasher();
		x.hash(&mut hasher);
		hasher.finish()
	}
}

impl Default for DefaultHashBuilder {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

pub struct DefaultHasher {
	inner: ahash::AHasher
}

impl Hasher for DefaultHasher {
	fn finish(&self) -> u64 {
		self.inner.finish()
	}

	forward_hasher_to_inner! {
		fn write(&mut self, bytes: &[u8])(bytes);
		fn write_u8(&mut self, i: u8)(i);
		fn write_u16(&mut self, i: u16)(i);
		fn write_u32(&mut self, i: u32)(i);
		fn write_u64(&mut self, i: u64)(i);
		fn write_u128(&mut self, i: u128)(i);
		fn write_usize(&mut self, i: usize)(i);
		fn write_i8(&mut self, i: i8)(i);
		fn write_i16(&mut self, i: i16)(i);
		fn write_i32(&mut self, i: i32)(i);
		fn write_i64(&mut self, i: i64)(i);
		fn write_i128(&mut self, i: i128)(i);
		fn write_isize(&mut self, i: isize)(i);
		#[cfg(feature = "nightly")]
		fn write_length_prefix(&mut self, len: usize)(len);
		#[cfg(feature = "nightly")]
		fn write_str(&mut self, s: &str)(s);
	}
}

macro_rules! forward_hasher_to_inner {
	(
		$(
			$(#[$meta:meta])*
			fn $fn:ident
			(&mut self $($params:tt)*)
			($($args:tt)*);
		)*
	) => {
		$(
			$(#[$meta])*
			fn $fn(&mut self $($params)*) {
				self.inner.$fn($($args)*)
			}
		)*
	}
}
use forward_hasher_to_inner;
