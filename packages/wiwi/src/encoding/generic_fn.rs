#![allow(unused_imports, reason = "wip")]
use crate::prelude::*;
use super::{ base16, base32, base64, hex, rfc1751, z85 };

#[inline]
pub fn encode<E>(value: impl Encodable<E>) -> E::EncodeOutput
where
	E: Encoding
{
	value.encode()
}

#[inline]
pub fn decode<E>(value: impl Decodable<E>) -> E::DecodeOutput
where
	E: Encoding
{
	value.decode()
}

pub trait Encode {
	fn encode<E>(self) -> E::EncodeOutput
	where
		Self: Encodable<E>,
		E: Encoding;

	fn decode<E>(self) -> E::DecodeOutput
	where
		Self: Decodable<E>,
		E: Encoding;
}

impl<T> Encode for T {
	#[inline]
	fn encode<E>(self) -> E::EncodeOutput
	where
		Self: Encodable<E>,
		E: Encoding
	{
		encode(self)
	}

	#[inline]
	fn decode<E>(self) -> E::DecodeOutput
	where
		Self: Decodable<E>,
		E: Encoding
	{
		decode(self)
	}
}

pub trait Encodable<E>
where
	E: Encoding,
	Self: Sized
{
	fn encode(self) -> E::EncodeOutput;
}

pub trait Decodable<E>
where
	E: Encoding,
	Self: Sized
{
	fn decode(self) -> E::DecodeOutput;
}

pub trait Encoding
where
	Self: Sized + Sealed
{
	type EncodeOutput;
	type DecodeOutput;

	#[inline]
	fn encode<T>(value: T) -> Self::EncodeOutput
	where
		T: Encodable<Self>
	{
		value.encode()
	}

	#[inline]
	fn decode<T>(value: T) -> Self::DecodeOutput
	where
		T: Decodable<Self>
	{
		value.decode()
	}
}

pub struct Base16 {
	__private: ()
}

impl Sealed for Base16 {}

impl Encoding for Base16 {
	type EncodeOutput = String;
	type DecodeOutput = Result<Vec<u8>, base16::DecodeError>;
}

impl Encodable<Base16> for &[u8] {
	#[inline]
	fn encode(self) -> String {
		base16::encode_base16(self)
	}
}

impl Encodable<Base16> for &str {
	#[inline]
	fn encode(self) -> String {
		base16::encode_base16(self.as_bytes())
	}
}

impl Decodable<Base16> for &[u8] {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, base16::DecodeError> {
		base16::decode_base16(self)
	}
}

impl Decodable<Base16> for &str {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, base16::DecodeError> {
		base16::decode_base16(self.as_bytes())
	}
}

pub struct Base32 {
	__private: ()
}

impl Sealed for Base32 {}

impl Encoding for Base32 {
	type EncodeOutput = String;
	type DecodeOutput = (/* todo */);
}

impl Encodable<Base32> for &[u8] {
	#[inline]
	fn encode(self) -> String {
		base32::encode_base32(self)
	}
}

impl Encodable<Base32> for &str {
	#[inline]
	fn encode(self) -> String {
		base32::encode_base32(self.as_bytes())
	}
}

pub struct Base64 {
	__private: ()
}

impl Sealed for Base64 {}

impl Encoding for Base64 {
	type EncodeOutput = (/* todo */);
	type DecodeOutput = (/* todo */);
}

pub struct Hex {
	__private: ()
}

impl Sealed for Hex {}

impl Encoding for Hex {
	type EncodeOutput = String;
	type DecodeOutput = Result<Vec<u8>, hex::DecodeError>;
}

impl Encodable<Hex> for &[u8] {
	#[inline]
	fn encode(self) -> String {
		hex::encode_hex(self)
	}
}

impl Encodable<Hex> for &str {
	#[inline]
	fn encode(self) -> String {
		hex::encode_hex(self.as_bytes())
	}
}

impl Decodable<Hex> for &[u8] {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, hex::DecodeError> {
		hex::decode_hex(self)
	}
}

impl Decodable<Hex> for &str {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, hex::DecodeError> {
		hex::decode_hex(self.as_bytes())
	}
}

pub struct RFC1751 {
	__private: ()
}

impl Sealed for RFC1751 {}

impl Encoding for RFC1751 {
	type EncodeOutput = (/* todo */);
	type DecodeOutput = (/* todo */);
}

pub struct Z85 {
	__private: ()
}

impl Sealed for Z85 {}

impl Encoding for Z85 {
	type EncodeOutput = String;
	type DecodeOutput = Result<Vec<u8>, z85::DecodeError>;
}

impl Encodable<Z85> for &[u8] {
	#[inline]
	fn encode(self) -> String {
		z85::encode_z85(self)
	}
}

impl Encodable<Z85> for &str {
	#[inline]
	fn encode(self) -> String {
		z85::encode_z85(self.as_bytes())
	}
}

impl Decodable<Z85> for &[u8] {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, z85::DecodeError> {
		z85::decode_z85(self)
	}
}

impl Decodable<Z85> for &str {
	#[inline]
	fn decode(self) -> Result<Vec<u8>, z85::DecodeError> {
		z85::decode_z85(self.as_bytes())
	}
}

/// notouchie
pub trait Sealed {}
