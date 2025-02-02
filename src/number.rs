use crate::prelude_internal::*;

#[inline]
pub fn number() -> ExternNumberNs {
	let inner = raw::NUMBER.with(Clone::clone);
	let inner = unsafe { ExternObject::from_js_value_unchecked(inner) };
	ExternNumberNs { inner }
}
#[repr(transparent)]
pub struct ExternNumberNs {
	inner: ExternObject
}

#[repr(transparent)]
pub struct ExternNumber {
	inner: ExternAny
}

#[repr(transparent)]
pub struct ExternNumberObject {
	inner: ExternObject
}

mod raw {
	use super::*;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = Number
		)]
		pub(crate) static NUMBER: JsValue;
	}
}
