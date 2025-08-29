use crate::prelude::*;

#[derive(Clone)]
pub struct CronSchedule {
	minute: Minute,
	hour: Hour,
	day_of_month: DayOfMonth,
	month: Month,
	day_of_week: DayOfWeek
}

impl CronSchedule {
	/// Creates a blank cron schedule (equivalent to `* * * * *`)
	#[inline]
	pub const fn new() -> Self {
		Self {
			minute: Minute::new(),
			hour: Hour::new(),
			day_of_month: DayOfMonth::new(),
			month: Month::new(),
			day_of_week: DayOfWeek::new()
		}
	}

	#[inline]
	pub const fn add_minute(&mut self, minute: u64) {
		self.minute.add(minute);
	}

	#[inline]
	pub const fn add_minute_range(&mut self, e1: u64, e2: u64) {
		self.minute.add_range(e1, e2);
	}

	#[inline]
	pub const fn add_minute_step(&mut self, step: u64) {
		self.minute.add_step(step);
	}

	#[inline]
	pub const fn add_minute_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		self.minute.add_range_step(e1, e2, step);
	}

	#[inline]
	pub const fn add_hour(&mut self, hour: u64) {
		self.hour.add(hour);
	}

	#[inline]
	pub const fn add_hour_range(&mut self, e1: u64, e2: u64) {
		self.hour.add_range(e1, e2);
	}

	#[inline]
	pub const fn add_hour_step(&mut self, step: u64) {
		self.hour.add_step(step);
	}

	#[inline]
	pub const fn add_hour_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		self.hour.add_range_step(e1, e2, step);
	}

	#[inline]
	pub const fn add_day_of_month(&mut self, day_of_month: u64) {
		self.day_of_month.add(day_of_month);
	}

	#[inline]
	pub const fn add_day_of_month_range(&mut self, e1: u64, e2: u64) {
		self.day_of_month.add_range(e1, e2);
	}

	#[inline]
	pub const fn add_day_of_month_step(&mut self, step: u64) {
		self.day_of_month.add_step(step);
	}

	#[inline]
	pub const fn add_day_of_month_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		self.day_of_month.add_range_step(e1, e2, step);
	}

	#[inline]
	pub const fn add_month(&mut self, month: u64) {
		self.month.add(month);
	}

	#[inline]
	pub const fn add_month_range(&mut self, e1: u64, e2: u64) {
		self.month.add_range(e1, e2);
	}

	#[inline]
	pub const fn add_month_step(&mut self, step: u64) {
		self.month.add_step(step);
	}

	#[inline]
	pub const fn add_month_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		self.month.add_range_step(e1, e2, step);
	}

	#[inline]
	pub const fn add_day_of_week(&mut self, day_of_week: u64) {
		self.day_of_week.add(day_of_week);
	}

	#[inline]
	pub const fn add_day_of_week_range(&mut self, e1: u64, e2: u64) {
		self.day_of_week.add_range(e1, e2);
	}

	#[inline]
	pub const fn add_day_of_week_step(&mut self, step: u64) {
		self.day_of_week.add_step(step);
	}

	#[inline]
	pub const fn add_day_of_week_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		self.day_of_week.add_range_step(e1, e2, step);
	}

	#[inline]
	pub const fn preset_every_minute() -> Self {
		Self::new()
	}

	#[inline]
	pub const fn preset_every_1_minute() -> Self {
		Self::new()
	}

	#[inline]
	pub const fn preset_every_2_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_even_minute() -> Self {
		let mut c = Self::new();
		c.add_minute_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_uneven_minute() -> Self {
		let mut c = Self::new();
		c.add_minute_range_step(1, 59, 2);
		c
	}

	#[inline]
	pub const fn preset_every_3_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(3);
		c
	}

	#[inline]
	pub const fn preset_every_4_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(4);
		c
	}

	#[inline]
	pub const fn preset_every_5_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(5);
		c
	}

	#[inline]
	pub const fn preset_every_five_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(5);
		c
	}

	#[inline]
	pub const fn preset_every_6_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(6);
		c
	}

	#[inline]
	pub const fn preset_every_10_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(10);
		c
	}

	#[inline]
	pub const fn preset_every_15_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(15);
		c
	}

	#[inline]
	pub const fn preset_every_fifteen_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(15);
		c
	}

	#[inline]
	pub const fn preset_every_ten_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(10);
		c
	}

	#[inline]
	pub const fn preset_every_quarter_hour() -> Self {
		let mut c = Self::new();
		c.add_minute_step(15);
		c
	}

	#[inline]
	pub const fn preset_every_20_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(20);
		c
	}

	#[inline]
	pub const fn preset_every_30_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute_step(30);
		c
	}

	#[inline]
	pub const fn preset_every_hour_at_30_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute(30);
		c
	}

	#[inline]
	pub const fn preset_every_half_hour() -> Self {
		let mut c = Self::new();
		c.add_minute_step(30);
		c
	}

	#[inline]
	pub const fn preset_every_60_minutes() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c
	}

	#[inline]
	pub const fn preset_every_hour() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c
	}

	#[inline]
	pub const fn preset_every_1_hour() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c
	}

	#[inline]
	pub const fn preset_every_2_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_two_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_even_hour() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_other_hour() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_3_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(3);
		c
	}

	#[inline]
	pub const fn preset_every_three_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(3);
		c
	}

	#[inline]
	pub const fn preset_every_4_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(4);
		c
	}

	#[inline]
	pub const fn preset_every_6_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(6);
		c
	}

	#[inline]
	pub const fn preset_every_six_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(6);
		c
	}

	#[inline]
	pub const fn preset_every_8_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(8);
		c
	}

	#[inline]
	pub const fn preset_every_12_hours() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_step(12);
		c
	}

	#[inline]
	pub const fn preset_hour_range(start: u64, end: u64) -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_range(start, end);
		c
	}

	#[inline]
	pub const fn preset_between_certain_hours(start: u64, end: u64) -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour_range(start, end);
		c
	}

	#[inline]
	pub const fn preset_every_day() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_daily() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_once_a_day() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_every_night() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_every_day_at_0100() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(1);
		c
	}

	#[inline]
	pub const fn preset_every_day_at_0200() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(2);
		c
	}

	#[inline]
	pub const fn preset_every_day_at_0800() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(8);
		c
	}

	#[inline]
	pub const fn preset_every_morning() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(9);
		c
	}

	#[inline]
	pub const fn preset_every_midnight() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_every_day_at_midnight() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_every_night_at_midnight() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c
	}

	#[inline]
	pub const fn preset_every_sunday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_every_monday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::MON);
		c
	}

	#[inline]
	pub const fn preset_every_tuesday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::TUE);
		c
	}

	#[inline]
	pub const fn preset_every_wednesday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::WED);
		c
	}

	#[inline]
	pub const fn preset_every_thursday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::THU);
		c
	}

	#[inline]
	pub const fn preset_every_friday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::FRI);
		c
	}

	#[inline]
	pub const fn preset_every_friday_at_midnight() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::FRI);
		c
	}

	#[inline]
	pub const fn preset_every_saturday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SAT);
		c
	}

	#[inline]
	pub const fn preset_every_weekday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week_range(DayOfWeek::MON, DayOfWeek::FRI);
		c
	}

	#[inline]
	pub const fn preset_weekdays_only() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week_range(DayOfWeek::MON, DayOfWeek::FRI);
		c
	}

	#[inline]
	pub const fn preset_monday_to_friday() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week_range(DayOfWeek::MON, DayOfWeek::FRI);
		c
	}

	#[inline]
	pub const fn preset_every_weekend() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SAT);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_weekends_only() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SAT);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_every_7_days() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_every_week() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_weekly() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_once_a_week() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_week(DayOfWeek::SUN);
		c
	}

	#[inline]
	pub const fn preset_every_month() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c
	}

	#[inline]
	pub const fn preset_monthly() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c
	}

	#[inline]
	pub const fn preset_once_a_month() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c
	}

	#[inline]
	pub const fn preset_every_other_month() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c.add_month_step(2);
		c
	}

	#[inline]
	pub const fn preset_every_quarter() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c.add_month_step(3);
		c
	}

	#[inline]
	pub const fn preset_every_6_months() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c.add_month_step(6);
		c
	}

	#[inline]
	pub const fn preset_every_year() -> Self {
		let mut c = Self::new();
		c.add_minute(0);
		c.add_hour(0);
		c.add_day_of_month(1);
		c.add_month(1);
		c
	}
}

