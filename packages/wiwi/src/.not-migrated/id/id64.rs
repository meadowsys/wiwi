use crate::num_traits::*;
use crate::rand::ThreadLocalChaCha8Rng;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::num::NonZeroU64;

/// 46 bits gives space for >2000years with millisecond precision.
/// 18 bits remaining after this
///
/// This goes out to about `zo 24 nov 4199 01:22:57 UTC`
const TIMESTAMP_SHIFT: u8 = 64 - 46;

/// to make way for 4 lower bits for the random component. Keep count in the next
/// 14 bits after the timestamp. 14 bits gives 16384 IDs/ms
const COUNT_SHIFT: u8 = 4;

/// one more than 14 lower bits filled. count must be / will always be below
/// this value
const MAX_COUNT: u16 = 1 << 14;

/// keep only 4 lower bits of random byte.
const RANDOM_COMPONENT_MASK: u8 = 0b1111;

/// When storing as an signed int, we flip the top bit (two's complement), so
/// sorting numerically by this signed int representation of the ID will still
/// work the same.
const TOP_BIT: u64 = 1 << (u64::BITS - 1);

/// counting from the most significant to least significant bit, bits 1 to 64:
///
/// - (1-46) 46 bits for timestamp (this is >2000years with millisecond precision,
///   up to year 4199)
/// - (47-60) 14 bits for increment (this is max 16384 IDs/ms, or about 16M IDs/s)
/// - (61-64) last 4 bits for randomness (so IDs within one ms aren't just increments)
///
/// IDs generated from one single instance of this generator are guaranteed to
/// be unique and monotonically increasing.
pub struct IDGenerator64 {
	/// unix epoch time
	last_generated_time: u64,
	// 14 bits fits in u32 (duh)
	count: u16
}

impl IDGenerator64 {
	#[inline]
	// no
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		// if, _somehow_, the first ID is indeed generated at UNIX time 0,
		// this initial count value of 1 will mean that ID is not zero. However,
		// in practical use, the first ID generated will have last_generated_time
		// overwritten with something bigger, so this initial value won't matter.
		let count = 1;

		let last_generated_time = 0;

		Self { last_generated_time, count }
	}

	#[inline]
	// no
	#[allow(clippy::should_implement_trait)]
	pub fn next(&mut self) -> Option<GeneratedID64> {
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("we are before 01 jan 1970 lol?")
			.as_millis()
			.into_u64_lossy();

		if now > self.last_generated_time {
			self.last_generated_time = now;
			self.count = 0;
		}

		(self.count < MAX_COUNT).then(|| {
			let now = self.last_generated_time << TIMESTAMP_SHIFT;
			let random = (ThreadLocalChaCha8Rng::gen_rand::<u8>() & RANDOM_COMPONENT_MASK).into_u64();

			// guaranteed to fit within 14 bits, as checked by
			// bool statement before this closure
			let increment = (self.count << COUNT_SHIFT).into_u64();
			self.count += 1;

			// SAFETY: generated ID is always not zero, see comment in
			// Self::new for explanation
			let unsigned = unsafe { NonZeroU64::new_unchecked(now | random | increment) };
			GeneratedID64 { unsigned }
		})
	}
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeneratedID64 {
	/// [`NonZeroU64`] to allow for null ptr optimisation
	unsigned: NonZeroU64
}

impl GeneratedID64 {
	#[inline]
	pub fn unix_time(&self) -> u64 {
		self.unsigned.get() >> TIMESTAMP_SHIFT
	}

	#[inline]
	pub fn as_signed(&self) -> i64 {
		unsigned_to_signed(self.unsigned.get())
	}

	#[inline]
	pub fn as_unsigned(&self) -> u64 {
		self.unsigned.get()
	}

	#[inline]
	pub fn as_unsigned_nonzero(&self) -> NonZeroU64 {
		self.unsigned
	}

	/// # Safety
	///
	/// Given signed value must not be isize::MIN (not possible in regular use,
	/// assuming the ID is indeed valid)
	#[inline]
	pub unsafe fn from_signed_unchecked(signed: i64) -> Self {
		let unsigned = NonZeroU64::new_unchecked(signed_to_unsigned(signed));
		Self { unsigned }
	}

