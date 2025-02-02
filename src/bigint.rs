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

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = BigInt
		)]
		pub(crate) static BIGINT: JsValue;
	}
}
