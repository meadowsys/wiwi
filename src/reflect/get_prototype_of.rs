//! Builder for `Reflect.get_prototype_of()` and supporting elements

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
	target: ObjectSlot<'h>
}

// - todo `gen_state!` invocation (generates state trait, state
//   container struct, uninit type def, impl state for statecontainer)
gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit
}

// - todo impl builder uninit
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

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.getPrototypeOf(target)

// - todo impl block for builder fns, `change_state`, internal functions etc
impl<'h, S> Builder<'h, S>
where
	S: State
{
	gen_change_state!('h);
}

// - todo target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)
