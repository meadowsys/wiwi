//! Builder for `Reflect.construct()` and supporting elements

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
	arguments_list: ArgumentsListSlot<'h>,
	new_target: ObjectSlot<'h>
}

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

// - todo impl blocks for finished ones with `execute` or `build` fns
//   Reflect.construct(target, argumentsList)
//   Reflect.construct(target, argumentsList, newTarget)
impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>,
	ArgumentsList: SlotUnchecked<ArgumentsListSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<ArgumentsList>,
	Uninit
>> {
	/// Executes `Reflect.construct(target, argumentsList)`
	// todo better return type
	#[inline]
	pub fn execute(self) -> Result<ExternAny, ExternAny> {
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
	/// Executes `Reflect.construct(target, argumentsList, newTarget)`
	// todo better return type
	#[inline]
	pub fn execute(self) -> Result<ExternAny, ExternAny> {
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
pub union ArgumentsListSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}
