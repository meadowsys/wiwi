use crate::prelude::*;
use crate::rc::RcThread;

/// Identifier incrementer for generating unique LSL identifiers
pub struct IdentIncrementer {
	next: RcThread<cell::Cell<u64>>
}

impl IdentIncrementer {
	pub fn new() -> Self {
		Self { next: RcThread::from_value(cell::Cell::new(0)) }
	}

	pub fn next(&self) -> Ident {
		let ident = self.next.as_value_ref().get();
		self.next.as_value_ref().set(ident.checked_add(1).unwrap());
		Ident { ident }
	}
}

impl Clone for IdentIncrementer {
	fn clone(&self) -> Self {
		Self { next: RcThread::clone(&self.next) }
	}
}

#[derive(Clone)]
pub struct Ident {
	ident: u64
}

impl Ident {
	fn to_u64(&self) -> u64 {
		self.ident
	}
}
