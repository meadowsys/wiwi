//! Builder for `Reflect.getPrototypeOf()` and supporting elements

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
	target: TargetSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: TargetSlot::uninit()
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>
>> {
	/// Calls the function `Reflect.getPrototypeOf(target)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
			}

			raw::get_prototype_of(target)
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
		TargetSlot

		target
		target_unchecked
	}
}

gen_slot! {
	/// Slot type accepting all objects
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
