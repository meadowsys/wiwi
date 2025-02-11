use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject<'h, S: State> {
	inner: Inner,
	__marker: PhantomDataBuilder<'h, S>
}

struct Inner {}

pub trait State {}

mod raw {
	use super::*;
	use wasm_bindgen::JsValue;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = Object
		)]
		pub(crate) static OBJECT: JsValue;
	}
}
