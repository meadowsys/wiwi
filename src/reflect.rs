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

/// Global `Reflect` namespace
#[repr(transparent)]
pub struct ExternReflectNs {
	inner: ExternObject
}

impl ExternReflectNs {
	#[inline]
	fn as_object(&self) -> &ExternObject {
		&self.inner
	}

	/// Create builder for `Reflect.apply()`
	#[inline]
	pub fn apply(&self) -> apply::Builder<'static, apply::StateUninit> {
		apply::Builder::new()
	}

	/// Create builder for `Reflect.construct()`
	#[inline]
	pub fn construct(&self) -> construct::Builder<'static, construct::StateUninit> {
		construct::Builder::new()
	}

	/// Create builder for `Reflect.defineProperty()`
	#[inline]
	pub fn define_property(&self) -> define_property::Builder<'static, define_property::StateUninit> {
		define_property::Builder::new()
	}

	/// Create builder for `Reflect.deleteProperty()`
	#[inline]
	pub fn delete_property(&self) -> delete_property::Builder<'static, delete_property::StateUninit> {
		delete_property::Builder::new()
	}

	/// Create builder for `Reflect.get()`
	#[inline]
	pub fn get(&self) -> get::Builder<'static, get::StateUninit> {
		get::Builder::new()
	}

	/// Create builder for `Reflect.getOwnPropertyDescriptor()`
	#[inline]
	pub fn get_own_property_descriptor(&self) -> get_own_property_descriptor::Builder<'static, get_own_property_descriptor::StateUninit> {
		get_own_property_descriptor::Builder::new()
	}

	/// Create builder for `Reflect.getPrototypeOf()`
	#[inline]
	pub fn get_prototype_of(&self) -> get_prototype_of::Builder<'static, get_prototype_of::StateUninit> {
		get_prototype_of::Builder::new()
	}

	/// Create builder for `Reflect.has()`
	#[inline]
	pub fn has(&self) -> has::Builder<'static, has::StateUninit> {
		has::Builder::new()
	}

	/// Create builder for `Reflect.isExtensible()`
	#[inline]
	pub fn is_extensible(&self) -> is_extensible::Builder<'static, is_extensible::StateUninit> {
		is_extensible::Builder::new()
	}

	/// Create builder for `Reflect.ownKeys()`
	#[inline]
	pub fn own_keys(&self) -> own_keys::Builder<'static, own_keys::StateUninit> {
		own_keys::Builder::new()
	}

	/// Create builder for `Reflect.preventExtensions()`
	#[inline]
	pub fn prevent_extensions(&self) -> prevent_extensions::Builder<'static, prevent_extensions::StateUninit> {
		prevent_extensions::Builder::new()
	}

	/// Create builder for `Reflect.set()`
	#[inline]
	pub fn set(&self) -> set::Builder<'static, set::StateUninit> {
		set::Builder::new()
	}

	/// Create builder for `Reflect.setPrototypeOf()`
	#[inline]
	pub fn set_prototype_of(&self) -> set_prototype_of::Builder<'static, set_prototype_of::StateUninit> {
		set_prototype_of::Builder::new()
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

		#[wasm_bindgen(
			js_namespace = Reflect,
			catch
		)]
		pub(crate) unsafe fn apply(
			target: &JsValue,
			this_argument: &JsValue,
			// array-like
			arguments_list: &JsValue,
			// returns result of calling given target function
			// throws TypeError
		) -> Result<JsValue, JsValue>;

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
