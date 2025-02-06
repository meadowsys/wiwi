//! Builder for `Reflect.defineProperty()` and supporting elements

use crate::prelude_internal::*;
use super::raw;

// - todo builder struct (with repr(transparent), `inner`, `__marker`)
#[repr(transparent)]
pub struct Builder<'h, S>
where
	S: State
{
	inner: BuilderInner<'h>,
	__marker: PhantomDataInvariant<S>
}

// - todo builder inner struct
struct BuilderInner<'h> {
	target: ObjectSlot<'h>,
	property_key: PropertyKeySlot<'h>,
	attributes: AttributesSlot<'h>
}

// - todo `gen_state!` invocation (generates state trait, state
//   container struct, uninit type def, impl state for statecontainer)
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

// - todo impl builder uninit
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

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.defineProperty(target, propertyKey, attributes)
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

// - todo impl block for builder fns, `change_state`, internal functions etc
impl<'h, S> Builder<'h, S>
where
	S: State
{
	gen_change_state!('h);
}

// - todo target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)
pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union AttributesSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
