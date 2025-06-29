use std::{ ptr, slice };
use std::marker::PhantomData;

pub mod error;
pub mod marker;

mod array;
mod binary;
mod bool;
mod cow;
mod map;
mod option;
mod number;
mod single_type_array;
mod string;

pub use self::error::{
	Error,
	Result
};

pub use self::array::SliceSerialiser;
pub use self::binary::{
	Binary,
	BinarySerialiser
};
pub use self::bool::BoolSerialiser;
pub use self::map::{
	DeserialiseMapError,
	MapSerialiser
};
pub use self::option::OptionSerialiser;
pub use self::number::{
	U8Serialiser,
	I8Serialiser,
	NumberSerialiserSigned,
	NumberSerialiserUnsigned,
	NumberSerialiserFloat
};
pub use single_type_array::{
	SingleTypeArray,
	SingleTypeArraySerialise,
	SingleTypeArraySerialiser,
	SingleTypeArraySerialiserImpl
};
pub use self::string::StrSerialiser;

/// Serialise the given value to bytes
///
/// # Examples
///
/// ```
/// # use wiwi::serialiser_binary::serialise;
/// let data = String::from("glory cute");
/// let serialised = serialise(&data);
///
/// // str marker and length
/// assert_eq!(&serialised[0..2], [0xa8, 10]);
/// // the string
/// assert_eq!(&serialised[2..], b"glory cute")
/// ```
pub fn serialise<T>(item: &T) -> Vec<u8>
where
	T: ?Sized + Serialise
{
	let mut vec = Vec::new();
	let mut buf = OutputVecBuffer::new(&mut vec);
	serialise_into(item, &mut buf);
	vec
}

/// Serialise the given value to bytes, writing the bytes into the provided
/// output buffer
///
/// # Examples
///
/// ```
/// # use wiwi::serialiser_binary::serialise_into;
/// let data = String::from("glory cute");
/// let mut output = Vec::new();
/// serialise_into(&data, &mut output);
///
/// // str marker and length
/// assert_eq!(&output[0..2], [0xa8, 10]);
/// // the string
/// assert_eq!(&output[2..], b"glory cute")
/// ```
pub fn serialise_into<T, O>(item: &T, buf: &mut O)
where
	T: ?Sized + Serialise,
	O: Output
{
	let serialiser = item.build_serialiser();
	// SAFETY: callers have no invariants to hold, unsafe is for implementor
	// to assert a correct implementation
	let capacity = unsafe { serialiser.needed_capacity() };

	buf.reserve(capacity);
	// SAFETY: we have called `reserve` on `buf`
	// with `capacity` returned by `needed_capacity`
	unsafe { serialiser.serialise(buf) }
}

/// Deserialise an instance of `T` from the provided input bytes
///
/// Zero copy deserialisation is available for some types.
///
/// # Examples
///
/// ```
/// # use wiwi::serialiser_binary::deserialise;
/// let bytes = Vec::from(b"\xa8\x0aglory cute");
///
/// let deserialised: String = deserialise(&bytes).unwrap();
/// assert_eq!(deserialised, "glory cute");
///
/// // `&str` is a type that offers zero copy deserialisation
/// let deserialised: &str = deserialise(&bytes).unwrap();
/// assert_eq!(deserialised, "glory cute");
/// ```
pub fn deserialise<'h, T>(bytes: &'h [u8]) -> Result<T, T::Error>
where
	T: Deserialise<'h>
{
	let mut input = InputSliceBuffer::new(bytes);
	Ok(use_ok!(
		T::deserialise(&mut input),
		val => if input.is_empty() {
			val
		} else {
			return error::expected(error::expected::DESC_EXPECTED_EOF)
				.found(error::found::DESC_FOUND_TRAILING_BYTES)
				.wrap_foreign()
		}
	))
}

