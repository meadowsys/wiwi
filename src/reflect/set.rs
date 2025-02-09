//! Builder for `Reflect.set()` and supporting elements

use crate::prelude_internal::*;
use super::raw;

#[repr(transparent)]
pub struct Builder<'h, S>
where
	S: State
{
	inner: BuilderInner<'h>,
	__marker: PhantomDataInvariant<S>
}

struct BuilderInner<'h> {
	target: ObjectSlot<'h>,
	property_key: PropertyKeySlot<'h>,
	value: ValueSlot<'h>,
	receiver: ReceiverSlot<'h>,
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	PropertyKey
	PropertyKeyInit

	Value
	ValueInit

	Receiver
	ReceiverInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: ObjectSlot::uninit(),
				property_key: PropertyKeySlot { uninit: () },
				value: ValueSlot { uninit: () },
				receiver: ReceiverSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Value: SlotUnchecked<ValueSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Uninit
>> {
	/// Calls the function `Reflect.set(target, propertyKey, value)`
	// todo make the return types better
	// returns boolean
	#[inline]
	pub fn call_fn(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				value: Value
			}

			let raw = raw::set3(
				target,
				property_key,
				value
			).unwrap();
			ExternAny::from_js_value(raw)
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Value: SlotUnchecked<ValueSlot<'h>>,
	Receiver: SlotUnchecked<ReceiverSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Init<Receiver>
>> {
	/// Calls the function `Reflect.set(target, propertyKey, value, receiver)`
	// todo make the return types better
	// returns boolean
	#[inline]
	pub fn call_fn(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				value: Value
				receiver: Receiver
			}

			let raw = raw::set4(
				target,
				property_key,
				value,
				receiver
			).unwrap();
			ExternAny::from_js_value(raw)
		}
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	gen_change_state!();

	gen_builder_fn! {
		Target
		TargetInit
		ObjectSlot

		target
		target_unchecked
	}

	gen_builder_fn! {
		PropertyKey
		PropertyKeyInit
		PropertyKeySlot

		property_key
		property_key_unchecked
	}

	gen_builder_fn! {
		Value
		ValueInit
		ValueSlot

		value
		value_unchecked
	}

	gen_builder_fn! {
		Receiver
		ReceiverInit
		ReceiverSlot

		receiver
		receiver_unchecked
	}
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny,
	str: &'h str
}

unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

unsafe impl<'h> Slot<PropertyKeySlot<'h>> for &'h str {}
unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h str {
	type Result = ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { str: self }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> ExternAny {
		unsafe { ExternAny::from_str(slot.str) }
	}

	#[inline]
	fn as_ref(result: &ExternAny) -> &ExternAny {
		result
	}
}

// todo alloc feature? I dunno how alloc interacts with no_std just yet
#[cfg(feature = "std")]
unsafe impl<'h> Slot<PropertyKeySlot<'h>> for &'h String {}
#[cfg(feature = "std")]
unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h String {
	type Result = ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		unsafe { <&str>::write(self, slot) }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> ExternAny {
		unsafe { <&str>::read(slot) }
	}

	#[inline]
	fn as_ref(result: &ExternAny) -> &ExternAny {
		result
	}
}

pub union ValueSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

unsafe impl<'h> SlotUnchecked<ValueSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ValueSlot<'h>) {
		*slot = ValueSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ValueSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

pub union ReceiverSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

unsafe impl<'h> SlotUnchecked<ReceiverSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ReceiverSlot<'h>) {
		*slot = ReceiverSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ReceiverSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}
