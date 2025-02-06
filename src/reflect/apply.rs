//! Builder for `Reflect.apply()` and supporting elements

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
	this_argument: ThisArgumentSlot<'h>,
	arguments_list: ArgumentsListSlot<'h>
}

// - todo `gen_state!` invocation (generates state trait, state
//   container struct, uninit type def, impl state for statecontainer)
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

// - todo impl builder uninit
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

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.apply(target, thisArgument, argumentsList)
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

// - todo impl block for builder fns, `change_state`, internal functions etc
impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline]
	unsafe fn change_state<'h2, S2, F>(self, f: F) -> Builder<'h2, S2>
	where
		'h: 'h2,
		S2: State,
		F: FnOnce(&mut Builder<'h2, S2>)
	{
		let mut changed = Builder {
			inner: self.inner,
			__marker: PhantomData
		};

		f(&mut changed);
		changed
	}
}

// - todo target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)
pub union ThisArgumentSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
