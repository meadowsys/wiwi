//! Builder for `Reflect.get_own_property_descriptor()` and supporting elements

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
	TargetInitWith

	PropertyKey
	PropertyKeyInit
	PropertyKeyInitWith
}

// - todo impl builder uninit

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.getOwnPropertyDescriptor(target, propertyKey)

// - todo impl block for builder fns, `change_state`, internal functions etc

// - todo target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)