/// Deserialise an instance of `T` from the provided input bytes, without
/// enforcing that all the bytes in the provided bytes are used
///
/// Other than that relaxed requirement, this function behaves exactly the
/// same as [`deserialise`].
///
/// # Examples
///
/// ```
/// # use wiwi::serialiser_binary::deserialise_lax;
/// let bytes = Vec::from(b"\xa8\x0aglory cute and a bunch of trailing bytes :)");
///
/// let (value, remaining_bytes): (&str, _) = deserialise_lax(&bytes).unwrap();
/// assert_eq!(value, "glory cute");
/// assert_eq!(remaining_bytes, b" and a bunch of trailing bytes :)");
/// ```
pub fn deserialise_lax<'h, T>(bytes: &'h [u8]) -> Result<(T, &'h [u8]), T::Error>
where
	T: Deserialise<'h>
{
	let mut input = InputSliceBuffer::new(bytes);
	let val = use_ok!(T::deserialise(&mut input));
	let remaining = input.as_slice();
	Ok((val, remaining))
}

pub trait Serialise {
	type Serialiser<'h>: Serialiser<'h> where Self: 'h;

	/// Gather all data required for serialisation, and store them in this
	/// serialiser struct
	///
	/// You should do most if not all of the heavy lifting here, and store it all
	/// in the serialiser struct.
	fn build_serialiser(&self) -> Self::Serialiser<'_>;
}

impl<T: ?Sized + Serialise> Serialise for &T {
	type Serialiser<'h> = T::Serialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> T::Serialiser<'_> {
		(**self).build_serialiser()
	}
}

impl<T: ?Sized + Serialise> Serialise for &mut T {
	type Serialiser<'h> = T::Serialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> T::Serialiser<'_> {
		(**self).build_serialiser()
	}
}

impl<T: ?Sized + Serialise> Serialise for Box<T> {
	type Serialiser<'h> = T::Serialiser<'h> where T: 'h;

	fn build_serialiser(&self) -> T::Serialiser<'_> {
		T::build_serialiser(self)
	}
}

pub trait Serialiser<'h> {
	/// Get the amount of bytes this item will take up when serialised
	///
	/// # Safety
	///
	/// There are no invariants for callers to uphold.
	///
	/// Implementations must return an accurate value, as unsafe code is allowed
	/// to rely on this for soundness.
	unsafe fn needed_capacity(&self) -> usize;

	/// Serialise `self` into the provided output buffer
	///
	/// # Safety
	///
	/// The provided output `buf` must have at least the amount of free bytes
	/// available to write to as `needed_capacity` returns (ie. calling
	/// `needed_capacity`, then calling `reserve` on `buf` with the value returned
	/// from `needed_capacity`, satisfies this precondition).
	unsafe fn serialise<O: Output>(&self, buf: &mut O);
}

pub trait Deserialise<'h>: Sized {
	type Error: std::error::Error + From<Error>;

	fn deserialise<I: Input<'h>>(buf: &mut I) -> Result<Self, Self::Error> {
		let marker = use_ok!(
			buf.read_byte(),
			#err err => err.expected(
				error::expected::DESC_EXPECTED_MARKER
			).wrap_foreign()
		);

		Self::deserialise_with_marker(buf, marker)
	}

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Self, Self::Error>;
}

impl<'h, T: Deserialise<'h>> Deserialise<'h> for Box<T> {
	type Error = T::Error;

	fn deserialise_with_marker<I: Input<'h>>(buf: &mut I, marker: u8) -> Result<Box<T>, T::Error> {
		Ok(Box::new(use_ok!(T::deserialise_with_marker(buf, marker))))
	}
}

pub trait Output {
	/// Reserve the given amount of bytes in the buffer
	fn reserve(&mut self, bytes: usize);

	/// Write the byte in
	///
	/// # Safety
	///
	/// You must have reserved at least 1 before calling this.
	unsafe fn write_byte(&mut self, byte: u8);

	/// Write all the bytes in the byte slice in
	///
	/// # Safety
	///
	/// You must have reserved at least the amount of space as `bytes.len()`
	/// before calling this.
	unsafe fn write_bytes(&mut self, bytes: &[u8]);
}

impl Output for Vec<u8> {
	fn reserve(&mut self, bytes: usize) {
		self.reserve(bytes);
	}

