#![doc = include_str!("../README.md")]

#![allow(clippy::should_implement_trait)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]

// TODO: remove when more finished
#![allow(dead_code, unused_imports, unused_variables)]

use ::cfg_if::cfg_if;

pub mod prelude;

#[cfg(feature = "clock-timer")]
pub mod clock_timer;

#[cfg(feature = "clock-timer-2")]
pub mod clock_timer_2;

#[cfg(feature = "debounce")]
pub mod debounce;
feature_cfg_compile_check!("debounce-dyn-fn", cfg of "debounce");

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

// misc other checks

#[cfg(all(feature = "clock-timer", feature = "clock-timer-2"))]
compile_error!("Cannot have both `clock-timer` and `clock-timer-2` features enabled");

// macros and stuff

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

/// has to be run in this module and not in the feature modules themselves
/// because then, if this *should have* triggered an error, it won't because
/// the feature is off and module excluded from compilation lol
macro_rules! feature_cfg_compile_check {
	($cfgname:literal, cfg of $featname:literal) => {
		#[cfg(all(
			feature = $cfgname,
			not(feature = $featname)
		))]
		compile_error!(concat!("`", $cfgname, "` is a configuration feature of `", $featname, "`, and does nothing when enabled on its own"));
	}
}
use feature_cfg_compile_check;
