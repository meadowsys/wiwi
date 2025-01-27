use crate::{ builder_api::*, internal_prelude::* };

pub fn stringify() {}

#[repr(transparent)]
pub struct Builder<'h, S> {
	inner: MaybeUninit<BuilderInner<'h>>,
	__marker: PhantomDataInvariant<S>
}

struct BuilderInner<'h> {
	value: &'h JsValue,
	replacer: &'h JsValue,
	space: &'h JsValue
}

// - todo submodule for builder impl details
//   - todo imports
//   - todo pub type for init/uninit
//   - todo builder struct def
//   - todo builder state trait def
//   - todo builder state container struct def
//   - todo impl builder state trait
//   - todo impl uninit for `new()` fn
//   - todo impl<S> where S: builder state trait for all the fns including `build()`
//     (`build()` calls `finish_init(..)`)
//   - todo impl block, same as previous one in headers and stuffs, for the internal fns
























// stringify(value, replacer, space) ...

// stringify(value)
// stringify(value, replacer)
// stringify(value, replacer, space)

// impl init, uninit, uninit
// impl init, init, uninit
// impl init, uninit, init
// impl init, init, init

// replacer can be function, or array of strings/numbers