	unsafe fn write_bytes(&mut self, bytes: &[u8]) {
		let len = self.len();
		// SAFETY: `len` is `self`'s len, so this ptr offset is in bounds
		let ptr = unsafe { self.as_mut_ptr().add(len) };

		// SAFETY: caller promises to have called `reserve` on `self` with
		// at least `bytes.len()` bytes, so there is at least `bytes.len()`
		// unused bytes, `ptr` is pointer to the start of the uninitialised
		// region, and we only write `bytes.len()` bytes, staying in bounds
		unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len()) }

		// SAFETY: `len` is the previous length of `self`, and we just wrote
		// to `bytes.len()` bytes
		unsafe { self.set_len(len + bytes.len()) }
	}

	unsafe fn write_byte(&mut self, byte: u8) {
		let len = self.len();
		let ptr = self.as_mut_ptr().add(len);

		ptr::write(ptr, byte);
		self.set_len(len + 1);
	}
}

pub trait Input<'h> {
	/// Reads the next specified amount of bytes, returning a pointer to it if
	/// there is enough bytes left
	///
	/// Semantically, the returned pointer is granting shared access to the
	/// region of memory of length `bytes`, for lifetime `'h`.
	///
	/// Calling this function is safe, because doing anything with the returned
	/// pointer is unsafe, and then you have to adhere to contract.
	fn read_bytes_ptr(&mut self, bytes: usize) -> Result<*const u8, error::ErrorFound>;

	/// Reads the next specified amount of bytes, returning a slice if there is
	/// enough bytes left
	fn read_bytes(&mut self, bytes: usize) -> Result<&'h [u8], error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes_ptr(bytes),
			// SAFETY: it's valid semantically to create a slice of `bytes` len to the
			// returned pointer for lifetime `'h` (see doc on `read_bytes_ptr` fn)
			ptr => unsafe { slice::from_raw_parts(ptr, bytes) },
			#err err => err.wrap()
		))
	}

	/// Reads the next byte and returns it, if there is at least one left
	fn read_byte(&mut self) -> Result<u8, error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes_ptr(1),
			// SAFETY: semantically `read_bytes_ptr` gives us one byte in
			// the ptr it returns so this is safe to dereference
			byte => unsafe { *byte },
			#err err => err.wrap()
		))
	}

	/// Reads `BYTES` bytes, returning a pointer to an array if there
	/// are enough bytes left
	///
	/// This is the same as [`read_bytes`](Input::read_bytes), but it returns
	/// a reference to a constant sized array, instead of a dynamically sized
	/// slice.
	fn read_bytes_const<const BYTES: usize>(&mut self) -> Result<&'h [u8; BYTES], error::ErrorFound> {
		Ok(use_ok!(
			self.read_bytes_ptr(BYTES),
			// SAFETY: semantically we are given access to next `BYTES` bytes, so
			// casting the pointer to an array of length `BYTES` is valid
			bytes => unsafe { &*bytes.cast::<[u8; BYTES]>() }
		))
	}
}

pub struct OutputVecBuffer<'h> {
	vec: &'h mut Vec<u8>,
	ptr: *const u8
}

impl<'h> OutputVecBuffer<'h> {
	/// Create a new [`OutputVecBuffer`] with the given [`Vec`]
	pub fn new(vec: &'h mut Vec<u8>) -> Self {
		let ptr = vec.as_mut_ptr();
		Self { vec, ptr }
	}
}

impl<'h> Output for OutputVecBuffer<'h> {
	fn reserve(&mut self, bytes: usize) {
		self.vec.reserve_exact(bytes);

		let len = self.vec.len();
		let ptr = self.vec.as_mut_ptr();

		// SAFETY: `ptr` was just gotten from `vec`, *after* the reserve call.
		// `len` is existing amount of elements in the vec, so is in bounds
		self.ptr = unsafe { ptr.add(len).cast_const() };
	}

	unsafe fn write_bytes(&mut self, bytes: &[u8]) {
		let bytes_len = bytes.len();

		ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.cast_mut(), bytes_len);
		self.ptr = self.ptr.add(bytes_len);

		let vec_len = self.vec.len();
		self.vec.set_len(vec_len + bytes_len);
	}

	unsafe fn write_byte(&mut self, byte: u8) {
		ptr::write(self.ptr.cast_mut(), byte);

		let vec_len = self.vec.len();
		self.vec.set_len(vec_len + 1);
		self.ptr = self.ptr.add(1);
	}
}

