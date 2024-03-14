//! An interval tracking clock

super::runtime_selection_compile_check!("clock-timer-2");

#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "tokio")]
mod tokio {
	pub use ::chrono;

	use ::chrono::{ DateTime, Local, NaiveDateTime, TimeDelta, Timelike, TimeZone };
	use ::std::future::Future;
	use ::tokio::time::sleep;

	/// An interval tracking clock. Takes a start time, an end time or a run duration,
	/// and an interval. Calls to [`tick`][ClockTimer::tick] will return only if
	/// the current time is at or past the time of the next interval, waiting so
	/// that it is before returning. It yields timing information when returning.
	/// If this falls behind time for some reason, the ticks will be yielded with
	/// the time information at when it was supposed to yield, until catching up.
	pub struct ClockTimer {
		next_tick: DateTime<Local>,
		interval: TimeDelta,
		elapsed: TimeDelta,
		remaining: TimeDelta
	}

	/// Timing information for one tick
	pub struct Tick {
		this_tick: DateTime<Local>,
		elapsed: TimeDelta,
		remaining: TimeDelta
	}

	impl ClockTimer {
		/// Gets a [`ClockTimer`] builder
		#[inline]
		pub fn builder() -> builder::Builder {
			builder::Builder::new()
		}

		/// Runs the next tick and returns timing information for it, if this
		/// interval is not finished already.
		pub async fn tick(&mut self) -> Option<Tick> {
			if self.remaining < TimeDelta::zero() { return None }

			let tick = Tick {
				this_tick: self.next_tick,
				elapsed: self.elapsed,
				remaining: self.remaining
			};

			self.next_tick += self.interval;
			self.elapsed += self.interval;
			self.remaining -= self.interval;

			let delta = tick.this_tick - Local::now();

			if delta <= TimeDelta::zero() { return Some(tick) }
			sleep(delta.to_std().unwrap()).await;

			Some(tick)
		}

		/// Convenience function, equivalent to running a `while let Some(tick)`
		/// loop. When awaited on, the closure provided will be called every tick.
		/// This consumes self and runs it to completion.
		#[inline]
		pub async fn run_to_end<F, Fu>(mut self, mut f: F)
		where
			F: FnMut(Tick) -> Fu,
			Fu: Future<Output = ()>
		{
			while let Some(tick) = self.tick().await {
				f(tick).await;
			}
		}
	}

	pub mod builder {
		use super::*;

		pub struct Builder {
			__private: ()
		}

		impl Builder {
			/// New builder. You can also obtain a builder through [`ClockTimer::builder`]
			#[inline]
			pub fn new() -> Self {
				// its gonna be noop lol
				Self { __private: () }
			}

			/// Sets the start date/time of the ClockTimer, or in other words, the
			/// time of the first tick.
			#[inline]
			pub fn with_start_datetime<TZ: TimeZone>(self, datetime: DateTime<TZ>) -> BuilderWithStart {
				let start = datetime.with_timezone(&Local);
				BuilderWithStart { start }
			}
		}

		pub struct BuilderWithStart {
			start: DateTime<Local>
		}

		impl BuilderWithStart {
			/// Sets the end date/time of the ClockTimer. ClockTimer will run until
			/// this time is _passed_. A tick _will_ be emitted if the last tick is equal
			/// to the end time.
			#[inline]
			pub fn with_end_datetime<TZ: TimeZone>(self, datetime: DateTime<TZ>) -> BuilderWithEnd {
				let Self { start } = self;
				let end = datetime.with_timezone(&Local);
				BuilderWithEnd { start, end }
			}

			/// Sets a duration to run this ClockTimer for. Internally, the end time
			/// is calculated and stored based on start time and the provided duration.
			#[inline]
			pub fn with_duration(self, duration: TimeDelta) -> BuilderWithEnd {
				let Self { start } = self;
				let end = start + duration;
				BuilderWithEnd { start, end }
			}
		}

		pub struct BuilderWithEnd {
			start: DateTime<Local>,
			end: DateTime<Local>
		}

		impl BuilderWithEnd {
			/// Sets interval to run at, or the time between ticks.
			#[inline]
			pub fn with_interval(self, interval: TimeDelta) -> BuilderWithInterval {
				let Self { start, end } = self;
				BuilderWithInterval { start, end, interval }
			}
		}

		pub struct BuilderWithInterval {
			start: DateTime<Local>,
			end: DateTime<Local>,
			interval: TimeDelta
		}

		impl BuilderWithInterval {
			/// Builds and returns a [`ClockTimer`]
			#[inline]
			pub fn build(self) -> ClockTimer {
				let Self { start: next_tick, end, interval } = self;
				let elapsed = TimeDelta::zero();
				let remaining = end - next_tick;

				ClockTimer { next_tick, interval, elapsed, remaining }
			}
		}
	}
}
