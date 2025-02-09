//! Builder for `Reflect.construct()` and supporting elements

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
	target: TargetSlot<'h>,
	arguments_list: ArgumentsListSlot<'h>,
	new_target: NewTargetSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	ArgumentsList
	ArgumentsListInit

	NewTarget
	NewTargetInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: TargetSlot::uninit(),
				arguments_list: ArgumentsListSlot { uninit: () },
				new_target: NewTargetSlot::uninit()
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<ArgumentsList>,
	Uninit
>> {
	/// Calls the function `Reflect.construct(target, argumentsList)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				arguments_list: ArgumentsList
			}

			raw::construct2(target, arguments_list)
				.map(ExternAny::from_js_value)
				.map_err(ExternAny::from_js_value)
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>,
	NewTarget: SlotUnchecked<NewTargetSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<ArgumentsList>,
	Init<NewTarget>
>> {
	/// Calls the function `Reflect.construct(target, argumentsList, newTarget)`
	// todo better return type
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				arguments_list: ArgumentsList
				new_target: NewTarget
			}

			raw::construct3(target, arguments_list, new_target)
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

	gen_builder_fn! {
		ArgumentsList
		ArgumentsListInit
		ArgumentsListSlot

		arguments_list
		arguments_list_unchecked
	}

	gen_builder_fn! {
		NewTarget
		NewTargetInit
		NewTargetSlot

		new_target
		new_target_unchecked
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

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

gen_slot! {
	/// Slot type accepting all objects
	NewTargetSlot
	field any { &'h ExternAny }

	impl for { &'h ExternObject } {
		result { &'h ExternAny }
		write(self, slot) {
			*slot = NewTargetSlot { any: self }
		}
		read(slot) {
			unsafe { slot.any }
		}
		as_ref(result) {
			result
		}
	}
}