pub struct InputSliceBuffer<'h> {
	ptr: *const u8,
	remaining: usize,
	__marker: PhantomData<&'h [u8]>
}

impl<'h> InputSliceBuffer<'h> {
	fn new(bytes: &'h [u8]) -> Self {
		Self {
			ptr: bytes.as_ptr(),
			remaining: bytes.len(),
			__marker: PhantomData
		}
	}

	/// Returns if there is any bytes remaining in the slice
	fn is_empty(&self) -> bool {
		self.remaining == 0
	}

	/// Gets a slice of the remaining bytes (with same lifetime as the slice
	/// that was used to create `self`)
	fn as_slice(&self) -> &'h [u8] {
		// SAFETY: every time we read from `self`, we offset `ptr` forward by
		// that much and decrease `remaining` by that much too. So this slice
		// created represents the tail end of the initial buffer (ie. what's left)
		unsafe { slice::from_raw_parts(self.ptr, self.remaining) }
	}
}

impl<'h> Input<'h> for InputSliceBuffer<'h> {
	fn read_bytes_ptr(&mut self, bytes: usize) -> Result<*const u8, error::ErrorFound> {
		if self.remaining < bytes {
			error::found_eof().wrap()
		} else {
			let ptr = self.ptr;

			self.remaining -= bytes;
			// SAFETY: `self.remaining` is the remaining amount of bytes `ptr` is
			// valid for, and we just "took" `bytes` bytes, so we offset `ptr`
			// forward alongside `remaining`
			self.ptr = unsafe { self.ptr.add(bytes) };

			Ok(ptr)
		}
	}
}

macro_rules! use_ok {
	($result:expr) => {
		match $result {
			Ok(val) => { val }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, #err $err:ident => $err_op:expr) => {
		match $result {
			Ok(val) => { val }
			Err($err) => { return $err_op }
		}
	};

	($result:expr, $val:ident => $op:expr) => {
		match $result {
			Ok($val) => { $op }
			Err(err) => { return Err(err) }
		}
	};

	($result:expr, $val:ident => $op:expr, #err $err:ident => $err_op:expr) => {
		match $result {
			Ok($val) => { $op }
			Err($err) => { return $err_op }
		}
	};
}
use use_ok;

// macro_rules! use_some {
// 	($option:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return $none_op }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return $none_op }
// 		}
// 	};
// }
// use use_some;

macro_rules! consts {
	{
		@impl(const)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $val:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub const $name: $($type)+ = $val;
			)*
		}
	};

	{
		@impl(static)
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$((
			$(#[$meta:meta])*
			$name:ident, $val:expr, $($type:tt)+
		))*
	} => {
		$(#[$mod_meta])*
		$mod_vis mod $mod_name {
			$(
				$(#[$meta])*
				pub static $name: $($type)+ = $val;
			)*
		}
	};

	{
		const type u8
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $val:expr)*
	} => {
		consts! {
			@impl(const)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $val, u8))*
		}
	};

	{
		static type &'static str
		$(#[$mod_meta:meta])*
		$mod_vis:vis mod $mod_name:ident
		$($(#[$meta:meta])* $name:ident = $val:expr)*
	} => {
		consts! {
			@impl(static)
			$(#[$mod_meta])*
			$mod_vis mod $mod_name
			$(($(#[$meta])* $name, $val, &'static str))*
		}
	};
}
use consts;

/// Internal prelude
#[allow(unused_imports)]
mod internal_prelude {
	pub(super) use crate::num_traits::*;
	pub(super) use super::{
		Deserialise,
		Input,
		Output,
		Serialise,
		Serialiser,
		use_ok
		// use_some
	};
	pub(super) use super::error::{
		Error,
		ErrorExpected,
		ErrorFound,
		Result,
		expected,
		found,
		found_eof,
		found_something_else
	};
	pub(super) use super::error::expected::*;
	pub(super) use super::error::found::*;
	pub(super) use super::marker::*;
	pub(super) use super::marker::markers::*;
}