impl Default for CronSchedule {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Clone, Copy)]
pub struct Minute {
	inner: u64
}

impl Minute {
	pub const MIN: u64 = 0;
	pub const MAX: u64 = 59;

	#[inline]
	const fn new() -> Self {
		Self { inner: 0 }
	}

	#[inline]
	const fn check_range(minute: u64) -> bool {
		minute <= 59
	}

	#[inline]
	const fn add(&mut self, minute: u64) {
		if !self.add_checked(minute) {
			panic!("provided `minute` out of range")
		}
	}

	#[inline]
	const fn add_checked(&mut self, minute: u64) -> bool {
		if !Self::check_range(minute) { return false }

		// SAFETY: just checked `minute` is in range
		unsafe { self.add_unchecked(minute) }
		true
	}

	#[inline]
	const unsafe fn add_unchecked(&mut self, minute: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(minute)) }

		self.inner |= 1 << minute;
	}

	#[inline]
	const fn add_range(&mut self, e1: u64, e2: u64) {
		if !self.add_range_checked(e1, e2) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_checked(&mut self, e1: u64, e2: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_unchecked(e1, e2) }
		true
	}

	#[inline]
	const unsafe fn add_range_unchecked(&mut self, e1: u64, e2: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { self.add_range_step_unchecked(e1, e2, 1) }
	}

	#[inline]
	const fn add_step(&mut self, step: u64) {
		// SAFETY: safety precondition upheld by caller
		// `MIN` and `MAX` are within range too
		unsafe { self.add_range_step_unchecked(Self::MIN, Self::MAX, step) }
	}

	#[inline]
	const fn add_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		if !self.add_range_step_checked(e1, e2, step) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_step_checked(&mut self, e1: u64, e2: u64, step: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_step_unchecked(e1, e2, step) }
		true
	}

	#[inline]
	const unsafe fn add_range_step_unchecked(&mut self, mut e1: u64, e2: u64, step: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(e1)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(Self::check_range(e2)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(e1 <= e2) }

		while e1 <= e2 {
			// SAFETY: safety precondition upheld by caller
			// and checked by if statement
			unsafe { self.add_unchecked(e1) }

			e1 += step;
		}
	}

	// #[inline]
	// const fn union(&mut self, other: Self) {
	// 	self.inner |= other.inner;
	// }

	// #[inline]
	// const fn intersection(&mut self, other: Self) {
	// 	self.inner &= other.inner;
	// }
}

