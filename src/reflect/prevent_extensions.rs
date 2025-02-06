//! Builder for `Reflect.prevent_extensions()` and supporting elements

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
	target: TargetSlot<'h>
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
				target: TargetSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>
>> {
	#[inline]
	/// Executes `Reflect.prevent_extensions(target)`
	// todo return boolean
	pub fn execute(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
			}

			let raw = raw::prevent_extensions(
				target
			).unwrap_unchecked();
			ExternAny::from_js_value(raw)
		}
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline]
	pub fn target<'h2, T>(
		self,
		target: T
	) -> Builder<'h2, S::TargetInit<T>>
	where
		'h: 'h2,
		S::Target: IsUninit,
		T: Slot<TargetSlot<'h2>>
	{
		unsafe { self.target_unchecked(target) }
	}

	#[inline]
	pub unsafe fn target_unchecked<'h2, T>(
		self,
		target: T
	) -> Builder<'h2, S::TargetInit<T>>
	where
		'h: 'h2,
		S::Target: IsUninit,
		T: SlotUnchecked<TargetSlot<'h2>>
	{
		unsafe { self.change_state(|b| target.write(&mut b.inner.target)) }
	}

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

pub union TargetSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

unsafe impl<'h> Slot<TargetSlot<'h>> for &'h ExternObject {}
unsafe impl<'h> SlotUnchecked<TargetSlot<'h>> for &'h ExternObject {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut TargetSlot<'h>) {
		*slot = TargetSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: TargetSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

unsafe impl<'h> SlotUnchecked<TargetSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut TargetSlot<'h>) {
		*slot = TargetSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: TargetSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}
