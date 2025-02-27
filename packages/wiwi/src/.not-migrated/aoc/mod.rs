use std::{ env, fs, slice, str };
use std::io::{ BufReader, Read as _ };
use crate::iter::Iter;

pub struct InputStruct {
	file: String,
	consumed: usize
}

impl InputStruct {
	pub fn new(year: usize, day: usize) -> Self {
		let mut assumed_input_dir = env::current_dir()
			.expect("failed to get current dir");
		assumed_input_dir.push("input");
		assumed_input_dir.push(&*format!("{year:04}-d{day:02}.txt"));

		let mut file = fs::OpenOptions::new()
			.read(true)
			.open(&*assumed_input_dir)
			.unwrap_or_else(|e| panic!("try to read input file for year {year:04} day {day:02} at {assumed_input_dir:?}, not found: {e:?}"));

		let mut s = String::new();
		file.read_to_string(&mut s).expect("an error occured reading input file");
		Self { file: s, consumed: 0 }
	}

	pub fn iter_chars(&mut self) -> IterChars {
		IterChars { inner: self }
	}

	pub fn reset(&mut self) {
		self.consumed = 0
	}
}

pub struct IterChars<'h> {
	inner: &'h mut InputStruct
}

impl<'h> Iter for IterChars<'h> {
	type Item = char;
	fn next(&mut self) -> Option<char> {
		let c = self.inner.file[self.inner.consumed..].chars().next()?;
		self.inner.consumed += c.len_utf8();
		Some(c)
	}
}