#[derive(Clone, Copy)]
pub struct Hour {
	inner: u32
}

impl Hour {
	pub const MIN: u64 = 0;
	pub const MAX: u64 = 23;

	#[inline]
	const fn new() -> Self {
		Self { inner: 0 }
	}

	#[inline]
	const fn check_range(hour: u64) -> bool {
		hour <= 23
	}

	#[inline]
	const fn add(&mut self, hour: u64) {
		if !self.add_checked(hour) {
			panic!("provided `hour` out of range")
		}
	}

	#[inline]
	const fn add_checked(&mut self, hour: u64) -> bool {
		if !Self::check_range(hour) { return false }

		// SAFETY: just checked `hour` is in range
		unsafe { self.add_unchecked(hour) }
		true
	}

	#[inline]
	const unsafe fn add_unchecked(&mut self, hour: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(hour)) }

		self.inner |= 1 << hour;
	}

	#[inline]
	const fn add_range(&mut self, e1: u64, e2: u64) {
		if !self.add_range_checked(e1, e2) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_checked(&mut self, e1: u64, e2: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_unchecked(e1, e2) }
		true
	}

	#[inline]
	const unsafe fn add_range_unchecked(&mut self, e1: u64, e2: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { self.add_range_step_unchecked(e1, e2, 1) }
	}

	#[inline]
	const fn add_step(&mut self, step: u64) {
		// SAFETY: safety precondition upheld by caller
		// `MIN` and `MAX` are within range too
		unsafe { self.add_range_step_unchecked(Self::MIN, Self::MAX, step) }
	}

	#[inline]
	const fn add_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		if !self.add_range_step_checked(e1, e2, step) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_step_checked(&mut self, e1: u64, e2: u64, step: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_step_unchecked(e1, e2, step) }
		true
	}

	#[inline]
	const unsafe fn add_range_step_unchecked(&mut self, mut e1: u64, e2: u64, step: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(e1)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(Self::check_range(e2)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(e1 <= e2) }

		while e1 <= e2 {
			// SAFETY: safety precondition upheld by caller
			// and checked by if statement
			unsafe { self.add_unchecked(e1) }

			e1 += step;
		}
	}

	// #[inline]
	// const fn union(&mut self, other: Self) {
	// 	self.inner |= other.inner;
	// }

	// #[inline]
	// const fn intersection(&mut self, other: Self) {
	// 	self.inner &= other.inner;
	// }
}

