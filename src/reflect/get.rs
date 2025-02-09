//! Builder for `Reflect.get()` and supporting elements

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
	receiver: ReceiverSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	PropertyKey
	PropertyKeyInit

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
				receiver: ReceiverSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Uninit
>> {
	/// Calls the function `Reflect.get(target, propertyKey)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
			}

			raw::get2(target, property_key)
				.map(ExternAny::from_js_value)
				.map_err(ExternAny::from_js_value)
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Receiver: SlotUnchecked<ReceiverSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Receiver>
>> {
	/// Calls the function `Reflect.get(target, propertyKey, receiver)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				receiver: Receiver
			}

			raw::get3(target, property_key, receiver)
				.map(ExternAny::from_js_value)
				.map_err(ExternAny::from_js_value)
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
		Receiver
		ReceiverInit
		ReceiverSlot

		receiver
		receiver_unchecked
	}
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union ReceiverSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
