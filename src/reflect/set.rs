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
	TargetInitWith

	PropertyKey
	PropertyKeyInit
	PropertyKeyInitWith

	Value
	ValueInit
	ValueInitWith

	Receiver
	ReceiverInit
	ReceiverInitWith
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
	Target: AcceptableInSlot<TargetSlot<'h>, Result: AsRef<JsValue>>,
	PropertyKey: AcceptableInSlot<PropertyKeySlot<'h>, Result: AsRef<JsValue>>,
	Value: AcceptableInSlot<ValueSlot<'h>, Result: AsRef<JsValue>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Uninit
>> {
	/// Executes `Reflect.set(target, property_key, value)`
	// todo make the return types better
	#[inline]
	pub fn execute(self) -> Result<JsValue, JsValue> {
		read_slots! {
			self
			target: Target
			property_key: PropertyKey
			value: Value
		}

		unsafe { raw::set3(target, property_key, value) }
	}
}

impl<
	'h,
	Target: AcceptableInSlot<TargetSlot<'h>, Result: AsRef<JsValue>>,
	PropertyKey: AcceptableInSlot<PropertyKeySlot<'h>, Result: AsRef<JsValue>>,
	Value: AcceptableInSlot<ValueSlot<'h>, Result: AsRef<JsValue>>,
	Receiver: AcceptableInSlot<ReceiverSlot<'h>, Result: AsRef<JsValue>>
> Builder<'h, StateContainer<
	Init<Target>,
	Init<PropertyKey>,
	Init<Value>,
	Init<Receiver>
>> {
	/// Executes `Reflect.set(target, property_key, value, receiver)`
	// todo make the return types better
	#[inline]
	pub fn execute(self) -> Result<JsValue, JsValue> {
		read_slots! {
			self
			target: Target
			property_key: PropertyKey
			value: Value
			receiver: Receiver
		}

		unsafe { raw::set4(target, property_key, value, receiver) }
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
	) -> Builder<'h2, S::TargetInitWith<T>>
	where
		'h: 'h2,
		S::Target: IsUninit,
		T: AcceptableInSlot<TargetSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.target_unchecked(target) }
	}

	#[inline]
	pub unsafe fn target_unchecked<'h2, T>(
		self,
		target: T
	) -> Builder<'h2, S::TargetInitWith<T>>
	where
		'h: 'h2,
		S::Target: IsUninit,
		T: AcceptableInSlotUnchecked<TargetSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.change_state(|b| target.write_unchecked(&mut b.inner.target)) }
	}

	#[inline]
	pub fn property_key<'h2, T>(
		self,
		property_key: T
	) -> Builder<'h2, S::PropertyKeyInitWith<T>>
	where
		'h: 'h2,
		S::PropertyKey: IsUninit,
		T: AcceptableInSlot<PropertyKeySlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.property_key_unchecked(property_key) }
	}

	#[inline]
	pub unsafe fn property_key_unchecked<'h2, T>(
		self,
		property_key: T
	) -> Builder<'h2, S::PropertyKeyInitWith<T>>
	where
		'h: 'h2,
		S::PropertyKey: IsUninit,
		T: AcceptableInSlotUnchecked<PropertyKeySlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.change_state(|b| property_key.write_unchecked(&mut b.inner.property_key)) }
	}

	#[inline]
	pub fn value<'h2, T>(
		self,
		value: T
	) -> Builder<'h2, S::ValueInitWith<T>>
	where
		'h: 'h2,
		S::Value: IsUninit,
		T: AcceptableInSlot<ValueSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.value_unchecked(value) }
	}

	#[inline]
	pub unsafe fn value_unchecked<'h2, T>(
		self,
		value: T
	) -> Builder<'h2, S::ValueInitWith<T>>
	where
		'h: 'h2,
		S::Value: IsUninit,
		T: AcceptableInSlotUnchecked<ValueSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.change_state(|b| value.write_unchecked(&mut b.inner.value)) }
	}

	#[inline]
	pub fn receiver<'h2, T>(
		self,
		receiver: T
	) -> Builder<'h2, S::ReceiverInitWith<T>>
	where
		'h: 'h2,
		S::Receiver: IsUninit,
		T: AcceptableInSlot<ReceiverSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.receiver_unchecked(receiver) }
	}

	#[inline]
	pub unsafe fn receiver_unchecked<'h2, T>(
		self,
		receiver: T
	) -> Builder<'h2, S::ReceiverInitWith<T>>
	where
		'h: 'h2,
		S::Receiver: IsUninit,
		T: AcceptableInSlotUnchecked<ReceiverSlot<'h2>, Result: AsRef<JsValue>>
	{
		unsafe { self.change_state(|b| receiver.write_unchecked(&mut b.inner.receiver)) }
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
	extern_any: &'h ExternAny
}

