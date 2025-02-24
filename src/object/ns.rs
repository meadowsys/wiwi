// todo this file is incomplete

use crate::prelude_internal::*;
use super::raw;

gen_struct! {
	struct ExternObjectNs;

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

gen_builder_fns! {
	struct ExternObjectNs;

	state Value;
	init ValueInit;
	slot ValueSlot;

	field value;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		todo!()
	};
	return Result<ExternAny, ExternAny>;

	field value;
	state Value;
	init Uninit;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	return ExternAny;

	field value;
	state Value;
	init Init<ValueSlot, AnyMarker>;
}

gen_slot! {
	slot ValueSlot;
}

gen_slot_impl! {
	slot ValueSlot;
	impl &'h ExternAny;

	type_marker AnyMarker;
	result &'h ExternAny;

	simple_rw any;
	autoderef;
}
