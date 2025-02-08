//! Builder for `Reflect.getOwnPropertyDescriptor()` and supporting elements

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
			inner: BuilderInner {
				target: ObjectSlot::uninit(),
				property_key: PropertyKeySlot { uninit: () }
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
	gen_change_state!('h);

	gen_builder_fn! {
		'h

		Target
		TargetInit
		ObjectSlot

		target
		target_unchecked
	}

	gen_builder_fn! {
		'h

		PropertyKey
		PropertyKeyInit
		PropertyKeySlot

		property_key
		property_key_unchecked
	}
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
