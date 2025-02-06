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
	target: ObjectSlot<'h>,
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
				target: ObjectSlot::uninit(),
				this_argument: ThisArgumentSlot { uninit: () },
				arguments_list: ArgumentsListSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	ThisArgument: SlotUnchecked<ThisArgumentSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<ThisArgument>,
	Init<ArgumentsList>
>> {
	/// Executes `Reflect.apply(target, thisArgument, argumentsList)`
	// todo make the return types better
	#[inline]
	pub fn execute(self) -> Result<ExternAny, ExternAny> {
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
	gen_change_state!('h);
}

pub union ThisArgumentSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
