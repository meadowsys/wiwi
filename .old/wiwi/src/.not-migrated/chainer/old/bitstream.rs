use crate::bitstream::Encoder;
use super::VecChain;

#[repr(transparent)]
pub struct BitstreamEncoderChain {
	inner: Encoder
}

// TODO: eventually ref/mut versions

impl BitstreamEncoderChain {
	pub fn new() -> Self {
		Encoder::new().into()
	}

	pub fn with_output_capacity(capacity: usize) -> Self {
		Encoder::with_output_capacity(capacity).into()
	}
}

impl BitstreamEncoderChain {
	pub fn into_bytes(self) -> Vec<u8> {
		self.inner.into_bytes()
	}

	pub fn into_bytes_chainer(self) -> VecChain<u8> {
		self.into_bytes().into()
	}
}

impl BitstreamEncoderChain {
	pub unsafe fn write_bits_u8_unchecked(mut self, num_bits: usize, bits: u8) -> Self {
		self.inner.write_bits_u8_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u16_unchecked(mut self, num_bits: usize, bits: u16) -> Self {
		self.inner.write_bits_u16_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u32_unchecked(mut self, num_bits: usize, bits: u32) -> Self {
		self.inner.write_bits_u32_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u64_unchecked(mut self, num_bits: usize, bits: u64) -> Self {
		self.inner.write_bits_u64_unchecked(num_bits, bits);
		self
	}

	pub unsafe fn write_bits_u128_unchecked(mut self, num_bits: usize, bits: u128) -> Self {
		self.inner.write_bits_u128_unchecked(num_bits, bits);
		self
	}
}

impl From<Encoder> for BitstreamEncoderChain {
	fn from(value: Encoder) -> Self {
		Self { inner: value }
	}
}
