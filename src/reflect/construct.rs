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
	target: ObjectSlot<'h>,
	arguments_list: ArgumentsListSlot<'h>,
	new_target: ObjectSlot<'h>
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
				target: ObjectSlot::uninit(),
				arguments_list: ArgumentsListSlot { uninit: () },
				new_target: ObjectSlot::uninit()
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
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
	Target: SlotUnchecked<ObjectSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>,
	NewTarget: SlotUnchecked<ObjectSlot<'h>>
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
	gen_change_state!('h);
}

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
