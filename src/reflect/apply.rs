//! Builder for `Reflect.apply()` and supporting elements

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
	target: TargetSlot <'h>,
	this_argument: ThisArgumentSlot<'h>,
	arguments_list: ArgumentsListSlot<'h>
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	ThisArgument
	ThisArgumentInit

	ArgumentsList
	ArgumentsListInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: TargetSlot::uninit(),
				this_argument: ThisArgumentSlot { uninit: () },
				arguments_list: ArgumentsListSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot <'h>>,
	ThisArgument: SlotUnchecked<ThisArgumentSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<ThisArgument>,
	Init<ArgumentsList>
>> {
	/// Calls `Reflect.apply(target, thisArgument, argumentsList)`
	// todo make the return types better
	#[inline]
	pub fn call_fn(self) -> Result<ExternAny, ExternAny> {
		unsafe {
			read_slots! {
				self
				target: Target
				this_argument: ThisArgument
				arguments_list: ArgumentsList
			}

			raw::apply(target, this_argument, arguments_list)
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

		/// Set the target function to call
		///
		/// [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply#target)
		target
		target_unchecked
	}

	gen_builder_fn! {
		ThisArgument
		ThisArgumentInit
		ThisArgumentSlot

		/// Set the value of `this` provided for the call to `target`
		///
		/// [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply#thisargument)
		this_argument
		this_argument_unchecked
	}

	gen_builder_fn! {
		ArgumentsList
		ArgumentsListInit
		ArgumentsListSlot

		/// Sets the list of arguments to use when calling the function
		///
		/// [MDN docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply#argumentslist)
		arguments_list
		arguments_list_unchecked
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

pub union ThisArgumentSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
