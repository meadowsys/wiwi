super::runtime_selection_compile_check!("clock-timer-2");

#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "tokio")]
mod tokio {
	pub use ::chrono;

	use ::chrono::{ DateTime, Local, NaiveDateTime, TimeDelta, Timelike, TimeZone };
	use ::std::future::Future;
	use ::tokio::time::sleep;

	pub struct ClockTimer {
		next_tick: DateTime<Local>,
		interval: TimeDelta,
		elapsed: TimeDelta,
		remaining: TimeDelta
	}

	pub struct Tick {
		this_tick: DateTime<Local>,
		elapsed: TimeDelta,
		remaining: TimeDelta
	}

	impl ClockTimer {
		#[inline]
		pub fn builder() -> builder::Builder {
			builder::Builder::new()
		}

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
			#[inline]
			pub fn new() -> Self {
				Self { __private: () }
			}

			#[inline]
			pub fn with_start_datetime<TZ: TimeZone>(self, datetime: DateTime<TZ>) -> BuilderWithStart {
				let start = datetime.with_timezone(&Local);
				BuilderWithStart { start }
			}

			// pub fn with_start_rfc3339(date: &str)
		}

		pub struct BuilderWithStart {
			start: DateTime<Local>
		}

		impl BuilderWithStart {
			#[inline]
			pub fn with_end_datetime<TZ: TimeZone>(self, datetime: DateTime<TZ>) -> BuilderWithEnd {
				let Self { start } = self;
				let end = datetime.with_timezone(&Local);
				BuilderWithEnd { start, end }
			}

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
