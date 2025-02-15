use crate::prelude_internal::*;

// #[repr(transparent)]
// pub struct ExternObject<'h, S: State> {
// 	inner: Inner<'h>,
// 	__marker: PhantomDataBuilder<'h, S>
// }

// struct Inner<'h> {
// 	__deref: Option<ExternAny>,
// 	value: ValueSlot<'h>
// }

gen_struct! {
	struct ExternObject;

	deref ExternAny;
	deref_value {
		let value = raw::OBJECT.with(Clone::clone);
		ExternAny::from_js_value(value)
	};

	field value: ValueSlot;
}

gen_state! {
	field Value;
	init ValueInit;
}

gen_slot! {
	slot ValueSlot;
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
	}
}
