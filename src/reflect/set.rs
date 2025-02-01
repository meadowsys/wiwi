use crate::prelude_internal::*;
use super::raw;

#[repr(transparent)]
pub struct Builder<'h, S> {
	inner: BuilderInner<'h>,
	__marker: PhantomDataInvariant<S>
}

struct BuilderInner<'h> {
	target: MaybeUninit<&'h JsValue>,
	property_key: MaybeUninit<&'h JsValue>,
	value: MaybeUninit<&'h JsValue>,
	receiver: MaybeUninit<&'h JsValue>
}

pub trait State {
	type Target: InitStatus;
	type TargetInit: State;

	type PropertyKey: InitStatus;
	type PropertyKeyInit: State;

	type Value: InitStatus;
	type ValueInit: State;

	type Receiver: InitStatus;
	type ReceiverInit: State;
}

pub struct StateContainer<Target, PropertyKey, Value, Receiver> {
	__marker: PhantomDataInvariant<(Target, PropertyKey, Value, Receiver)>
}

pub type StateUninit = StateContainer<Uninit, Uninit, Uninit, Uninit>;

impl<
	Target: InitStatus,
	PropertyKey: InitStatus,
	Value: InitStatus,
	Receiver: InitStatus
> State for StateContainer<
	Target,
	PropertyKey,
	Value,
	Receiver
> {
	type Target = Target;
	type TargetInit = StateContainer<Init, PropertyKey, Value, Receiver>;

	type PropertyKey = PropertyKey;
	type PropertyKeyInit = StateContainer<Target, Init, Value, Receiver>;

	type Value = Value;
	type ValueInit = StateContainer<Target, PropertyKey, Init, Receiver>;

	type Receiver = Receiver;
	type ReceiverInit = StateContainer<Target, PropertyKey, Value, Init>;
}

impl Builder<'static, StateUninit> {
	#[inline(always)]
	pub(super) fn new() -> Self {
		Self {
			inner: BuilderInner {
				target: uninit(),
				property_key: uninit(),
				value: uninit(),
				receiver: uninit()
			},
			__marker: PhantomData
		}
	}
}

impl Builder<'_, StateContainer<
	Init,
	Init,
	Init,
	Uninit
>> {
	/// Calls `Reflect.set(target, property_key, value)`
	// todo make the return types better
	#[inline(always)]
	pub fn call(self) -> Result<JsValue, JsValue> {
		let target = unsafe { self.inner.target.assume_init() };
		let property_key = unsafe { self.inner.property_key.assume_init() };
		let value = unsafe { self.inner.value.assume_init() };
		raw::set3(target, property_key, value)
	}
}

impl Builder<'_, StateContainer<
	Init,
	Init,
	Init,
	Init
>> {
	/// Calls `Reflect.set(target, property_key, value, receiver)`
	// todo make the return types better
	#[inline(always)]
	pub fn call(self) -> Result<JsValue, JsValue> {
		let target = unsafe { self.inner.target.assume_init() };
		let property_key = unsafe { self.inner.property_key.assume_init() };
		let value = unsafe { self.inner.value.assume_init() };
		let receiver = unsafe { self.inner.receiver.assume_init() };
		raw::set4(target, property_key, value, receiver)
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline(always)]
	pub fn target<'h2>(
		mut self,
		// todo do the type for target properly
		target: &'h2 JsValue
	) -> Builder<'h2, S::TargetInit>
	where
		'h: 'h2,
		S::Target: IsUninit
	{
		unsafe {
			self.inner.target
				.as_mut_ptr()
				.cast_write(target);
			self.change_state()
		}
	}

	#[inline(always)]
	pub fn property_key<'h2>(
		mut self,
		// todo do the type for property_key properly
		property_key: &'h2 JsValue
	) -> Builder<'h2, S::PropertyKeyInit>
	where
		'h: 'h2,
		S::PropertyKey: IsUninit
	{
		unsafe {
			self.inner.property_key
				.as_mut_ptr()
				.cast_write(property_key);
			self.change_state()
		}
	}

	#[inline(always)]
	pub fn value<'h2>(
		mut self,
		// todo do the type for value properly
		value: &'h2 JsValue
	) -> Builder<'h2, S::ValueInit>
	where
		'h: 'h2,
		S::Value: IsUninit
	{
		unsafe {
			self.inner.value
				.as_mut_ptr()
				.cast_write(value);
			self.change_state()
		}
	}

	#[inline(always)]
	pub fn receiver<'h2>(
		mut self,
		// todo do the type for receiver properly
		receiver: &'h2 JsValue
	) -> Builder<'h2, S::ReceiverInit>
	where
		'h: 'h2,
		S::Receiver: IsUninit
	{
		unsafe {
			self.inner.receiver
				.as_mut_ptr()
				.cast_write(receiver);
			self.change_state()
		}
	}

	#[inline(always)]
	unsafe fn change_state<'h2, S2>(self) -> Builder<'h2, S2>
	where
		'h: 'h2
	{
		Builder {
			inner: self.inner,
			__marker: PhantomData
		}
	}
}
