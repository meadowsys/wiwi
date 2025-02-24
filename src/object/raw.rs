use crate::prelude_internal::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern {
	#[wasm_bindgen(
		thread_local_v2,
		js_name = "Object"
	)]
	pub(crate) static OBJECT: JsValue;
}
