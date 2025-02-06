use crate::{ ExternAny, ExternObject };
use crate::builder::*;

/// Slot type accepting all objects
pub union ObjectSlot<'h> {
	uninit: (),
	any: &'h ExternAny
}

impl ObjectSlot<'static> {
	#[inline]
	pub(crate) fn uninit() -> Self {
		Self { uninit: () }
	}
}

unsafe impl<'h> SlotUnchecked<ObjectSlot<'h>> for &'h ExternAny {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ObjectSlot<'h>) {
		*slot = ObjectSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ObjectSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}

unsafe impl<'h> Slot<ObjectSlot<'h>> for &'h ExternObject {}
unsafe impl<'h> SlotUnchecked<ObjectSlot<'h>> for &'h ExternObject {
	type Result = &'h ExternAny;

	#[inline]
	unsafe fn write(self, slot: &mut ObjectSlot<'h>) {
		*slot = ObjectSlot { any: self }
	}

	#[inline]
	unsafe fn read(slot: ObjectSlot<'h>) -> &'h ExternAny {
		unsafe { slot.any }
	}

	#[inline]
	fn as_ref(result: &&'h ExternAny) -> &'h ExternAny {
		result
	}
}
