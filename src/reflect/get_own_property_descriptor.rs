//! Builder for `Reflect.getOwnPropertyDescriptor()` and supporting elements

use crate::prelude_internal::*;
use super::raw;

#[repr(transparent)]
pub struct Builder<'h, S>
where
	S: State
{
	inner: Inner<'h>,
	__marker: PhantomDataInvariant<S>
}

struct Inner<'h> {
	target: TargetSlot<'h>,
	property_key: PropertyKeySlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	PropertyKey
	PropertyKeyInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: Inner {
				target: TargetSlot::uninit(),
				property_key: PropertyKeySlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>
>> {
	/// Calls the function `Reflect.getOwnPropertyDescriptor(target, propertyKey)`
	// todo make return types better
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
			}

			raw::get_own_property_descriptor(target, property_key)
				.map(ExternAny::from_js_value)
				.map_err(ExternAny::from_js_value)
		}
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	gen_change_state!(Builder);

	gen_builder_fn! {
		Builder

		Target
		TargetInit
		TargetSlot

		target
		target_unchecked
	}

	gen_builder_fn! {
		Builder

		PropertyKey
		PropertyKeyInit
		PropertyKeySlot

		property_key
		property_key_unchecked
	}
}

gen_slot! {
	TargetSlot
	field any { &'h ExternAny }

	impl for { &'h ExternObject } {
		result { &'h ExternAny }
		write(self, slot) {
			*slot = TargetSlot { any: self }
		}
		read(slot) {
			unsafe { slot.any }
		}
		as_ref(result) {
			result
		}
	}
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
