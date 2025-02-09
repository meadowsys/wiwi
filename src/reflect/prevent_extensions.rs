//! Builder for `Reflect.preventExtensions()` and supporting elements

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
	target: ObjectSlot<'h>
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
				target: ObjectSlot::uninit()
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>
>> {
	#[inline]
	/// Calls the function `Reflect.preventExtensions(target)`
	// todo return boolean
	pub fn call_fn(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
			}

			let raw = raw::prevent_extensions(
				target
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
}
