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

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.apply(target, thisArgument, argumentsList)

// - todo impl block for builder fns, `change_state`, internal functions etc

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
