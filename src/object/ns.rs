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
	// todo better return type
	return ExternObject<'static, crate::object::StateUninit>;

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
	// todo better return type
	return ExternAny;

	field value;
	state Value;
	init Init<ValueSlot, AnyMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return crate::ExternBigInt<'static, crate::bigint::StateUninit>;

	field value;
	state Value;
	init Init<ValueSlot, BigIntMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return crate::ExternBoolean<'static, crate::boolean::StateUninit>;

	field value;
	state Value;
	init Init<ValueSlot, BooleanMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return u8;

	field value;
	state Value;
	init Init<ValueSlot, NumberMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return ExternAny;

	field value;
	state Value;
	init Init<ValueSlot, ObjectMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return ExternAny;

	field value;
	state Value;
	init Init<ValueSlot, StringMarker>;
}

gen_call_fn! {
	struct ExternObjectNs;
	raw_call {
		let _ = value;
		todo!()
	};
	// todo better return type
	return ExternAny;

	field value;
	state Value;
	init Init<ValueSlot, SymbolMarker>;
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

gen_slot_impl! {
	slot ValueSlot;
	impl {
		S: crate::bigint::State
	} &'h crate::ExternBigInt<'h, S>;

	type_marker BigIntMarker;
	result &'h crate::ExternAny;

	simple_rw any;
	autoderef;
}

gen_slot_impl! {
	slot ValueSlot;
	impl {
		S: crate::boolean::State
	} &'h crate::ExternBoolean<'h, S>;

	type_marker BooleanMarker;
	result &'h crate::ExternAny;

	simple_rw any;
	autoderef;
}

gen_slot_impl! {
	slot ValueSlot;
	impl {
		S: crate::number::State
	} &'h crate::ExternNumber<'h, S>;

	type_marker NumberMarker;
	result &'h crate::ExternAny;

	simple_rw any;
	autoderef;
}

// gen_slot_impl! {
// 	slot ValueSlot;
// 	impl {
// 		S: crate::object::State
// 	} &'h crate::ExternObject<'h, S>;
//
// 	type_marker ObjectMarker;
// 	result &'h crate::ExternAny;
//
// 	simple_rw any;
// 	autoderef;
// }

gen_slot_impl! {
	slot ValueSlot;
	impl {
		S: crate::string::State
	} &'h crate::ExternString<'h, S>;

	type_marker StringMarker;
	result &'h crate::ExternAny;

	simple_rw any;
	autoderef;
}

gen_slot_impl! {
	slot ValueSlot;
	impl {
		S: crate::symbol::State
	} &'h crate::ExternSymbol<'h, S>;

	type_marker SymbolMarker;
	result &'h crate::ExternAny;

	simple_rw any;
	autoderef;
}
