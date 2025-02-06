//! Builder for `Reflect.construct()` and supporting elements

use crate::prelude_internal::*;
use super::raw;

// - todo builder struct (with repr(transparent), `inner`, `__marker`)

// - todo builder inner struct

// - todo `gen_state!` invocation (generates state trait, state
//   container struct, uninit type def, impl state for statecontainer)
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

// - todo impl builder uninit

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.construct(target, argumentsList)
//   Reflect.construct(target, argumentsList, newTarget)

// - todo impl block for builder fns, `change_state`, internal functions etc

// - todo target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)
pub union TargetSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

pub union NewTargetSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
