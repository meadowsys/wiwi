use crate::prelude_internal::*;

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
		// 	arguments_list: &JsValue,
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = construct,
		// 	catch
		// )]
		// pub(crate) unsafe fn construct2(
		// 	target: &JsValue,
		// 	arguments_list: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = construct,
		// 	catch
		// )]
		// pub(crate) unsafe fn construct3(
		// 	target: &JsValue,
		// 	arguments_list: &JsValue,
		// 	new_target: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = defineProperty,
		// 	catch
		// )]
		// pub(crate) unsafe fn define_property(
		// 	target: &JsValue,
		// 	property_key: &JsValue,
		// 	attributes: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = deleteProperty,
		// 	catch
		// )]
		// pub(crate) unsafe fn delete_property(
		// 	target: &JsValue,
		// 	property_key: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = get,
		// 	catch
		// )]
		// pub(crate) unsafe fn get2(
		// 	target: &JsValue,
		// 	property_key: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = get,
		// 	catch
		// )]
		// pub(crate) unsafe fn get3(
		// 	target: &JsValue,
		// 	property_key: &JsValue,
		// 	receiver: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = getOwnPropertyDescriptor,
		// 	catch
		// )]
		// pub(crate) unsafe fn get_own_property_descriptor(
		// 	target: &JsValue,
		// 	property_key: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = getPrototypeOf,
		// 	catch
		// )]
		// pub(crate) unsafe fn get_prototype_of(
		// 	target: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	catch
		// )]
		// pub(crate) unsafe fn has(
		// 	target: &JsValue,
		// 	property_key: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = isExtensible,
		// 	catch
		// )]
		// pub(crate) unsafe fn is_extensible(
		// 	target: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = ownKeys,
		// 	catch
		// )]
		// pub(crate) unsafe fn own_keys(
		// 	target: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = preventExtensions,
		// 	catch
		// )]
		// pub(crate) unsafe fn prevent_extensions(
		// 	target: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = set,
		// 	catch
		// )]
		// pub(crate) unsafe fn set3(
		// 	target: &JsValue,
		// 	property_key: &JsValue,
		// 	value: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = set,
		// 	catch
		// )]
		// pub(crate) unsafe fn set4(
		// 	target: &JsValue,
		// 	property_key: &JsValue,
		// 	value: &JsValue,
		// 	receiver: &JsValue
		// ) -> Result<JsValue, JsValue>;

		// #[wasm_bindgen(
		// 	js_namespace = Reflect,
		// 	js_name = setPrototypeOf,
		// 	catch
		// )]
		// pub(crate) unsafe fn set_prototype_of(
		// 	target: &JsValue,
		// 	prototype: &JsValue
		// ) -> Result<JsValue, JsValue>;
	}
}
