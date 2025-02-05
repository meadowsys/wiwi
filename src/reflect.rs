use crate::prelude_internal::*;

pub mod apply;
pub mod construct;
pub mod define_property;
pub mod delete_property;
pub mod get;
pub mod get_own_property_descriptor;
pub mod get_prototype_of;
pub mod has;
pub mod is_extensible;
pub mod own_keys;
pub mod prevent_extensions;
pub mod set;
pub mod set_prototype_of;

/// Get the global `Reflect` namespace object
#[inline]
pub fn reflect() -> ExternReflectNs {
	let inner = raw::REFLECT.with(Clone::clone);
	let inner = ExternAny::from_js_value(inner);
	let inner = unsafe { ExternObject::from_any_unchecked(inner) };
	ExternReflectNs { inner }
}

#[repr(transparent)]
pub struct ExternReflectNs {
	inner: ExternObject
}

impl ExternReflectNs {
	#[inline]
	pub fn as_object(&self) -> &ExternObject {
		&self.inner
	}

	// pub fn apply(&self, ..) -> buildersomething

	#[inline]
	pub fn set(&self) -> set::Builder<'static, set::StateUninit> {
		set::Builder::new()
	}
}

impl Deref for ExternReflectNs {
	type Target = ExternObject;

	#[inline]
	fn deref(&self) -> &ExternObject {
		self.as_object()
	}
}

mod raw {
	use super::*;
	use wasm_bindgen::JsValue;

	#[wasm_bindgen]
	extern {
		#[wasm_bindgen(
			thread_local_v2,
			js_name = Reflect
		)]
		pub(crate) static REFLECT: JsValue;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	catch
		// )]
		// pub(crate) unsafe fn apply(
		// 	target: &JsValue,
		// 	this_argument: &JsValue,
		// 	// array-like
		// 	arguments_list: &JsValue,
		// 	// returns result of calling given target function
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = construct,
		// 	catch
		// )]
		// pub(crate) unsafe fn construct2(
		// 	// function
		// 	target: &JsValue,
		// 	// array-like
		// 	arguments_list: &JsValue
		// 	// returns new instance of target, init by target as constructor
		// 	// and given arguments_list
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = construct,
		// 	catch
		// )]
		// pub(crate) unsafe fn construct3(
		// 	// function
		// 	target: &JsValue,
		// 	// array-like
		// 	arguments_list: &JsValue,
		// 	new_target: &JsValue
		// 	// returns new instance of new_target, init by target as constructor
		// 	// and given arguments_list
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = defineProperty,
		// 	catch
		// )]
		// pub(crate) unsafe fn define_property(
		// 	// object
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue,
		// 	attributes: &JsValue
		// 	// returns bool
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = deleteProperty,
		// 	catch
		// )]
		// pub(crate) unsafe fn delete_property(
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue
		// 	// returns bool
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = get,
		// 	catch
		// )]
		// pub(crate) unsafe fn get2(
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue
		// 	// returns value of property
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = get,
		// 	catch
		// )]
		// pub(crate) unsafe fn get3(
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue,
		// 	receiver: &JsValue
		// 	// returns value of property
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = getOwnPropertyDescriptor,
		// 	catch
		// )]
		// pub(crate) unsafe fn get_own_property_descriptor(
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue
		// 	// returns property descriptor object or undefined
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = getPrototypeOf,
		// 	catch
		// )]
		// pub(crate) unsafe fn get_prototype_of(
		// 	target: &JsValue
		// 	// returns prototype (obj or null)
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	catch
		// )]
		// pub(crate) unsafe fn has(
		// 	target: &JsValue,
		// 	// name of property (string? or autocast?)
		// 	property_key: &JsValue
		// 	// returns bool
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = isExtensible,
		// 	catch
		// )]
		// pub(crate) unsafe fn is_extensible(
		// 	target: &JsValue
		// 	// returns bool
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = ownKeys,
		// 	catch
		// )]
		// pub(crate) unsafe fn own_keys(
		// 	target: &JsValue
		// 	// returns array of target's own property keys
		// 	// (incl strings and symbols)
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;

		#[wasm_bindgen(
			js_namespace = Reflect,
			js_name = preventExtensions,
			catch
		)]
		pub(crate) unsafe fn prevent_extensions(
			target: &JsValue
			// returns bool
			// throws TypeError
		) -> Result<JsValue, JsValue>;

		#[wasm_bindgen(
			js_namespace = Reflect,
			js_name = set,
			catch
		)]
		pub(crate) unsafe fn set3(
			target: &JsValue,
			// name of property (string? or autocast?)
			property_key: &JsValue,
			// value to set
			value: &JsValue
			// returns bool
			// throws TypeError
		) -> Result<JsValue, JsValue>;

		#[wasm_bindgen(
			js_namespace = Reflect,
			js_name = set,
			catch
		)]
		pub(crate) unsafe fn set4(
			target: &JsValue,
			// name of property (string? or autocast?)
			property_key: &JsValue,
			value: &JsValue,
			receiver: &JsValue
			// returns bool
			// throws TypeError
		) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = setPrototypeOf,
		// 	catch
		// )]
		// pub(crate) unsafe fn set_prototype_of(
		// 	target: &JsValue,
		// 	// new prototype (object or null)
		// 	prototype: &JsValue
		// 	// returns bool
		// 	// throws TypeError
		// ) -> Result<JsValue, JsValue>;
	}
}
