use rand::{ CryptoRng, Error, RngCore, SeedableRng };
use rand::rngs::{ OsRng, adapter::ReseedingRng };
use rand_chacha::{ ChaCha8Core, ChaCha12Core, ChaCha20Core };
use std::cell::RefCell;

/// Internal macro to declare a thread local rng struct with the provided
/// struct name, core struct type, and reseeder struct
macro_rules! decl_thread_local_rng {
	{
		$(#[$struct_meta:meta])*
		$struct_name:ident $core:ident $reseeder:ident
	} => {
		$(#[$struct_meta])*
		pub struct $struct_name;

		const _: () = {
			thread_local! {
				static RNG_STATIC: RefCell<ReseedingRng<$core, $reseeder>> = {
					let rng = $core::from_rng($reseeder).expect(concat!("could not initialise ", stringify!($core), " thread-local rng"));
					let rng = ReseedingRng::new(rng, 16384, OsRng);
					RefCell::new(rng)
				};
			}

			impl $struct_name {
				/// Generates a random value of type `T` using this generator
				#[inline]
				pub fn gen_rand<T: Randomisable>() -> T {
					RNG_STATIC.with_borrow_mut(|rng| T::gen_rand(rng))
				}

				/// Fills a slice of values of type `T` using this generator
				/// Doing it with this function is preferable to manually implementing it, as this function
				/// can provide optimised implementations where available.
				#[inline]
				pub fn fill<T: Randomisable>(slice: &mut [T]) {
					RNG_STATIC.with_borrow_mut(|rng| T::fill(rng, slice))
				}
			}

			impl RngCore for $struct_name {
				#[inline]
				fn next_u32(&mut self) -> u32 {
					RNG_STATIC.with_borrow_mut(|rng| rng.next_u32())
				}

				#[inline]
				fn next_u64(&mut self) -> u64 {
					RNG_STATIC.with_borrow_mut(|rng| rng.next_u64())
				}

				#[inline]
				fn fill_bytes(&mut self, dest: &mut [u8]) {
					RNG_STATIC.with_borrow_mut(|rng| rng.fill_bytes(dest))
				}

				#[inline]
				fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
					RNG_STATIC.with_borrow_mut(|rng| rng.try_fill_bytes(dest))
				}
			}
		};
	}
}

decl_thread_local_rng! {
	/// ChaCha with 8 rounds, thread local random number generator, seeding using
	/// the OS-provided randomness source, and reseeding every 16KiB of output
	ThreadLocalChaCha8Rng ChaCha8Core OsRng
}

decl_thread_local_rng! {
	/// ChaCha with 12 rounds, thread local random number generator, seeding using
	/// the OS-provided randomness source, and reseeding every 16KiB of output
	ThreadLocalChaCha12Rng ChaCha12Core OsRng
}

decl_thread_local_rng! {
	/// ChaCha with 20 rounds, thread local random number generator, seeding using
	/// the OS-provided randomness source, and reseeding every 16KiB of output
	ThreadLocalChaCha20Rng ChaCha20Core OsRng
}

impl CryptoRng for ThreadLocalChaCha20Rng {}

/// A trait for items that can be randomly generated from a given
/// random number generator
pub trait Randomisable: Sized {
	/// Generates a random value with the given random number generator
	fn gen_rand<R: RngCore>(rng: &mut R) -> Self;

	/// Fills a slice with the given random number generator
	///
	/// Doing it with this function is preferable to manually implementing it, as this function
	/// can provide optimised implementations where available.
	#[inline]
	fn fill<R: RngCore>(rng: &mut R, slice: &mut [Self]) {
		for slot in slice {
			*slot = Randomisable::gen_rand(rng);
		}
	}
}

/// Internal macro to generate implementations of [`Randomisable`] for integers
/// based on the provided [`gen_rand`](Randomisable::gen_rand) body and optionally
/// [`fill`](Randomisable::fill) body
macro_rules! impl_int_randomisable {
	($($num:ident {
		$gen_rand_rng:ident => $gen_rand_expr:expr
		$(; ($fill_rng_rng:ident, $fill_rng_slice:ident) => $fill_expr:expr;)?
	})*) => {
		$(
			impl Randomisable for $num {
				#[inline]
				fn gen_rand<R: RngCore>($gen_rand_rng: &mut R) -> $num {
					$gen_rand_expr
				}

				$(
					#[inline]
					fn fill<R: RngCore>($fill_rng_rng: &mut R, $fill_rng_slice: &mut [$num]) {
						$fill_expr
					}
				)?
			}
		)*
	}
}

impl_int_randomisable! {
	u64 { rng => rand_u64(rng) }
	u128 { rng => rand_u128(rng) }
	i64 { rng => rand_i64(rng) }
	i128 { rng => rand_i128(rng) }

	u8 {
		rng => rand_u8(rng);
		(rng, slice) => {
			// SAFETY: u8 and u64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<u64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_u8(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_u64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_u8(rng));
		};
	}

	u16 {
		rng => rand_u16(rng);
		(rng, slice) => {
			// SAFETY: u16 and u64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<u64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_u16(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_u64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_u16(rng));
		};
	}

	u32 {
		rng => rand_u32(rng);
		(rng, slice) => {
			// SAFETY: u32 and u64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<u64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_u32(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_u64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_u32(rng));
		};
	}

	i8 {
		rng => rand_i8(rng);
		(rng, slice) => {
			// SAFETY: i8 and i64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<i64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_i8(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_i64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_i8(rng));
		};
	}

	i16 {
		rng => rand_i16(rng);
		(rng, slice) => {
			// SAFETY: i16 and i64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<i64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_i16(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_i64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_i16(rng));
		};
	}

	i32 {
		rng => rand_i32(rng);
		(rng, slice) => {
			// SAFETY: i32 and i64 are plain old data types
			let (prefix, middle, suffix) = unsafe { slice.align_to_mut::<i64>() };

			prefix.iter_mut().for_each(|slot| *slot = rand_i32(rng));
			middle.iter_mut().for_each(|slot| *slot = rand_i64(rng));
			suffix.iter_mut().for_each(|slot| *slot = rand_i32(rng));
		};
	}
}

/// Generate a [`u8`] from the provided `rng` by truncating a generated [`u32`]
#[inline]
fn rand_u8<R: RngCore>(rng: &mut R) -> u8 {
	rand_u32(rng) as _
}

/// Generate a [`u16`] from the provided `rng` by truncating a generated [`u32`]
#[inline]
fn rand_u16<R: RngCore>(rng: &mut R) -> u16 {
	rand_u32(rng) as _
}

/// Generate a [`u32`] from the provided `rng`
#[inline]
fn rand_u32<R: RngCore>(rng: &mut R) -> u32 {
	rng.next_u32()
}

/// Generate a [`u64`] from the provided `rng`
#[inline]
fn rand_u64<R: RngCore>(rng: &mut R) -> u64 {
	rng.next_u64()
}

/// Generate a [`u128`] from the provided `rng` by generating 2 [`u64`]'s,
/// using one for the higher 64 bits and one for the lower 64 bits
#[inline]
fn rand_u128<R: RngCore>(rng: &mut R) -> u128 {
	let lower = rng.next_u64() as u128;
	let upper = rng.next_u64() as u128;
	lower | upper << 64
}

/// Generate a [`i8`] from the provided `rng` by generating a [`u8`]
/// and no-op bitwise casting it to a [`i8`]
#[inline]
fn rand_i8<R: RngCore>(rng: &mut R) -> i8 {
	rand_u8(rng) as _
}

/// Generate a [`i16`] from the provided `rng` by generating a [`u16`]
/// and no-op bitwise casting it to a [`i16`]
#[inline]
fn rand_i16<R: RngCore>(rng: &mut R) -> i16 {
	rand_u16(rng) as _
}

/// Generate a [`i32`] from the provided `rng` by generating a [`u32`]
/// and no-op bitwise casting it to a [`i32`]
#[inline]
fn rand_i32<R: RngCore>(rng: &mut R) -> i32 {
	rand_u32(rng) as _
}

/// Generate a [`i64`] from the provided `rng` by generating a [`u64`]
/// and no-op bitwise casting it to a [`i64`]
#[inline]
fn rand_i64<R: RngCore>(rng: &mut R) -> i64 {
	rand_u64(rng) as _
}

/// Generate a [`i128`] from the provided `rng` by generating a [`u128`]
/// and no-op bitwise casting it to a [`i128`]
#[inline]
fn rand_i128<R: RngCore>(rng: &mut R) -> i128 {
	rand_u128(rng) as _
}
