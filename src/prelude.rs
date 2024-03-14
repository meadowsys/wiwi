//! Prelude module, exporting types from enabled features

#[cfg(all(feature = "clock-timer", not(feature = "clock-timer-2")))]
pub use crate::clock_timer::{
	ClockTimer,
	ClockTimerError,
	TimeDelta,
	Timelike
};

#[cfg(all(feature = "clock-timer-2", not(feature = "clock-timer")))]
pub use crate::clock_timer_2::{
	ClockTimer,
	Tick,
	chrono::{
		DateTime,
		Local,
		TimeDelta
	}
};

#[cfg(feature = "debounce")]
pub use crate::debounce::{
	debounce,
	debounce_immediate,
	debounce_with_rt,
	debounce_immediate_with_rt
};

#[cfg(feature = "h")]
pub use crate::h::h;

#[cfg(feature = "lazy-wrap")]
pub use crate::lazy_wrap::{
	LazyWrap,
	LazyWrapState
};

#[cfg(feature = "string-pool")]
pub use crate::string_pool::String;
