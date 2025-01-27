use crate::{ builder_api::*, internal_prelude::* };

#[inline(always)]
pub fn parse() -> BuilderUninit {
	Builder::new()
}

pub type BuilderUninit = Builder<'static, StateContainer<Uninit, Uninit>>;

#[repr(transparent)]
pub struct Builder<'h, S> {
	str: MaybeUninit<BuilderStr<'h>>,
	__marker: PhantomDataInvariant<S>
}

union BuilderStr<'h> {
	rs: &'h str,
	js: ManuallyDrop<JsString>
}

pub trait State {
	type StrRs: InitStatus;
	type InitStrRs: State;

	type StrJs: InitStatus;
	type InitStrJs: State;
}

pub struct StateContainer<StrRust, StrJs> {
	__marker: PhantomDataInvariant<(StrRust, StrJs)>
}

impl<
	StrRust: InitStatus,
	StrJs: InitStatus
> State for StateContainer<StrRust, StrJs> {
	type StrRs = StrRust;
	type InitStrRs = StateContainer<Init, StrJs>;

	type StrJs = StrJs;
	type InitStrJs = StateContainer<StrRust, Init>;
}

/// Trait for [`json::parse`](parse) for string types that can be used as input
///
/// # Safety
///
/// The provided [`ReturnState`](UsableAsStr::ReturnState) must properly encode
/// the new state based on the old state.
pub unsafe trait UsableAsStr<'h> {
	type ReturnState<S>: State where S: State;

	fn put_str_in<S>(self, builder: Builder<'h, S>) -> Builder<'h, Self::ReturnState<S>>
	where
		S: State,
		S::StrJs: IsUninit,
		S::StrRs: IsUninit;
}

unsafe impl<'h> UsableAsStr<'h> for &'h str {
	type ReturnState<S> = S::InitStrRs where S: State;

	#[inline(always)]
	fn put_str_in<S>(self, mut builder: Builder<'h, S>) -> Builder<'h, Self::ReturnState<S>>
	where
		S: State,
		S::StrJs: IsUninit,
		S::StrRs: IsUninit
	{
		unsafe {
			let ptr = &raw mut (*builder.str.as_mut_ptr()).rs;
			ptr.write(self);
			builder.change_state()
		}
	}
}

unsafe impl UsableAsStr<'static> for JsString {
	type ReturnState<S> = S::InitStrRs where S: State;

	#[inline(always)]
	fn put_str_in<S>(self, mut builder: Builder<'static, S>) -> Builder<'static, Self::ReturnState<S>>
	where
		S: State,
		S::StrJs: IsUninit,
		S::StrRs: IsUninit
	{
		unsafe {
			let ptr = &raw mut (*builder.str.as_mut_ptr()).js;
			ptr.write(ManuallyDrop::new(self));
			builder.change_state()
		}
	}
}

impl BuilderUninit {
	#[inline(always)]
	fn new() -> Self {
		Self { str: uninit(), __marker: PhantomData }
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline(always)]
	pub fn str<'t, T>(self, s: T) -> Builder<'t, T::ReturnState<S>>
	where
		'h: 't,
		T: UsableAsStr<'t>,
		S::StrJs: IsUninit,
		S::StrRs: IsUninit,
	{
		s.put_str_in(self)
	}
}

impl Builder<'_, StateContainer<Init, Uninit>> {
	pub fn call(self) -> Result<JsValue, JsValue> {
		let s = unsafe { self.str.assume_init().rs };
		js_sys::JSON::parse(s)
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline(always)]
	unsafe fn change_state<'h2, S2>(self) -> Builder<'h2, S2>
	where
		'h: 'h2
	{
		Builder { str: self.str, __marker: PhantomData }
	}
}
