pub extern crate chrono;
extern crate tokio;

use crate::prelude::*;
use chrono::{ DateTime, Local, TimeDelta, TimeZone };
use std::future::Future;
use tokio::time::sleep;

/// An interval tracking clock. Takes a start time, an end time or a run duration,
/// and an interval. Calls to [`tick`][ClockTimer::tick] will return only if
/// the current time is at or past the time of the next interval, waiting so
/// that it is before returning. It yields timing information when returning.
/// If this falls behind time for some reason, the ticks will be yielded with
/// the time information at when it was supposed to yield, until catching up.
pub struct ClockTimer {
	/// The time the next tick will trigger
	///
	/// In a newly created clock timer, this is the starting time
	next_tick: DateTime<Local>,

	/// How often this clock timer will yield ticks
	interval: TimeDelta,

	/// How much time has elapsed since the first tick
	///
	/// More precisely, this tracks how much time is between the first
	/// tick, and the next tick if there is one. Otherwise, the value in this
	/// field is meaningless.
	elapsed: TimeDelta,

	/// How much time is remaining
	///
	/// More precisely, this tracks how much time is remaining after the time in
	/// [`next_tick`](ClockTimer::next_tick)
	remaining: TimeDelta
}

/// Timing information for one tick
pub struct Tick {
	/// The time of this tick (or, if this tick was delayed, what time this tick
	/// was scheduled to be yielded at)
	this_tick: DateTime<Local>,

	/// The duration from the first tick to this tick (scheduled time),
	/// ie. the time the clock timer has been running
	elapsed: TimeDelta,

	/// The duration from this tick to the last tick (scheduled time),
	/// ie. the remaining time the clock timer will run before stopping
	remaining: TimeDelta,

	/// Whether or not this tick has been delayed
	///
	/// Note: We have not properly tested this (except in the
	/// [april fools prank](https://www.fimfiction.net/story/553695/) that this
	/// was built for of course heh), and we suspect this value is always `true`
	/// no matter if it was _actually_ delayed by the definition of what you'd
	/// expect. You might expect this to be `true` if previous task took too long
	/// or something, ie. this was called delayed because of the application
	/// logic itself, rather than little OS scheduling runtime things, ie. OS
	/// thread scheduling, tokio task scheduling, syncronisation stuff, etc etc.
	/// We expect this to always be `true`, because tokio will not wake up and
	/// poll again a task until the time has passed, and never before, and if
	/// there's any tiny bit of delay introduced anywhere detectable by the time
	/// APIs, be it from OS thread syncronisation, or tokio syncronisation, or
	/// the arithmetic and time putting the task away to sleep by the async
	/// runtime, or something, which, based on how these things work, this will
	/// likely always happen and make ths `true`.
	///
	/// ...whew ramble
	delayed: bool
}

impl ClockTimer {
	/// Gets a [`ClockTimer`] builder
	#[inline]
	pub fn builder() -> builder2::Builder {
		builder2::Builder::new()
	}

