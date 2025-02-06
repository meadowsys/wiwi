//! Builder for `Reflect.preventExtensions()` and supporting elements

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
	target: ObjectSlot<'h>
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
				target: ObjectSlot::uninit()
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<ObjectSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>
>> {
	#[inline]
	/// Executes `Reflect.preventExtensions(target)`
	// todo return boolean
	pub fn execute(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
			}

			let raw = raw::prevent_extensions(
				target
			).unwrap();
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
		T: Slot<ObjectSlot<'h2>>
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
		T: SlotUnchecked<ObjectSlot<'h2>>
	{
		unsafe { self.change_state(|b| target.write(&mut b.inner.target)) }
	}

	gen_change_state!('h);
}