#[derive(Clone, Copy)]
pub struct DayOfMonth {
	inner: u32
}

impl DayOfMonth {
	pub const MIN: u64 = 1;
	pub const MAX: u64 = 31;

	#[inline]
	const fn new() -> Self {
		Self { inner: 0 }
	}

	#[inline]
	const fn check_range(day_of_month: u64) -> bool {
		day_of_month >= 1 && day_of_month <= 31
	}

	#[inline]
	const fn add(&mut self, day_of_month: u64) {
		if !self.add_checked(day_of_month) {
			panic!("provided `day_of_month` out of range")
		}
	}

	#[inline]
	const fn add_checked(&mut self, day_of_month: u64) -> bool {
		if !Self::check_range(day_of_month) { return false }

		// SAFETY: just checked `day_of_month` is in range
		unsafe { self.add_unchecked(day_of_month) }
		true
	}

	#[inline]
	const unsafe fn add_unchecked(&mut self, day_of_month: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(day_of_month)) }

		self.inner |= 1 << (day_of_month - 1);
	}

	#[inline]
	const fn add_range(&mut self, e1: u64, e2: u64) {
		if !self.add_range_checked(e1, e2) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_checked(&mut self, e1: u64, e2: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_unchecked(e1, e2) }
		true
	}

	#[inline]
	const unsafe fn add_range_unchecked(&mut self, e1: u64, e2: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { self.add_range_step_unchecked(e1, e2, 1) }
	}

	#[inline]
	const fn add_step(&mut self, step: u64) {
		// SAFETY: safety precondition upheld by caller
		// `MIN` and `MAX` are within range too
		unsafe { self.add_range_step_unchecked(Self::MIN, Self::MAX, step) }
	}

	#[inline]
	const fn add_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		if !self.add_range_step_checked(e1, e2, step) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_step_checked(&mut self, e1: u64, e2: u64, step: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_step_unchecked(e1, e2, step) }
		true
	}

	#[inline]
	const unsafe fn add_range_step_unchecked(&mut self, mut e1: u64, e2: u64, step: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(e1)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(Self::check_range(e2)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(e1 <= e2) }

		while e1 <= e2 {
			// SAFETY: safety precondition upheld by caller
			// and checked by if statement
			unsafe { self.add_unchecked(e1) }

			e1 += step;
		}
	}

	// #[inline]
	// const fn union(&mut self, other: Self) {
	// 	self.inner |= other.inner;
	// }

	// #[inline]
	// const fn intersection(&mut self, other: Self) {
	// 	self.inner &= other.inner;
	// }
}