	/// Runs the next tick and returns timing information for it, if this
	/// interval is not finished already.
	#[inline]
	pub async fn tick(&mut self) -> Option<Tick> {
		if self.remaining < TimeDelta::zero() { return None }

		let mut tick = Tick {
			this_tick: self.next_tick,
			elapsed: self.elapsed,
			remaining: self.remaining,
			delayed: false
		};

		self.next_tick += self.interval;
		self.elapsed += self.interval;
		self.remaining -= self.interval;

		let delta = tick.this_tick - Local::now();

		// TODO: rethink delayed detection?
		// because it is highly likely that due to various factors out of our
		// control (eg. OS scheduling, tokio runtime scheduling, work stealing,
		// syncronisation stuff, etc etc), we won't get polled until technically
		// after our scheduled time, leading this to always be true? tests needed,
		// and this delay is in the order of milliseconds, or maybe even micros/nanos
		if delta <= TimeDelta::zero() {
			// highly unlikely, but if delta somehow manages to hit exactly 0,
			// we consider it on time. Maybe we should say like, if now is
			// within 1ms after the set tick time? dunno (see above todo comment)
			if delta < TimeDelta::zero() { tick.delayed = true }
			return Some(tick)
		}

		// we checked above and returned if `delta` is lte zero,
		// so this won't panic
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

impl Tick {
	/// Get time of this tick
	#[inline]
	pub fn time(&self) -> DateTime<Local> {
		self.this_tick
	}

	/// Get elapsed time since the start of this timer
	#[inline]
	pub fn elapsed(&self) -> TimeDelta {
		self.elapsed
	}

	/// Get remaining runtime of this timer
	#[inline]
	pub fn remaining(&self) -> TimeDelta {
		self.remaining
	}

	/// Get start time of this timer
	#[inline]
	pub fn start_time(&self) -> DateTime<Local> {
		self.this_tick - self.elapsed
	}

	/// Get end time of this timer
	#[inline]
	pub fn end_time(&self) -> DateTime<Local> {
		self.this_tick + self.remaining
	}

	/// Get total runtime of this timer, including elapsed
	/// time and remaining time
	#[inline]
	pub fn total_runtime(&self) -> TimeDelta {
		self.elapsed + self.remaining
	}

	/// Returns if this tick was delayed. This tick is considered delayed if
	/// the tick function was called after the time of this tick had already past.
	///
	/// Note: does the same thing as [`past_due`][Self::past_due]
	#[inline]
	pub fn delayed(&self) -> bool {
		self.delayed
	}

	/// Returns if this tick is past due. This tick is considered past due if
	/// the tick function was called after the time of this tick had already past.
	///
	/// Note: does the same thing as [`delayed`][Self::delayed]
	#[inline]
	pub fn past_due(&self) -> bool {
		self.delayed
	}
}

/// [`ClockTimer`] builder structs
pub mod builder {
	use super::*;

	/// Builder for [`ClockTimer`].
	pub struct Builder {
		/// Forcing users to use [`new`] because I dunno style or something, that
		/// [`new`] call and this struct is just literally gonna get optimised
		/// away to nothing
		///
		/// [`new`]: Builder::new
		__private: ()
	}

	impl Builder {
		/// New builder. You can also obtain a builder through [`ClockTimer::builder`]
		// there is no default that makes sense here
		#[expect(clippy::new_without_default, reason = "api design")]
		#[inline]
		pub fn new() -> Self {
			// its gonna optimise away to be noop lol
			// I think it provides a good API though,
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

	/// Intermediate builder state struct, returned after calling a method on
	/// [`Builder`]
	///
	/// Most likely you won't need to ever interact with this type directly.
	/// You're probably looking for [`Builder`].
	pub struct BuilderWithStart {
		/// The provided start datetime
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

	/// Intermediate builder state struct, returned after calling a method on
	/// [`BuilderWithStart`]
	///
	/// Most likely you won't need to ever interact with this type directly.
	/// You're probably looking for [`Builder`].
	pub struct BuilderWithEnd {
		/// The provided start datetime (from prev stage of builder)
		start: DateTime<Local>,

		/// The end datetime, either provided or calculated
		/// from a runtime duration
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

	/// Intermediate builder state struct, returned after calling a method on
	/// [`BuilderWithEnd`]
	///
	/// Most likely you won't need to ever interact with this type directly.
	/// You're probably looking for [`Builder`].
	pub struct BuilderWithInterval {
		/// The provided start datetime (from prev stage of builder)
		start: DateTime<Local>,

		/// The end datetime, either provided or calculated from a runtime
		/// duration (from prev stage of builder)
		end: DateTime<Local>,

		/// The provided trigger interval
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

pub mod builder2 {
	use super::*;
	use crate::builder::{ Init, Uninit };

	#[repr(transparent)]
	pub struct Builder<
		Start = Uninit,
		End = Uninit,
		Interval = Uninit
	> {
		inner: BuilderInner,
		__marker: PhantomData<(Start, End, Interval)>
	}

	struct BuilderInner {
		start: MaybeUninit<DateTime<Local>>,
		end: MaybeUninit<DateTime<Local>>,
		interval: MaybeUninit<TimeDelta>
	}

	impl Builder {
		#[expect(clippy::new_without_default, reason = "api design")]
		#[inline]
		pub fn new() -> Builder {
			Builder {
				inner: BuilderInner {
					start: MaybeUninit::uninit(),
					end: MaybeUninit::uninit(),
					interval: MaybeUninit::uninit()
				},
				__marker: PhantomData
			}
		}
	}

	impl<End, Interval> Builder<Uninit, End, Interval> {
		#[inline]
		pub fn with_start_datetime<TZ: TimeZone>(mut self, datetime: DateTime<TZ>) -> Builder<Init, End, Interval> {
			self.inner.start.write(datetime.with_timezone(&Local));
			Builder { inner: self.inner, __marker: PhantomData }
		}
	}

	impl<Start, Interval> Builder<Start, Uninit, Interval> {
		#[inline]
		pub fn with_end_datetime<TZ: TimeZone>(mut self, datetime: DateTime<TZ>) -> Builder<Start, Init, Interval> {
			self.inner.end.write(datetime.with_timezone(&Local));
			Builder { inner: self.inner, __marker: PhantomData }
		}
	}

	impl<Interval> Builder<Init, Uninit, Interval> {
		#[inline]
		pub fn with_duration(mut self, duration: TimeDelta) -> Builder<Init, Init, Interval> {
			// SAFETY: enforced by type system (typestate pattern)
			let start = unsafe { self.inner.start.assume_init() };

			self.inner.end.write(start + duration);
			Builder { inner: self.inner, __marker: PhantomData }
		}
	}

	impl<Start, End> Builder<Start, End, Uninit> {
		#[inline]
		pub fn with_interval(mut self, interval: TimeDelta) -> Builder<Start, End, Init> {
			self.inner.interval.write(interval);
			Builder { inner: self.inner, __marker: PhantomData }
		}
	}

	impl Builder<Init, Init, Init> {
		#[inline]
		pub fn build(self) -> ClockTimer {
			// SAFETY: enforced by type system (typestate pattern)
			let start = unsafe { self.inner.start.assume_init() };
			// SAFETY: enforced by type system (typestate pattern)
			let end = unsafe { self.inner.end.assume_init() };
			// SAFETY: enforced by type system (typestate pattern)
			let interval = unsafe { self.inner.interval.assume_init() };

			ClockTimer {
				next_tick: start,
				interval,
				elapsed: TimeDelta::zero(),
				remaining: end - start
			}
		}
	}
}
