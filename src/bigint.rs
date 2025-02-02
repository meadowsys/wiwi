use crate::prelude_internal::*;

#[inline]
pub fn bigint() -> ExternBigintNs {
	let inner = raw::BIGINT.with(Clone::clone);
	let inner = unsafe { ExternObject::from_js_value_unchecked(inner) };
	ExternBigintNs { inner }
}

#[repr(transparent)]
pub struct ExternBigintNs {
	inner: ExternObject
}

#[repr(transparent)]
pub struct ExternBigint {
	inner: ExternAny
}

// todo ??
// pub struct ExternBigintObject {}

mod raw {
	use super::*;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = BigInt
		)]
		pub(crate) static BIGINT: JsValue;
	}
}