#[derive(Clone, Copy)]
pub struct Month {
	inner: u16
}

impl Month {
	pub const MIN: u64 = 1;
	pub const MAX: u64 = 12;

	pub const JANUARY: u64 = 1;
	pub const FEBRUARY: u64 = 2;
	pub const MARCH: u64 = 3;
	pub const APRIL: u64 = 4;
	pub const MAY: u64 = 5;
	pub const JUNE: u64 = 6;
	pub const JULY: u64 = 7;
	pub const AUGUST: u64 = 8;
	pub const SEPTEMBER: u64 = 9;
	pub const OCTOBER: u64 = 10;
	pub const NOVEMBER: u64 = 11;
	pub const DECEMBER: u64 = 12;

	pub const JAN: u64 = Self::JANUARY;
	pub const FEB: u64 = Self::FEBRUARY;
	pub const MAR: u64 = Self::MARCH;
	pub const APR: u64 = Self::APRIL;
	// le may is already 3 letter
	pub const JUN: u64 = Self::JUNE;
	pub const JUL: u64 = Self::JULY;
	pub const AUG: u64 = Self::AUGUST;
	pub const SEP: u64 = Self::SEPTEMBER;
	pub const OCT: u64 = Self::OCTOBER;
	pub const NOV: u64 = Self::NOVEMBER;
	pub const DEC: u64 = Self::DECEMBER;

	#[inline]
	const fn new() -> Self {
		Self { inner: 0 }
	}

	#[inline]
	const fn check_range(month: u64) -> bool {
		month >= 1 && month <= 12
	}

	#[inline]
	const fn add(&mut self, month: u64) {
		if !self.add_checked(month) {
			panic!("provided `month` out of range")
		}
	}

	#[inline]
	const fn add_checked(&mut self, month: u64) -> bool {
		if !Self::check_range(month) { return false }

		// SAFETY: just checked `month` is in range
		unsafe { self.add_unchecked(month) }
		true
	}

	#[inline]
	const unsafe fn add_unchecked(&mut self, month: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(month)) }

		self.inner |= 1 << (month - 1);
	}

	#[inline]
	const fn add_range(&mut self, e1: u64, e2: u64) {
		if !self.add_range_checked(e1, e2) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_checked(&mut self, e1: u64, e2: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_unchecked(e1, e2) }
		true
	}

	#[inline]
	const unsafe fn add_range_unchecked(&mut self, e1: u64, e2: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { self.add_range_step_unchecked(e1, e2, 1) }
	}

	#[inline]
	const fn add_step(&mut self, step: u64) {
		// SAFETY: safety precondition upheld by caller
		// `MIN` and `MAX` are within range too
		unsafe { self.add_range_step_unchecked(Self::MIN, Self::MAX, step) }
	}

	#[inline]
	const fn add_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		if !self.add_range_step_checked(e1, e2, step) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_step_checked(&mut self, e1: u64, e2: u64, step: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_step_unchecked(e1, e2, step) }
		true
	}

	#[inline]
	const unsafe fn add_range_step_unchecked(&mut self, mut e1: u64, e2: u64, step: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(e1)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(Self::check_range(e2)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(e1 <= e2) }

		while e1 <= e2 {
			// SAFETY: safety precondition upheld by caller
			// and checked by if statement
			unsafe { self.add_unchecked(e1) }

			e1 += step;
		}
	}

	// #[inline]
	// const fn union(&mut self, other: Self) {
	// 	self.inner |= other.inner;
	// }

	// #[inline]
	// const fn intersection(&mut self, other: Self) {
	// 	self.inner &= other.inner;
	// }
}

