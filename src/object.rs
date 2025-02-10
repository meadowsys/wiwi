use crate::prelude_internal::*;

gen_builder! {
	ExternObject {
		deref: ExternAny;

		target {
			ty: Target;
			ty_init: TargetInit;
			ty_slot: TargetSlot;
		}

		property_key {
			ty: PropertyKey;
			ty_init: PropertyKeyInit;
			ty_slot: PropertyKeySlot;
		}

		value {
			ty: Value;
			ty_init: ValueInit;
			ty_slot: ValueSlot;
		}

		receiver {
			ty: Receiver;
			ty_init: ReceiverInit;
			ty_slot: ReceiverSlot;
		}
	}
}
