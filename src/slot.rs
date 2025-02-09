use crate::{ ExternAny, ExternObject };
use crate::util::{ Slot, SlotUnchecked, gen_slot };

gen_slot! {
	/// Slot type accepting all objects
	ObjectSlot
	field any { &'h ExternAny }

	impl for { &'h ExternObject } {
		result { &'h ExternAny }
		write(self, slot) {
			*slot = ObjectSlot { any: self }
		}
		read(slot) {
			unsafe { slot.any }
		}
		as_ref(result) {
			result
		}
	}
}