#[derive(Clone, Copy)]
pub struct DayOfWeek {
	inner: u8
}

impl DayOfWeek {
	pub const MIN: u64 = 0;
	pub const MAX: u64 = 6;

	pub const SUNDAY: u64 = 0;
	pub const MONDAY: u64 = 1;
	pub const TUESDAY: u64 = 2;
	pub const WEDNESDAY: u64 = 3;
	pub const THURSDAY: u64 = 4;
	pub const FRIDAY: u64 = 5;
	pub const SATURDAY: u64 = 6;

	pub const SUN: u64 = Self::SUNDAY;
	pub const MON: u64 = Self::MONDAY;
	pub const TUE: u64 = Self::TUESDAY;
	pub const WED: u64 = Self::WEDNESDAY;
	pub const THU: u64 = Self::THURSDAY;
	pub const FRI: u64 = Self::FRIDAY;
	pub const SAT: u64 = Self::SATURDAY;

	#[inline]
	const fn new() -> Self {
		Self { inner: 0 }
	}

	#[inline]
	const fn check_range(day_of_week: u64) -> bool {
		day_of_week <= 6
	}

	#[inline]
	const fn add(&mut self, day_of_week: u64) {
		if !self.add_checked(day_of_week) {
			panic!("provided `day_of_week` out of range")
		}
	}

	#[inline]
	const fn add_checked(&mut self, day_of_week: u64) -> bool {
		if !Self::check_range(day_of_week) { return false }

		// SAFETY: just checked `day_of_week` is in range
		unsafe { self.add_unchecked(day_of_week) }
		true
	}

	#[inline]
	const unsafe fn add_unchecked(&mut self, day_of_week: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(day_of_week)) }

		self.inner |= 1 << day_of_week;
	}

	#[inline]
	const fn add_range(&mut self, e1: u64, e2: u64) {
		if !self.add_range_checked(e1, e2) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_checked(&mut self, e1: u64, e2: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_unchecked(e1, e2) }
		true
	}

	#[inline]
	const unsafe fn add_range_unchecked(&mut self, e1: u64, e2: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { self.add_range_step_unchecked(e1, e2, 1) }
	}

	#[inline]
	const fn add_step(&mut self, step: u64) {
		// SAFETY: safety precondition upheld by caller
		// `MIN` and `MAX` are within range too
		unsafe { self.add_range_step_unchecked(Self::MIN, Self::MAX, step) }
	}

	#[inline]
	const fn add_range_step(&mut self, e1: u64, e2: u64, step: u64) {
		if !self.add_range_step_checked(e1, e2, step) {
			panic!("provided `e1` and/or `e2` out of range")
		}
	}

	#[inline]
	const fn add_range_step_checked(&mut self, e1: u64, e2: u64, step: u64) -> bool {
		if !Self::check_range(e1) { return false }
		if !Self::check_range(e2) { return false }

		// SAFETY: just checked `e1` and `e2` are in range
		unsafe { self.add_range_step_unchecked(e1, e2, step) }
		true
	}

	#[inline]
	const unsafe fn add_range_step_unchecked(&mut self, mut e1: u64, e2: u64, step: u64) {
		// SAFETY: safety precondition upheld by caller
		unsafe { hint::assert_unchecked(Self::check_range(e1)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(Self::check_range(e2)) }
		// SAFETY: same as above
		unsafe { hint::assert_unchecked(e1 <= e2) }

		while e1 <= e2 {
			// SAFETY: safety precondition upheld by caller
			// and checked by if statement
			unsafe { self.add_unchecked(e1) }

			e1 += step;
		}
	}

	// #[inline]
	// const fn union(&mut self, other: Self) {
	// 	self.inner |= other.inner;
	// }

	// #[inline]
	// const fn intersection(&mut self, other: Self) {
	// 	self.inner &= other.inner;
	// }
}
