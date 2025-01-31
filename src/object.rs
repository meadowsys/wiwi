use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[expect(clippy::new_without_default, reason = "shut")]
	#[inline(always)]
	pub fn new() -> Self {
		let inner = ExternAny::from_js_value(raw::new_object());
		Self { inner }
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}

mod raw {
	use super::*;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(js_name = Object)]
		pub(crate) fn new_object() -> JsValue;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::prelude_test::*;

	#[wasm_bindgen_test]
	fn object_creation_doesnt_die() {
		// todo remove me or something
		std::hint::black_box(ExternObject::new());
	}
}
