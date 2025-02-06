//! Builder for `Reflect.defineProperty()` and supporting elements

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
	attributes: AttributesSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	PropertyKey
	PropertyKeyInit

	Attributes
	AttributesInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: ObjectSlot::uninit(),
				property_key: PropertyKeySlot { uninit: () },
				attributes: AttributesSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Attributes: SlotUnchecked<AttributesSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Attributes>
>> {
	/// Executes `Reflect.defineProperty(target, propertyKey, attributes)`
	// todo better return type
	#[inline]
	pub fn execute(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				attributes: Attributes
			}

			raw::define_property(target, property_key, attributes)
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
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union AttributesSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
