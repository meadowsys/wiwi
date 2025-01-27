use super::ExternValue;

#[repr(transparent)]
pub struct ExternBigInt {
	inner: wasm_bindgen::JsValue
}

impl TryFrom<ExternValue> for ExternBigInt {
	type Error = ();

	fn try_from(value: ExternValue) -> Result<Self, ()> {
		value.as_sys()
			.is_bigint()
			.then(|| Self { inner: value.into_sys() })
			.ok_or(())
	}
}
