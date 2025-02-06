//! Builder for `Reflect.set()` and supporting elements

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
	target: TargetSlot<'h>,
	property_key: PropertyKeySlot<'h>,
	value: ValueSlot<'h>,
	receiver: ReceiverSlot<'h>,
}

gen_state! {
	State
	StateContainer
	StateUninit

	Target
	TargetInit

	PropertyKey
	PropertyKeyInit

	Value
	ValueInit

	Receiver
	ReceiverInit
}

impl Builder<'static, StateUninit> {
	#[inline]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: TargetSlot { uninit: () },
				property_key: PropertyKeySlot { uninit: () },
				value: ValueSlot { uninit: () },
				receiver: ReceiverSlot { uninit: () }
			},
			__marker: PhantomData
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Value: SlotUnchecked<ValueSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Uninit
>> {
	/// Executes `Reflect.set(target, property_key, value)`
	// todo make the return types better
	// returns boolean, do unwrap unchecked
	#[inline]
	pub fn execute(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				value: Value
			}

			let raw = raw::set3(
				target,
				property_key,
				value
			).unwrap_unchecked();
			ExternAny::from_js_value(raw)
		}
	}
}

impl<
	'h,
	Target: SlotUnchecked<TargetSlot<'h>>,
	PropertyKey: SlotUnchecked<PropertyKeySlot<'h>>,
	Value: SlotUnchecked<ValueSlot<'h>>,
	Receiver: SlotUnchecked<ReceiverSlot<'h>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Init<Receiver>
>> {
	/// Executes `Reflect.set(target, property_key, value, receiver)`
	// todo make the return types better
	// returns boolean, do unwrap unchecked
	#[inline]
	pub fn execute(self) -> ExternAny {
		unsafe {
			read_slots! {
				self
				target: Target
				property_key: PropertyKey
				value: Value
				receiver: Receiver
			}

			let raw = raw::set4(
				target,
				property_key,
				value,
				receiver
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
	pub fn property_key<'h2, T>(
		self,
		property_key: T
	) -> Builder<'h2, S::PropertyKeyInit<T>>
	where
		'h: 'h2,
		S::PropertyKey: IsUninit,
		T: Slot<PropertyKeySlot<'h2>>
	{
		unsafe { self.property_key_unchecked(property_key) }
	}

	#[inline]
	pub unsafe fn property_key_unchecked<'h2, T>(
		self,
		property_key: T
	) -> Builder<'h2, S::PropertyKeyInit<T>>
	where
		'h: 'h2,
		S::PropertyKey: IsUninit,
		T: SlotUnchecked<PropertyKeySlot<'h2>>
	{
		unsafe { self.change_state(|b| property_key.write(&mut b.inner.property_key)) }
	}

	#[inline]
	pub fn value<'h2, T>(
		self,
		value: T
	) -> Builder<'h2, S::ValueInit<T>>
	where
		'h: 'h2,
		S::Value: IsUninit,
		T: Slot<ValueSlot<'h2>>
	{
		unsafe { self.value_unchecked(value) }
	}

	#[inline]
	pub unsafe fn value_unchecked<'h2, T>(
		self,
		value: T
	) -> Builder<'h2, S::ValueInit<T>>
	where
		'h: 'h2,
		S::Value: IsUninit,
		T: SlotUnchecked<ValueSlot<'h2>>
	{
		unsafe { self.change_state(|b| value.write(&mut b.inner.value)) }
	}

	#[inline]
	pub fn receiver<'h2, T>(
		self,
		receiver: T
	) -> Builder<'h2, S::ReceiverInit<T>>
	where
		'h: 'h2,
		S::Receiver: IsUninit,
		T: Slot<ReceiverSlot<'h2>>
	{
		unsafe { self.receiver_unchecked(receiver) }
	}

	#[inline]
	pub unsafe fn receiver_unchecked<'h2, T>(
		self,
		receiver: T
	) -> Builder<'h2, S::ReceiverInit<T>>
	where
		'h: 'h2,
		S::Receiver: IsUninit,
		T: SlotUnchecked<ReceiverSlot<'h2>>
	{
		unsafe { self.change_state(|b| receiver.write(&mut b.inner.receiver)) }
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

pub union PropertyKeySlot<'h> {
	uninit: (),
	any: &'h ExternAny,
	str: &'h str
}

unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

unsafe impl<'h> Slot<PropertyKeySlot<'h>> for &'h str {}
unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h str {
	type Result = ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { str: self }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> ExternAny {
		unsafe { ExternAny::from_str(slot.str) }
	}

	#[inline]
	fn as_ref(result: &ExternAny) -> &ExternAny {
		result
	}
}

unsafe impl<'h> Slot<PropertyKeySlot<'h>> for &'h String {}
unsafe impl<'h> SlotUnchecked<PropertyKeySlot<'h>> for &'h String {
	type Result = ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		unsafe { <&str>::write(self, slot) }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> ExternAny {
		unsafe { <&str>::read(slot) }
	}

	#[inline]
	fn as_ref(result: &ExternAny) -> &ExternAny {
		result
	}
}

pub union ValueSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

unsafe impl<'h> SlotUnchecked<ValueSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ValueSlot<'h>) {
		*slot = ValueSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ValueSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

pub union ReceiverSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

unsafe impl<'h> SlotUnchecked<ReceiverSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ReceiverSlot<'h>) {
		*slot = ReceiverSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ReceiverSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}