	/// # Safety
	///
	/// Given unsigned value must not be zero (not possible in regular use,
	/// assuming the ID is indeed valid)
	#[inline]
	pub unsafe fn from_unsigned_unchecked(unsigned: u64) -> Self {
		let unsigned = NonZeroU64::new_unchecked(unsigned);
		Self { unsigned }
	}

	#[inline]
	pub fn from_signed(signed: i64) -> Option<Self> {
		NonZeroU64::new(signed_to_unsigned(signed))
			.map(|unsigned| Self { unsigned })
	}

	#[inline]
	pub fn from_unsigned(unsigned: u64) -> Option<Self> {
		NonZeroU64::new(unsigned)
			.map(|unsigned| Self { unsigned })
	}
}

impl GeneratedID64 {
	const ALPHANUMERIC_ALPHABET: &'static [u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

	pub fn to_alphanumeric_string(&self) -> String {
		// (2 ^ 64) / (62 ^ 11) roughly equals 0.35, which,
		// output here will always be 11 or less chars
		let mut string = String::with_capacity(11);
		// SAFETY: we will only push chars from the char table,
		// which only contains ASCII characters
		let string_vec = unsafe { string.as_mut_vec() };
		let mut val = self.unsigned.get();

		while val > 0 {
			string_vec.push(Self::ALPHANUMERIC_ALPHABET[(val % 62).into_usize()]);
			val /= 62;
		}

		debug_assert!(string.len() <= 11);
		string
	}

	pub fn from_alphanumeric_string(s: &str) -> Option<Self> {
		// stringified cannot be more than 11 chars
		// (see `to_alphanumeric_string`)
		if s.len() > 11 { return None }

		let mut result = 0u128;

		// if s were to contain chars with 2 or more bytes
		// its invalid anyways
		for c in s.bytes().rev() {
			let byte = match c {
				// 0-9
				b'0'..=b'9' => { c - b'0' }
				// 10-35
				b'A'..=b'Z' => { (c - b'A') + 10 }
				// 36-61
				b'a'..=b'z' => { (c - b'a') + 36 }
				_ => { return None }
			};

			result *= 62;
			result += byte.into_u128();
		}

		if result > u64::MAX.into_u128() { return None }
		let result = NonZeroU64::new(result.into_u64_lossy())?;

		Some(Self { unsigned: result })
	}
}

#[inline]
fn unsigned_to_signed(unsigned: u64) -> i64 {
	(unsigned ^ TOP_BIT).into_i64_lossy()
}

#[inline]
fn signed_to_unsigned(signed: i64) -> u64 {
	(signed.into_u64_lossy()) ^ TOP_BIT
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn signed_unsigned_util_fns_sanity_check() {
		let things = [
			(u64::MIN, i64::MIN),
			(u64::MAX, i64::MAX)
		];

		for (unsigned, signed) in things {
			assert_eq!(unsigned, signed_to_unsigned(signed));
			assert_eq!(signed, unsigned_to_signed(unsigned));
		}
	}

	#[test]
	fn signed_conversion() {
		let mut gen = IDGenerator64::new();

		for i in 0..1000 {
			let id = gen.next().unwrap();

			let unsigned_converted = unsafe { GeneratedID64::from_unsigned_unchecked(id.as_unsigned()) };
			assert_eq!(id.unsigned, unsigned_converted.unsigned);

			let signed_converted = unsafe { GeneratedID64::from_signed_unchecked(id.as_signed()) };
			assert_eq!(id.unsigned, signed_converted.unsigned);

			if i % 50 == 0 {
				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		}
	}

	#[test]
	fn roundtrip_alphanumeric_string_conversion() {
		let mut gen = IDGenerator64::new();

		for i in 0..1000 {
			let id = gen.next().unwrap();

			let string = id.to_alphanumeric_string();
			let decoded = GeneratedID64::from_alphanumeric_string(&string).unwrap();

			assert_eq!(id.unsigned, decoded.unsigned);

			if i % 50 == 0 {
				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		}
	}
}
