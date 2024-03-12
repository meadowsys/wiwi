#![allow(clippy::should_implement_trait)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

// TODO: remove when more finished
#![allow(dead_code, unused_imports, unused_variables)]

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "lazy-wrap")] {
		mod lazy_wrap;
		pub use lazy_wrap::{ LazyWrap, LazyWrapState };
	}
}

cfg_if! {
	if #[cfg(feature = "string-pool")] {
		mod string_pool;
	}
}
