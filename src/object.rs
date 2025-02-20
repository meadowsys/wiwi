use crate::prelude_internal::*;

gen_struct! {
	struct ExternObject;

	deref ExternAny;
	deref_value {
		let value = raw::OBJECT.with(Clone::clone);
		ExternAny::from_js_value(value)
	};

	field value ValueSlot;
}

gen_state! {
	field Value;
	init ValueInit;
}

gen_slot! {
	slot ValueSlot;
}

gen_builder_fns! {
	struct ExternObject;

	state Value;
	init ValueInit;
	slot ValueSlot;

	field value;

	field_unchecked value_unchecked;
}

gen_call_fn! {
	struct ExternObject;
	raw_call {
		let _ = value;
		todo!()
	};
	return Result<ExternAny, ExternAny>;

	field value;
	state Value;
	init Init<ValueSlot>;
}

gen_call_fn! {
	struct ExternObject;
	raw_call {
		todo!()
	};
	return Result<ExternAny, ExternAny>;

	field value;
	state Value;
	init Uninit;
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
