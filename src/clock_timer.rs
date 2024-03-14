//! An interval tracking clock.
//!
//! This one is deprecated in favour of the one available by enabling feature
//! `clock-timer-2`

#![deprecated = "use feature clock-timer-2 instead"]

super::runtime_selection_compile_check!("clock-timer");

#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "tokio")]
mod tokio {
	use ::chrono::{ DateTime, Local, NaiveDateTime, ParseError };
	pub use ::chrono::{ TimeDelta, Timelike };
	use ::thiserror::Error;

	pub const SECS_PER_MIN: u64 = 60;
	pub const SECS_PER_HOUR: u64 = SECS_PER_MIN * 60;
	pub const SECS_PER_DAY: u64 = SECS_PER_HOUR * 24;

	/// returns a [`NaiveDateTime`] representing now.
	pub fn now() -> NaiveDateTime {
		Local::now().naive_local()
	}

	#[derive(Debug)]
	pub struct ClockTimer {
		next_tick: NaiveDateTime,
		end: NaiveDateTime,
		interval: TimeDelta,
	}

	impl ClockTimer {
		pub fn with_start_rfc3339(date: &str) -> Result<Self, ClockTimerError> {
			let next_tick = DateTime::parse_from_rfc3339(date)?.naive_local();
			Ok(Self::with_naive_datetime(next_tick))
		}

		pub fn with_start_custom_fmt(date: &str, format: &str) -> Result<Self, ClockTimerError> {
			let next_tick = NaiveDateTime::parse_from_str(date, format)?;
			Ok(Self::with_naive_datetime(next_tick))
		}

		pub fn with_naive_datetime(date: NaiveDateTime) -> Self {
			let next_tick = date;
			let duration = TimeDelta::try_seconds(SECS_PER_HOUR as _).unwrap();
			let end = date + duration;
			let interval = TimeDelta::try_seconds(1).unwrap();

			Self { next_tick, end, interval }
		}

		/// Sets the total run duration. This will tick once per interval until
		/// this duration has elapsed.
		pub fn set_run_duration(&mut self, duration: TimeDelta) {
			if duration < TimeDelta::zero() {
				panic!("provided run duration must not be negative");
			}

			self.end = self.next_tick + duration;
		}

		/// Sets run interval. This will tick once per the provided duration, until the
		/// total run duration has elapsed.
		pub fn set_run_interval(&mut self, interval: TimeDelta) {
			if interval < TimeDelta::zero() {
				panic!("provided run interval must not be negative");
			}

			self.interval = interval;
		}

		pub async fn tick(&mut self) -> Option<NaiveDateTime> {
			// get now
			let now = Local::now().naive_local();

			// if we're after end date, return
			if self.end <= now { return None }

			// store this tick time that we're trying to wait to
			let this_tick = self.next_tick;

			// get diff from now to target tick
			let delta = this_tick - now;

			// move next tick ahead by interval time
			self.next_tick += self.interval;

			// if we're ahead of current tick, just return it
			if delta < TimeDelta::zero() { return Some(this_tick) }

			tokio::time::sleep(delta.to_std().unwrap()).await;
			Some(this_tick)
		}
	}

	#[derive(Debug, Error)]
	pub enum ClockTimerError {
		#[error(transparent)]
		ChronoParseError(#[from] ParseError)
	}
}
