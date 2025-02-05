use crate::prelude_internal::*;

/// Get the global `BigInt` namespace object
#[inline]
pub fn bigint() -> ExternBigintNs {
	let inner = raw::BIGINT.with(Clone::clone);
	let inner = ExternAny::from_js_value(inner);
	let inner = unsafe { ExternObject::from_any_unchecked(inner) };
	ExternBigintNs { inner }
}

/// Global `BigInt` namespace
#[repr(transparent)]
pub struct ExternBigintNs {
	inner: ExternObject
}

/// `bigint` primitive type
#[repr(transparent)]
pub struct ExternBigint {
	inner: ExternAny
}

impl ExternBigint {
	#[inline]
	pub fn as_any(&self) -> &ExternAny {
		&self.inner
	}
}

impl Deref for ExternBigint {
	type Target = ExternAny;

	#[inline]
	fn deref(&self) -> &ExternAny {
		self.as_any()
	}
}

// todo ??
// pub struct ExternBigintObject {}

mod raw {
	use super::*;
	use wasm_bindgen::JsValue;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = BigInt
		)]
		pub(crate) static BIGINT: JsValue;
	}
}
