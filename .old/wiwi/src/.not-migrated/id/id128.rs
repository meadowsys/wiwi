use crate::num_traits::*;
use crate::rand::ThreadLocalChaCha8Rng;
use std::time::{ SystemTime, UNIX_EPOCH };

/// 54 bits for timestamp gives >572K years or something like that, with
/// millisecond precision
///
/// This goes out to about `di 25 jul 572823 17:58:01 UTC`
///
/// After this field, we have 64 bits left.
const TIMESTAMP_MS_SHIFT: u8 = 128 - 54;

/// 10 bits after the millisecond field stores the microsecond value, if the
/// microseconds value can be trusted
///
/// If the microseconds value is not to be trusted, this field will turn into
/// a "secondary counter". Reset this back to 0 every time the millisecond value
/// increases.
const TIMESTAMP_MICROS_SHIFT: u8 = TIMESTAMP_MS_SHIFT - 10;

/// Next bit after timestamp encodes if the microseconds value should be
/// consideered accurate
///
/// Even with this unset, the timestamp is encoded with microseconds
/// as the unit (eg. 1000 microseconds instead of 1 millisecond), so IDs
/// generated with trusted microsecond data can interop with IDs without
/// trusted microsecond data, and sort correctly still
///
/// After this field, we have 63 bits left.
const MICROSECOND_TOGGLE_SHIFT: u8 = TIMESTAMP_MICROS_SHIFT - 1;

/// Next 8 bits after the trusted microseconds toggle stores a process ID,
/// a value that's unique to one process that concurrently generates IDs
///
/// 8 bits allows for up to 256 processes to generate IDs concurrently, each one
/// with its own unique process
const PROC_ID_SHIFT: u8 = MICROSECOND_TOGGLE_SHIFT - 8;

/// Next 15 bits after the process ID encodes an increment value for IDs
/// generated within the same microsecond
///
/// 15 bits allows for 32768 IDs to be generated per _microsecond_. If you are
/// generating IDs without a trusted microsecond value, you can use the microsecond
/// value to store count increments too, which allows for 32768000 (~32M) IDs to
/// be generated per millisecond.
const COUNT_SHIFT: u8 = PROC_ID_SHIFT - 15;

const MAX_COUNT: u16 = 1 << 15;
const MAX_MICROS_COUNT: u16 = 1 << 10;

/// Mask the lower 40 bits for the random component (generate random u64,
/// mask with this value)
const RAND_COMPONENT_MASK: u64 = 0xFF_FFFF_FFFF;

pub struct IDGenerator128 {
	gen_millis: u64,
	gen_micros_or_2e_count: u16,
	micros_accurate: bool,
	proc_id: u8,
	count: u16
}

impl IDGenerator128 {
	pub fn new_accurate_ms(proc_id: u8) -> Self {
		Self {
			gen_millis: 0,
			gen_micros_or_2e_count: 0,
			micros_accurate: true,
			proc_id,
			count: 1
		}
	}

	pub fn new_inaccurate_ms(proc_id: u8) -> Self {
		Self {
			gen_millis: 0,
			gen_micros_or_2e_count: 0,
			micros_accurate: false,
			proc_id,
			count: 1
		}
	}

	pub fn next_details(&mut self) -> Option<GeneratedID128Details> {
		let now = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.expect("we are before 01 jan 1970 lol?");

		let millis = now.as_millis().into_u64_lossy();
		let micros = (now.subsec_micros() % 1000).into_u16_lossy();

		if millis > self.gen_millis {
			self.gen_millis = millis;
			self.gen_micros_or_2e_count = if self.micros_accurate {
				micros
			} else {
				0
			};
			self.count = 0;

			Some(GeneratedID128Details {
				gen_millis: self.gen_millis,
				gen_micros_or_2e_count: self.gen_micros_or_2e_count,
				micros_accurate: self.micros_accurate,
				proc_id: self.proc_id,
				count: self.count,
				random: ThreadLocalChaCha8Rng::gen_rand()
			})
		} else if self.micros_accurate {
			if micros > self.gen_micros_or_2e_count  {
				self.gen_micros_or_2e_count = micros;
				self.count = 0;

				Some(GeneratedID128Details {
					gen_millis: self.gen_millis,
					gen_micros_or_2e_count: self.gen_micros_or_2e_count,
					micros_accurate: self.micros_accurate,
					proc_id: self.proc_id,
					count: self.count,
					random: ThreadLocalChaCha8Rng::gen_rand()
				})
			} else {
				if self.count >= MAX_COUNT { return None }

				let count = self.count;
				self.count += 1;

				Some(GeneratedID128Details {
					gen_millis: self.gen_millis,
					gen_micros_or_2e_count: self.gen_micros_or_2e_count,
					micros_accurate: self.micros_accurate,
					proc_id: self.proc_id,
					count: self.count,
					random: ThreadLocalChaCha8Rng::gen_rand()
				})
			}
		} else {
			if self.count >= MAX_COUNT {
				if self.gen_micros_or_2e_count >= MAX_MICROS_COUNT { return None }

				self.count = 0;

				let micros = self.gen_micros_or_2e_count;
				self.gen_micros_or_2e_count += 1;

				Some(GeneratedID128Details {
					gen_millis: self.gen_millis,
					gen_micros_or_2e_count: micros,
					micros_accurate: self.micros_accurate,
					proc_id: self.proc_id,
					count: self.count,
					random: ThreadLocalChaCha8Rng::gen_rand()
				})
			} else {
				let count = self.count;
				self.count += 1;

				Some(GeneratedID128Details {
					gen_millis: self.gen_millis,
					gen_micros_or_2e_count: self.gen_micros_or_2e_count,
					micros_accurate: self.micros_accurate,
					proc_id: self.proc_id,
					count: self.count,
					random: ThreadLocalChaCha8Rng::gen_rand()
				})
			}
		}
	}
}

pub struct GeneratedID128Details {
	gen_millis: u64,
	gen_micros_or_2e_count: u16,
	micros_accurate: bool,
	proc_id: u8,
	count: u16,
	random: u64
}

impl GeneratedID128Details {
	pub fn into_u128(self) -> u128 {
		let mut id = self.gen_millis.into_u128() << TIMESTAMP_MS_SHIFT;
		id |= self.gen_micros_or_2e_count.into_u128() << TIMESTAMP_MICROS_SHIFT;
		id |= (self.micros_accurate as u128) << MICROSECOND_TOGGLE_SHIFT;
		id |= self.proc_id.into_u128() << PROC_ID_SHIFT;
		id |= self.count.into_u128() << COUNT_SHIFT;
		id |= (self.random & RAND_COMPONENT_MASK).into_u128();
		id
	}
}
