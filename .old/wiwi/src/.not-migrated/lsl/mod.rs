use crate::id::{ IDGenerator64, GeneratedID64 };
use hashbrown::HashMap;
use std::marker::PhantomData;

mod lib;

pub struct Script {
	generator: IDGenerator64,
	vars: Vars,
	fns: Fns,
	state_default: State,
	states: HashMap<StateKey, State>
}

struct Vars {
	vars: Vec<Box<dyn Var>>
}

struct Fns {}

struct State {
	vars: Vars
}

#[derive(Clone, Copy, Hash)]
struct StateKey {
	id: GeneratedID64
}

struct ConcreteVar<T: Type> {
	ty: T,
	id: GeneratedID64,
	init_val: Option<String>
}

fn _assert_var_trait_object_safe(_: Box<dyn Var>) {}
trait Var {
	fn name(&self) -> String;
	fn init_val(&self) -> Option<String>;
}

impl<T: Type> Var for ConcreteVar<T> {
	fn name(&self) -> String {
		format!("var_{}", self.id.to_alphanumeric_string())
	}

	fn init_val(&self) -> Option<String> {
		self.init_val.clone()
	}
}

fn _assert_type_trait_object_safe(_: Box<dyn Type>) {}
trait Type {
	fn keyword(&self) -> &str;
}

mod ty {
	#[derive(Clone, Copy)]
	pub struct Float;
	#[derive(Clone, Copy)]
	pub struct Integer;
	#[derive(Clone, Copy)]
	pub struct Key;
	#[derive(Clone, Copy)]
	pub struct List;
	#[derive(Clone, Copy)]
	pub struct Rotation;
	#[derive(Clone, Copy)]
	pub struct String;
	#[derive(Clone, Copy)]
	pub struct Vector;
	#[derive(Clone, Copy)]
	pub struct Boolean;

	macro_rules! impl_type {
		{ $($ty:ident $keyword:literal)* } => {
			$(
				impl super::Type for $ty {
					fn keyword(&self) -> &str {
						$keyword
					}
				}
			)*
		}
	}

	impl_type! {
		Float "float"
		Integer "integer"
		Key "key"
		List "list"
		Rotation "rotation"
		String "string"
		Vector "vector"
		Boolean "integer"
	}
}
