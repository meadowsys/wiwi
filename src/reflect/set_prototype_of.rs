//! Builder for `Reflect.setPrototypeOf()` and supporting elements

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
			inner: BuilderInner {
				target: ObjectSlot::uninit(),
				prototype: PrototypeSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
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

		Prototype
		PrototypeInit
		PrototypeSlot

		prototype
		prototype_unchecked
	}
}

pub union PrototypeSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
