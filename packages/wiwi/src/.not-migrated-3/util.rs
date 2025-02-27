use crate::rust_std::boxed::Box;
use crate::rust_std::marker::{ Send, Sync };
use crate::rust_std::ops::Fn;
use crate::rust_std::panic::{ self, PanicHookInfo };

/// Augment the panic hook, adding a closure with your own code to
/// be run before the currently set panic hook
#[inline]
pub fn augment_panic_hook(hook: impl Fn(&PanicHookInfo<'_>) + Send + Sync + 'static) {
	let old = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		hook(panic_info);
		old(panic_info);
	}))
}

/// Convenience macro that declares many private submodules, and glob reexports
/// all items from them
///
/// See [module docs](self) for more info
#[macro_export]
macro_rules! export_all_submodules {
	{ $($mod:ident)* } => {
		$(
			mod $mod;
			pub use self::$mod::*;
		)*
	}
}
pub use export_all_submodules;
