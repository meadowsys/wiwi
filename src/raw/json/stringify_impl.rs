use crate::internal_prelude_raw::*;
use crate::builder_api::*;

#[expect(
	non_camel_case_types,
	reason = "mimicking method name"
)]
pub struct stringify;

impl stringify {
	#[inline(always)]
	pub fn value(self, value: &JsValue) -> Builder<'_, StateValueInit> {
		Builder::new().value(value)
	}

	#[inline(always)]
	pub fn replacer(self, replacer: &JsValue) -> Builder<'_, StateReplacerInit> {
		Builder::new().replacer(replacer)
	}

	#[inline(always)]
	pub fn space(self, space: &JsValue) -> Builder<'_, StateSpaceInit> {
		Builder::new().space(space)
	}
}

#[repr(transparent)]
pub struct Builder<'h, S> {
	inner: BuilderInner<'h>,
	__marker: PhantomDataInvariant<S>
}

struct BuilderInner<'h> {
	value: MaybeUninit<&'h JsValue>,
	replacer: MaybeUninit<&'h JsValue>,
	space: MaybeUninit<&'h JsValue>,
}

pub trait State {
	type Value: InitStatus;
	type ValueInit: State;

	type Replacer: InitStatus;
	type ReplacerInit: State;

	type Space: InitStatus;
	type SpaceInit: State;
}

pub struct StateContainer<Value, Replacer, Space> {
	__marker: PhantomDataInvariant<(Value, Replacer, Space)>
}

pub type StateUninit = StateContainer<Uninit, Uninit, Uninit>;
pub type StateValueInit = <StateUninit as State>::ValueInit;
pub type StateReplacerInit = <StateUninit as State>::ReplacerInit;
pub type StateSpaceInit = <StateUninit as State>::SpaceInit;

impl<
	Value: InitStatus,
	Replacer: InitStatus,
	Space: InitStatus
> State for StateContainer<Value, Replacer, Space> {
	type Value = Value;
	type ValueInit = StateContainer<Init, Replacer, Space>;

	type Replacer = Replacer;
	type ReplacerInit = StateContainer<Value, Init, Space>;

	type Space = Space;
	type SpaceInit = StateContainer<Value, Replacer, Init>;
}

impl Builder<'static, StateUninit> {
	#[inline(always)]
	fn new() -> Self {
		Self {
			inner: BuilderInner {
				value: uninit(),
				replacer: uninit(),
				space: uninit()
			},
			__marker: PhantomData
		}
	}
}

impl Builder<'_, StateContainer<Init, Uninit, Uninit>> {
	/// Calls `JSON.stringify(value)`
	#[inline(always)]
	pub fn call(self) -> Result<JsString, JsValue> {
		let value = unsafe { self.inner.value.assume_init() };
		js_sys::JSON::stringify(value)
	}
}

impl Builder<'_, StateContainer<Init, Init, Uninit>> {
	/// Calls `JSON.stringify(value, replacer)`
	#[inline(always)]
	pub fn call(self) -> Result<JsString, JsValue> {
		let value = unsafe { self.inner.value.assume_init() };
		let replacer = unsafe { self.inner.replacer.assume_init() };
		js_sys::JSON::stringify_with_replacer(value, replacer)
	}
}

impl Builder<'_, StateContainer<Init, Uninit, Init>> {
	/// Calls `JSON.stringify(value, null, space)`
	#[inline(always)]
	pub fn call(self) -> Result<JsString, JsValue> {
		let value = unsafe { self.inner.value.assume_init() };
		let replacer = &JsValue::null();
		let space = unsafe { self.inner.space.assume_init() };
		js_sys::JSON::stringify_with_replacer_and_space(value, replacer, space)
	}
}

impl Builder<'_, StateContainer<Init, Init, Init>> {
	/// Calls `JSON.stringify(value, replacer, space)`
	#[inline(always)]
	pub fn call(self) -> Result<JsString, JsValue> {
		let value = unsafe { self.inner.value.assume_init() };
		let replacer = unsafe { self.inner.replacer.assume_init() };
		let space = unsafe { self.inner.space.assume_init() };
		js_sys::JSON::stringify_with_replacer_and_space(value, replacer, space)
	}
}


impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline(always)]
	pub fn value<'h2>(mut self, value: &'h2 JsValue) -> Builder<'h2, S::ValueInit>
	where
		'h: 'h2,
		S::Value: IsUninit
	{
		unsafe {
			self.value_ptr().write(value);
			self.change_state()
		}
	}

	#[inline(always)]
	pub fn replacer<'h2>(mut self, replacer: &'h2 JsValue) -> Builder<'h2, S::ReplacerInit>
	where
		'h: 'h2,
		S::Replacer: IsUninit
	{
		unsafe {
			self.replacer_ptr().write(replacer);
			self.change_state()
		}
	}

	#[inline(always)]
	pub fn space<'h2>(mut self, space: &'h2 JsValue) -> Builder<'h2, S::SpaceInit>
	where
		'h: 'h2,
		S::Space: IsUninit
	{
		unsafe {
			self.space_ptr().write(space);
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

	/// # Safety
	///
	/// The pointer returned must only be written to (the lifetime of the returned
	/// pointer is contravariant over `'h`)
	#[inline(always)]
	unsafe fn value_ptr<'h2>(&mut self) -> *mut &'h2 JsValue
	where
		'h: 'h2
	{
		self.inner.value.as_mut_ptr().cast()
	}

	/// # Safety
	///
	/// The pointer returned must only be written to (the lifetime of the returned
	/// pointer is contravariant over `'h`)
	#[inline(always)]
	unsafe fn replacer_ptr<'h2>(&mut self) -> *mut &'h2 JsValue
	where
		'h: 'h2
	{
		self.inner.replacer.as_mut_ptr().cast()
	}

	/// # Safety
	///
	/// The pointer returned must only be written to (the lifetime of the returned
	/// pointer is contravariant over `'h`)
	#[inline(always)]
	unsafe fn space_ptr<'h2>(&mut self) -> *mut &'h2 JsValue
	where
		'h: 'h2
	{
		self.inner.space.as_mut_ptr().cast()
	}
}
