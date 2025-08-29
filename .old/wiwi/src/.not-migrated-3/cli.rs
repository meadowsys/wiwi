extern crate hashbrown;

use hashbrown::HashMap;

pub struct Cli {
	next_cli_opt_id: usize,
	short_opts: HashMap<char, usize>,
	long_opts: HashMap<&'static str, usize>,
	value_storage: HashMap<usize, ErasedOption>
}

impl Cli {
	// TODO: return type
	#[inline]
	pub fn bool<F, FReturn>(&mut self, f: F)
	where
		F: FnOnce(&mut CliOption<bool>) -> FReturn
	{
		let mut opt = <CliOption<_> as Default>::default();
		f(&mut opt);

		let CliOption { short, long, default } = opt;
		let next_cli_opt_id = self.next_cli_opt_id;
		self.next_cli_opt_id += 1;

		if let Some(s) = short {
			self.short_opts.insert_unique_unchecked(s, next_cli_opt_id);
		}
		if let Some(l) = long {
			self.long_opts.insert_unique_unchecked(l, next_cli_opt_id);
		}
		if let Some(def) = default {
			self.value_storage.insert_unique_unchecked(next_cli_opt_id, ErasedOption::new(def));
		}
	}
}

#[derive(Default)]
pub struct CliOption<T> {
	short: Option<char>,
	long: Option<&'static str>,
	default: Option<T>
}

impl<T> CliOption<T> {
	#[inline]
	pub fn short(&mut self, opt: char) -> &mut Self {
		self.short = Some(opt);
		self
	}

	#[inline]
	pub fn s(&mut self, opt: char) -> &mut Self {
		self.short(opt)
	}

	#[inline]
	pub fn long(&mut self, opt: &'static str) -> &mut Self {
		self.long = Some(opt);
		self
	}

	#[inline]
	pub fn l(&mut self, opt: &'static str) -> &mut Self {
		self.long(opt)
	}

	#[inline]
	pub fn default(&mut self, val: T) -> &mut Self {
		self.default = Some(val);
		self
	}

	#[inline]
	pub fn def(&mut self, val: T) -> &mut Self {
		self.default(val)
	}

	#[inline]
	pub fn no_default(&mut self) -> &mut Self {
		self.default = None;
		self
	}

	#[inline]
	pub fn no_def(&mut self) -> &mut Self {
		self.no_default()
	}
}

pub struct ErasedOption {
	ptr: *const ()
}

impl ErasedOption {
	#[inline]
	pub fn new<T>(value: T) -> Self {
		let ptr = Box::into_raw(Box::new(value)).cast::<()>().cast_const();
		Self { ptr }
	}

	/// # Safety
	///
	/// This method must be called with the same type that this box was
	/// constructed with
	pub unsafe fn into_value<T>(self) -> T {
		// SAFETY: caller promises to call this with the correct type. If the type
		// is correct, the pointer will be valid to move out of it
		unsafe { *Box::from_raw(self.ptr.cast::<T>().cast_mut()) }
	}
}

impl Drop for ErasedOption {
	fn drop(&mut self) {
		// we cannot know the type insie, so we can't take it back out safely
		// so we kinda have to let it leak
		// and only panic in debug mode, cause a silent leak in production is better
		// than the app crashing and burning
		#[cfg(debug_assertions)] {
			panic!("erased option has leaked");
		}
	}
}
