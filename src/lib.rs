#![allow(clippy::should_implement_trait)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

// TODO: remove when more finished
#![allow(dead_code, unused_imports, unused_variables)]

use ::cfg_if::cfg_if;

#[cfg(feature = "clock-timer")]
pub mod clock_timer;

#[cfg(feature = "h")]
pub mod h;

#[cfg(feature = "lazy-wrap")]
pub mod lazy_wrap;

#[cfg(feature = "string-pool")]
pub mod string_pool;

// ensure max one runtime is selected
cfg_if! {
	if #[cfg(all(
		feature = "tokio"
		// not(any(
		// 	// other runtime features go here
		// ))
	))] {
		// only tokio
	} else if #[cfg(not(any(
		feature = "tokio"
	)))] {
		// no runtime selected, ignore
	} else {
		// more than one runtime selected
		compile_error!("more than one runtime feature enabled; make sure only one of `tokio` features are enabled (by the way, there is only one runtime available, how have you managed to trigger this?????)");
	}
}

#[allow(unused)]
macro_rules! runtime_selection_compile_check {
	($featname:literal) => {
		#[cfg(not(any(
			feature = "tokio"
		)))]
		compile_error!(concat!("an async runtime is required to make use of `", $featname, "`; available runtimes (enable by selecting the crate feature): `tokio`"));
	}
}
#[allow(unused)]
use runtime_selection_compile_check;