unsafe impl<'h> AcceptableInSlotUnchecked<TargetSlot<'h>> for &'h JsValue {
	type Result = &'h JsValue;

	#[inline]
	unsafe fn write_unchecked(self, slot: &mut TargetSlot<'h>) {
		*slot = TargetSlot { extern_any: ExternAny::from_js_value_ref(self) }
	}

	#[inline]
	unsafe fn read_unchecked(slot: TargetSlot<'h>) -> &'h JsValue {
		unsafe { slot.extern_any.as_js_value() }
	}
}

pub union PropertyKeySlot<'h> {
	uninit: (),
	extern_any: &'h ExternAny,
	str: &'h str
}

unsafe impl<'h> AcceptableInSlotUnchecked<PropertyKeySlot<'h>> for &'h JsValue {
	type Result = &'h JsValue;

	#[inline]
	unsafe fn write_unchecked(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { extern_any: ExternAny::from_js_value_ref(self) }
	}

	#[inline]
	unsafe fn read_unchecked(slot: PropertyKeySlot<'h>) -> &'h JsValue {
		unsafe { slot.extern_any.as_js_value() }
	}
}

unsafe impl<'h> AcceptableInSlot<PropertyKeySlot<'h>> for &'h str {
	type Result = JsValue;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		*slot = PropertyKeySlot { str: self }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> JsValue {
		unsafe { JsValue::from_str(slot.str) }
	}
}

unsafe impl<'h> AcceptableInSlot<PropertyKeySlot<'h>> for &'h String {
	type Result = JsValue;

	#[inline]
	unsafe fn write(self, slot: &mut PropertyKeySlot<'h>) {
		unsafe { (**self).write(slot) }
	}

	#[inline]
	unsafe fn read(slot: PropertyKeySlot<'h>) -> JsValue {
		unsafe { <&str>::read(slot) }
	}
}

pub union ValueSlot<'h> {
	uninit: (),
	extern_any: &'h ExternAny
}

unsafe impl<'h> AcceptableInSlotUnchecked<ValueSlot<'h>> for &'h JsValue {
	type Result = &'h JsValue;

	#[inline]
	unsafe fn write_unchecked(self, slot: &mut ValueSlot<'h>) {
		*slot = ValueSlot { extern_any: ExternAny::from_js_value_ref(self) }
	}

	#[inline]
	unsafe fn read_unchecked(slot: ValueSlot<'h>) -> &'h JsValue {
		unsafe { slot.extern_any.as_js_value() }
	}
}

pub union ReceiverSlot<'h> {
	uninit: (),
	extern_any: &'h ExternAny
}

unsafe impl<'h> AcceptableInSlotUnchecked<ReceiverSlot<'h>> for &'h JsValue {
	type Result = &'h JsValue;

	#[inline]
	unsafe fn write_unchecked(self, slot: &mut ReceiverSlot<'h>) {
		*slot = ReceiverSlot { extern_any: ExternAny::from_js_value_ref(self) }
	}

	#[inline]
	unsafe fn read_unchecked(slot: ReceiverSlot<'h>) -> &'h JsValue {
		unsafe { slot.extern_any.as_js_value() }
	}
}
