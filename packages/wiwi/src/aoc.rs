use crate::prelude::*;
use crate::chain::GenericChainConversion as _;
use std::io::Read;

#[inline]
pub fn get_input(year: usize, day: usize) -> String {
	String::from_utf8(get_input_buf(year, day)).unwrap()
}

#[inline]
pub fn get_input_buf(year: usize, day: usize) -> Vec<u8> {
	let path = env::current_dir()
		.expect("failed to get current dir")
		.into_generic_chain()
		.with_inner(|p| p.push("input"))
		.with_inner(|p| p.push(&*format!("{year:04}-{day:02}")))
		.into_inner();

	let file = fs::OpenOptions::new()
		.read(true)
		.open(&*path)
		.unwrap_or_else(|e| panic!("failed to read input file (tried `{path:?}`): {e}"));

	(file, Vec::new())
		.into_generic_chain()
		.with_inner(|(f, v)| {
			f.read_to_end(v)
				.unwrap_or_else(|e| panic!("error occured reading input file at `{path:?}`: {e}"))
		})
		.map(|(_f, v)| v)
		.into_inner()
}

#[inline]
pub fn print_p1<T>(result: T)
where
	T: fmt::Display
{
	println!("part1: {result}")
}

#[inline]
pub fn print_p2<T>(result: T)
where
	T: fmt::Display
{
	println!("part2: {result}")
}

pub mod prelude {
	pub use crate::prelude::*;
	pub use crate::aoc::{ self, * };
	pub use crate::chain::{ self, * };
	pub use crate::num::*;
}
