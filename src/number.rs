use crate::prelude_internal::*;

/// Get the global `Number` namespace object
#[inline]
pub fn number() -> ExternNumberNs {
	let inner = raw::NUMBER.with(Clone::clone);
	let inner = ExternAny::from_js_value(inner);
	let inner = unsafe { ExternObject::from_any_unchecked(inner) };
	ExternNumberNs { inner }
}

/// Global `Number` namespace
#[repr(transparent)]
pub struct ExternNumberNs {
	inner: ExternObject
}

/// `number` primitive type
#[repr(transparent)]
pub struct ExternNumber {
	inner: ExternAny
}

impl ExternNumber {
	#[inline]
	pub fn as_any(&self) -> &ExternAny {
		&self.inner
	}
}

impl Deref for ExternNumber {
	type Target = ExternAny;

	#[inline]
	fn deref(&self) -> &ExternAny {
		self.as_any()
	}
}

/// `Number` wrapper object
#[repr(transparent)]
pub struct ExternNumberObject {
	inner: ExternObject
}

mod raw {
	use super::*;
	use wasm_bindgen::JsValue;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = Number
		)]
		pub(crate) static NUMBER: JsValue;
	}
}
