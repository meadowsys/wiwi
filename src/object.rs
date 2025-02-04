use crate::prelude_internal::*;

#[inline]
pub fn object() -> ExternObjectNs {
	let inner = raw::OBJECT.with(Clone::clone);
	let inner = ExternAny::from_js_value(inner);
	let inner = unsafe { ExternObject::from_any_unchecked(inner) };
	ExternObjectNs { inner }
}

#[repr(transparent)]
pub struct ExternObjectNs {
	inner: ExternObject
}

impl ExternObjectNs {
	#[expect(
		clippy::new_ret_no_self,
		clippy::wrong_self_convention,
		reason = "shut"
	)]
	#[inline]
	pub fn new(&self) -> ExternObject {
		unsafe { ExternObject::from_any_unchecked(ExternAny::from_js_value(raw::new_object())) }
	}
}

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[inline]
	pub unsafe fn from_any_unchecked(value: ExternAny) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub unsafe fn from_any_ref_unchecked(value: &ExternAny) -> &Self {
		// SAFETY: ExternObject is repr(transparent) over ExternAny
		unsafe { &*(&raw const value as *const ExternObject) }
	}

	#[inline]
	pub fn as_any(&self) -> &ExternAny {
		&self.inner
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline]
	fn deref(&self) -> &ExternAny {
		self.as_any()
	}
}

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

		#[wasm_bindgen(js_name = Object)]
		pub(crate) unsafe fn new_object() -> JsValue;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::prelude_test::*;

	// todo remove me maybe?
	#[wasm_bindgen_test]
	fn object_creation_doesnt_die() {
		black_box(object().new());
	}
}
