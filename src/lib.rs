#![allow(clippy::should_implement_trait)]

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "lazy-wrap")] {
		mod lazy_wrap;
		pub use lazy_wrap::{ LazyWrap, LazyWrapState };
	}
}
