//! Builder for `Reflect.setPrototypeOf()` and supporting elements

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
	prototype: PrototypeSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	Prototype
	PrototypeInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: Inner {
				target: TargetSlot::uninit(),
				prototype: PrototypeSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	Prototype: SlotUnchecked<PrototypeSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<Prototype>
>> {
	/// Calls the function `Reflect.setPrototypeOf(target, prototype)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				prototype: Prototype
			}

			raw::set_prototype_of(target, prototype)
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

		Prototype
		PrototypeInit
		PrototypeSlot

		prototype
		prototype_unchecked
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

pub union PrototypeSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
