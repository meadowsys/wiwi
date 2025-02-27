extern crate hashbrown;

use crate::prelude::*;
use crate::num::*;
use crate::rc::{ RcStr, Counter, AtomicCounter };
use hashbrown::HashSet;

struct JewelStore<C = AtomicCounter>
where
	C: Counter
{
	modifiers: HashSet<Modifier<C>>,
	jewels: Vec<Jewel<C>>
}

struct Jewel<C = AtomicCounter>
where
	C: Counter
{
	level: u16,
	size: i16,
	modifiers: JewelModifiers<C>
}

enum JewelModifiers<C = AtomicCounter>
where
	C: Counter
{
	Chipped([ModifierInstance<C>; 1]),
	Flawed([ModifierInstance<C>; 2]),
	Flawless([ModifierInstance<C>; 3]),
	Perfect([ModifierInstance<C>; 4])
}

struct ModifierInstance<C = AtomicCounter>
where
	C: Counter
{
	modifier: Modifier<C>,
	value: ModifierValue
}

struct Modifier<C = AtomicCounter>
where
	C: Counter
{
	inner: RcStr<C, ModifierMeta<C>>
}

struct ModifierMeta<C = AtomicCounter>
where
	C: Counter
{
	display_name: RcStr<C>,
	modifier_type: ModifierType
}

/// Type of modifier (flag or with numeric value, and associated
/// metadata if applicable)
enum ModifierType {
	/// A modifier that is either present, or not (eg. Soulbound)
	///
	/// For modifiers of this type, the modifier value is meaningless, and for
	/// the sake of being deterministic, it should be set to 0.
	Flag,

	/// A modifier that has an attached number (eg. +100% Trap Disarm, or +15.3 Mining Speed)
	NumericValue {
		/// The power of 10 the stored value needs to be divided by
		/// to get the true value
		///
		/// For example, if you have mining speed with value of 76 (76.0), and decimal
		/// shift 1, you'd need to divide the value by 10^1, or shift the decimal left
		/// by 1, to get the real value of 7.6.
		decimal_shift: u8
	}
}

/// The value of a modifier, including whether or not the modifier is legendary
struct ModifierValue {
	/// The highest bit stores if the modifier is legendary, and the rest store
	/// the modifier's value (if applicable)
	raw: i32
}

impl JewelStore {
	#[inline]
	fn new() -> Self {
		Self::with_counter()
	}

	#[inline]
	fn with_counter<C>() -> JewelStore<C>
	where
		C: Counter
	{
		JewelStore {
			modifiers: HashSet::new(),
			jewels: Vec::new()
		}
	}
}

impl<C> JewelStore<C>
where
	C: Counter
{
	/// Register a modifier by its identifier, return [`Ok`] if added and [`Err`]
	/// if the modifier already exists
	#[inline]
	fn add_modifier(&mut self, id: &str, display_name: &str, modifier_type: ModifierType) -> Result<(), ()> {
		if self.modifiers.get(id).is_none() {
			let modifier = Modifier::new(id, display_name, modifier_type);

			// SAFETY: just checked `id` is not in set
			unsafe {
				self.modifiers.insert_unique_unchecked(modifier);
			}

			Ok(())
		} else {
			Err(())
		}
	}
}

impl<C> Modifier<C>
where
	C: Counter
{
	/// Create new modifier
	///
	/// This will always allocate, as it has no knowledge of existing modifier
	/// instances. Clone an existing modifier instance if you want to reuse
	/// the allocation.
	#[inline]
	fn new(id: &str, display_name: &str, modifier_type: ModifierType) -> Self {
		Self {
			inner: RcStr::with_metadata(id, ModifierMeta {
				display_name: RcStr::new(display_name),
				modifier_type
			})
		}
	}
}

impl<C> Borrow<str> for Modifier<C>
where
	C: Counter
{
	#[inline]
	fn borrow(&self) -> &str {
		&self.inner
	}
}

impl<C> Clone for Modifier<C>
where
	C: Counter
{
	#[inline]
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<C, C2> PartialEq<Modifier<C2>> for Modifier<C>
where
	C: Counter,
	C2: Counter
{
	#[inline]
	fn eq(&self, other: &Modifier<C2>) -> bool {
		*self.inner == *other.inner
	}
}

impl<C> Eq for Modifier<C>
where
	C: Counter
{}

impl<C> Hash for Modifier<C>
where
	C: Counter
{
	#[inline]
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher
	{
		Hash::hash(&*self.inner, state)
	}
}

impl ModifierValue {
	/// Maximum storable modifier value
	const MAX: i32 = i32::MAX >> 1;

	/// Minimum storable modifier value (negative)
	const MIN: i32 = i32::MIN >> 1;

	#[inline]
	fn new(value: i32, is_legendary: bool) -> Self {
		Self::new_checked(value, is_legendary)
			.expect("modifier value out of bounds")
	}

	#[inline]
	fn new_checked(value: i32, is_legendary: bool) -> Option<Self> {
		(Self::MIN..=Self::MAX).contains(&value).then(|| {
			// SAFETY: we just checked we're within the allowed range
			unsafe { Self::new_unchecked(value, is_legendary) }
		})
	}

	/// # Safety
	///
	/// `value` must be within the range `MIN..=MAX`
	#[inline]
	unsafe fn new_unchecked(value: i32, is_legendary: bool) -> Self {
		Self { raw: value | (is_legendary.into_i32() << 31) }
	}

	#[inline]
	fn value(&self) -> i32 {
		// first shift left one to push out the legendary bit, then
		// (arithmetic) shift right one to bring back the bit with correct sign
		(self.raw << 1) >> 1
	}

	#[inline]
	fn is_legendary(&self) -> bool {
		self.raw >> 31 == 1
	}
}
