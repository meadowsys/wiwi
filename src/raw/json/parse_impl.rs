use crate::internal_prelude_raw::*;
use crate::builder_api::*;
use super::parse;

impl parse {
	#[inline(always)]
	pub fn text(self, text: &str) -> Builder<'_, StateTextInit> {
		Builder::new().text(text)
	}
}

#[repr(transparent)]
pub struct Builder<'h, S> {
	inner: BuilderInner<'h>,
	__marker: PhantomDataInvariant<S>
}

struct BuilderInner<'h> {
	text: MaybeUninit<&'h str>,
	#[expect(dead_code, reason = "not used for now")]
	reviver: MaybeUninit<()>
}

pub trait State {
	type Text: InitStatus;
	type TextInit: State;

	type Reviver: InitStatus;
	type ReviverInit: State;
}

pub struct StateContainer<Text, Reviver> {
	__marker: PhantomDataInvariant<(Text, Reviver)>
}

pub type StateUninit = StateContainer<Uninit, Uninit>;
pub type StateTextInit = <StateUninit as State>::TextInit;
pub type StateReviverInit = <StateUninit as State>::ReviverInit;

impl<Text, Reviver> State for StateContainer<Text, Reviver>
where
	Text: InitStatus,
	Reviver: InitStatus
{
	type Text = Text;
	type TextInit = StateContainer<Init, Reviver>;

	type Reviver = Reviver;
	type ReviverInit = StateContainer<Text, Init>;
}

impl Builder<'static, StateUninit> {
	#[inline(always)]
	fn new() -> Self {
		Self {
			inner: BuilderInner {
				text: uninit(),
				reviver: uninit(),
			},
			__marker: PhantomData
		}
	}
}

impl Builder<'_, StateTextInit> {
	#[inline(always)]
	pub fn call(self) -> Result<JsValue, JsValue> {
		let text = unsafe { self.inner.text.assume_init() };
		js_sys::JSON::parse(text)
	}
}

impl<'h, S> Builder<'h, S>
where
	S: State
{
	#[inline(always)]
	pub fn text<'h2>(mut self, text: &'h2 str) -> Builder<'h2, S::TextInit>
	where
		'h: 'h2
	{
		unsafe {
			self.text_ptr().write(text);
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
	unsafe fn text_ptr<'h2>(&mut self) -> *mut &'h2 str
	where
		'h: 'h2
	{
		self.inner.text.as_mut_ptr().cast()
	}
}
