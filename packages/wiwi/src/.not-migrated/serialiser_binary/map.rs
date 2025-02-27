use super::internal_prelude::*;
use super::NumberSerialiserUnsigned;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap as HashbrownHashMap;
use std::collections::{ BTreeMap, HashMap as StdHashMap };
use std::fmt;
use std::hash::{ BuildHasher, Hash };

// TODO: deterministic wrapper? cause hashmap iter ordering is nondeterministic

impl<K, V, S> Serialise for StdHashMap<K, V, S>
where
	K: Serialise,
	V: Serialise
{
	type Serialiser<'h> = MapSerialiser<'h, K, V> where K: 'h, V: 'h, S: 'h;

	fn build_serialiser(&self) -> MapSerialiser<'_, K, V> {
		MapSerialiser::new(self)
	}
}

impl<K, V> Serialise for BTreeMap<K, V>
where
	K: Serialise,
	V: Serialise
{
	type Serialiser<'h> = MapSerialiser<'h, K, V> where K: 'h, V: 'h;

	fn build_serialiser(&self) -> MapSerialiser<'_, K, V> {
		MapSerialiser::new(self)
	}
}

// TODO: allocator API?
#[cfg(feature = "hashbrown")]
impl<K, V, S> Serialise for HashbrownHashMap<K, V, S>
where
	K: Serialise,
	V: Serialise
{
	type Serialiser<'h> = MapSerialiser<'h, K, V> where K: 'h, V: 'h, S: 'h;

	fn build_serialiser(&self) -> MapSerialiser<'_, K, V> {
		MapSerialiser::new(self)
	}
}

pub struct MapSerialiser<'h, K: Serialise + 'h, V: Serialise + 'h> {
	kv: Vec<(K::Serialiser<'h>, V::Serialiser<'h>)>,
	len_ser: Option<<usize as Serialise>::Serialiser<'h>>
}

impl<'h, K, V> MapSerialiser<'h, K, V>
where
	K: Serialise,
	V: Serialise
{
	fn new<I: IntoIterator<Item = (&'h K, &'h V)>>(iter: I) -> Self {
		let kv = iter.into_iter()
			.map(|(k, v)| (k.build_serialiser(), v.build_serialiser()))
			.collect::<Vec<_>>();

		let len_ser = if kv.len() > u8::MAX.into_usize() {
			Some(NumberSerialiserUnsigned::new(kv.len()))
		} else {
			None
		};

		Self { kv, len_ser }
	}
}

impl<'h, K, V> Serialiser<'h> for MapSerialiser<'h, K, V>
where
	K: Serialise,
	V: Serialise
{
	unsafe fn needed_capacity(&self) -> usize {
		let len_ser = if let Some(len_ser) = &self.len_ser {
			// marker + length serialised
			1 + len_ser.needed_capacity()
		} else {
			// marker + one byte for len
			2
		};

		let items = self.kv.iter()
			.map(|(k, v)| k.needed_capacity() + v.needed_capacity())
			.sum::<usize>();

		len_ser + items
	}

	unsafe fn serialise<O: Output>(&self, buf: &mut O) {
		if let Some(len_ser) = &self.len_ser {
			buf.write_byte(MARKER_MAP_XL);
			len_ser.serialise(buf);
		} else {
			buf.write_byte(MARKER_MAP_8);
			buf.write_byte(self.kv.len().into_u8_lossy());
		}

		for (k, v) in &self.kv {
			k.serialise(buf);
			v.serialise(buf);
		}
	}
}

impl<'h, K, V, S> Deserialise<'h> for StdHashMap<K, V, S>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	S: BuildHasher + Default
{
	type Error = DeserialiseMapError<K::Error, V::Error>;

	fn deserialise_with_marker<I: Input<'h>>(
		buf: &mut I,
		marker: u8
	) -> Result<StdHashMap<K, V, S>, DeserialiseMapError<K::Error, V::Error>> {
		map_deser_impl(buf, marker)
	}
}

impl<'h, K, V> Deserialise<'h> for BTreeMap<K, V>
where
	K: Deserialise<'h> + Ord,
	V: Deserialise<'h>
{
	type Error = DeserialiseMapError<K::Error, V::Error>;

	fn deserialise_with_marker<I: Input<'h>>(
		buf: &mut I,
		marker: u8
	) -> Result<BTreeMap<K, V>, DeserialiseMapError<K::Error, V::Error>> {
		map_deser_impl(buf, marker)
	}
}

#[cfg(feature = "hashbrown")]
impl<'h, K, V, S> Deserialise<'h> for HashbrownHashMap<K, V, S>
where
	K: Deserialise<'h> + Eq + Hash,
	V: Deserialise<'h>,
	S: BuildHasher + Default
{
	type Error = DeserialiseMapError<K::Error, V::Error>;

	fn deserialise_with_marker<I: Input<'h>>(
		buf: &mut I,
		marker: u8
	) -> Result<HashbrownHashMap<K, V, S>, DeserialiseMapError<K::Error, V::Error>> {
		map_deser_impl(buf, marker)
	}
}

fn map_deser_impl<'h, K, V, M, I>(buf: &mut I, marker: u8) -> Result<M, DeserialiseMapError<K::Error, V::Error>>
where
	K: Deserialise<'h>,
	V: Deserialise<'h>,
	M: FromIterator<(K, V)>,
	I: Input<'h>
{
	let len = match marker {
		MARKER_MAP_8 => {
			use_ok!(
				buf.read_byte(),
				byte => byte.into_usize(),
				#err err => err.expected(DESC_EXPECTED_MAP)
					.wrap_foreign()
			)
		}
		MARKER_MAP_XL => {
			use_ok!(
				usize::deserialise(buf),
				#err err => err.expected(DESC_EXPECTED_MAP)
					.wrap_foreign()
			)
		}
		_ => {
			return expected(DESC_EXPECTED_MAP)
				.found_something_else()
				.wrap_foreign()
		}
	};

	(0..len).map(|_| {
		let k = use_ok!(
			K::deserialise(buf),
			#err err => Err(DeserialiseMapError::KError { err })
		);
		let v = use_ok!(
			V::deserialise(buf),
			#err err => Err(DeserialiseMapError::VError { err })
		);
		Ok((k, v))
	}).collect()
}

#[derive(Debug)]
pub enum DeserialiseMapError<K, V> {
	KError { err: K },
	VError { err: V },
	Wiwi { err: Error }
}

impl<K, V> fmt::Display for DeserialiseMapError<K, V>
where
	K: std::error::Error,
	V: std::error::Error
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::KError { err } => { fmt::Display::fmt(err, f) }
			Self::VError { err } => { fmt::Display::fmt(err, f) }
			Self::Wiwi { err } => { fmt::Display::fmt(err, f) }
		}
	}
}

impl<K, V> std::error::Error for DeserialiseMapError<K, V>
where
	K: std::error::Error,
	V: std::error::Error
{}

impl<K, V> From<Error> for DeserialiseMapError<K, V> {
	fn from(err: Error) -> Self {
		Self::Wiwi { err }
	}
}
