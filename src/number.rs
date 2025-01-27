#[inline(always)]
pub fn number(number: impl ToExternNumber) -> ExternNumber {
	number.to_extern_number()
}

#[repr(transparent)]
pub struct ExternNumber {
	inner: wasm_bindgen::JsValue
}

pub trait ToExternNumber {
	fn to_extern_number(self) -> ExternNumber;
}

impl ToExternNumber for i32 {
	#[inline]
	fn to_extern_number(self) -> ExternNumber {
		ExternNumber { inner: wasm_bindgen::JsValue::from_f64(self as _) }
	}
}
