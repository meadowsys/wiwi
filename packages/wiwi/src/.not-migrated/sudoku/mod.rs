use crate::chainer::{ ArrayChain, ChainHalf, NonChainHalf, SliceRefChain };
use crate::iter::*;
use crate::num_traits::*;
use std::mem::MaybeUninit;

pub struct Board {
	inner: [CellValue; 81]
}

/// Layout of the bitfield: `Gxxxxxx987654321`, where the digits 1..=9 are
/// marking values of 1 to 9, and G is marker for if this value is a given value
/// (part of the initial puzzle). If it is, only one of 1 to 9 may be marked,
/// which is the given value. If it isn't a given value, any combination may be
/// toggled on or off, to indicate possible values.
struct CellValue {
	/// bitfield
	bitfield: u16
}

impl Board {
	pub fn new(game: [[u8; 9]; 9]) -> Self {
		debug_assert_eq!({
			SliceRefChain::<_>::from(&game as &[_])
				.flatten()
				.into_nonchain()
				.len()
		}, 81);

		let mut board = MaybeUninit::<[CellValue; 81]>::uninit();
		let board_ptr = board.as_mut_ptr().cast::<CellValue>();

		for (r, arr) in game.into_iter().enumerate() {
			for (c, cell) in arr.into_iter().enumerate() {
				let offset = (r * 9) + c;

				if cell != 0 {
					// TODO: better error handling
					assert!(cell >= 1);
					assert!(cell <= 9);

					unsafe {
						board_ptr
							.add(offset)
							.write(CellValue::from_given_u8_unchecked(cell));
					}
				} else {
					unsafe {
						board_ptr
							.add(offset)
							.write(CellValue::new_conservatively_marked_nongiven());
					}
				}
			}
		}

		let inner = unsafe { board.assume_init() };
		Self { inner }
	}
}

impl CellValue {
	/// # Safety
	///
	/// The provided value must be within 1..=9
	#[inline]
	unsafe fn from_given_u8_unchecked(val: u8) -> Self {
		Self { bitfield: (1 << (val - 1)) | 0x8000 }
	}

	/// TODO: find a better name than "nongiven" to the values that aren't part
	/// of the initial puzzle lol
	#[inline]
	fn new_conservatively_marked_nongiven() -> Self {
		// last 9 bits filled
		Self { bitfield: 0b111111111 }
	}

	#[inline]
	fn new_empty_nongiven() -> Self {
		Self { bitfield: 0 }
	}

	#[inline]
	unsafe fn is_given(&self) -> bool {
		self.bitfield >> 15 == 1
	}

	/// # Safety
	///
	/// `self` must not be a given value, and `val` must be within 1..=9
	#[inline]
	unsafe fn mark_possible_unchecked(&mut self, val: u8) {
		self.bitfield |= 1 << (val - 1)
	}

	/// # Safety
	///
	/// `self` must not be a given value, and `val` must be within 1..=9
	#[inline]
	unsafe fn unmark_possible_unchecked(&mut self, val: u8) {
		self.bitfield &= !(1 << (val - 1))
	}

	/// # Safety
	///
	/// `self` must be a given value
	#[inline]
	unsafe fn value_of_given(&self) -> u8 {
		debug_assert!(
			self.is_given() && self.bitfield.count_ones() == 2,
			"a given value should only have G bit and one value bit set"
		);

		let mut value = 1;
		let mut acc = self.bitfield;

		loop {
			debug_assert!(value <= 9, "bitfield somehow unmarked (not given value?)");
			if acc & 1 == 1 { return value }

			acc >>= 1;
			value += 1;
		}
	}

	#[inline]
	unsafe fn contains_value(&self, val: u8) -> bool {
		(self.bitfield >> (val - 1)) & 1 != 0
	}

	#[inline]
	unsafe fn ungiven_possible_values_iter(&self) -> CellUngivenValuesIter {
		CellUngivenValuesIter::new(self.bitfield)
	}
}

struct CellUngivenValuesIter {
	value: u8,
	acc: u16
}

impl CellUngivenValuesIter {
	#[inline]
	fn new(bitfield: u16) -> Self {
		Self { value: 1, acc: bitfield }
	}
}

impl Iter for CellUngivenValuesIter {
	type Item = u8;
	fn next(&mut self) -> Option<u8> {
		while self.value <= 9 {
			let acc = self.acc;
			let value = self.value;

			self.value += 1;
			self.acc >>= 1;
			if acc & 1 == 1 { return Some(value) }
		}

		None
	}
}

/// Indexing with board offset (`0..81`) will yield offsets to the other tables.
/// 0 being row offsets, 1 being col offsets, 2 being group offsets
// const OFFSET_TABLE: OffsetTable = generate_offsets();
const OFFSET_TABLE: [[u8; 3]; 81] = generate_offsets();

const ROW_OFFSETS: [[u8; 9]; 9] = generate_rows_table();
const COL_OFFSETS: [[u8; 9]; 9] = generate_cols_table();
const GROUP_OFFSETS: [[u8; 9]; 9] = generate_groups_table();

const ALL_GROUP_OFFSETS: [[u8; 9]; 27] = get_all_offsets();

const fn generate_offsets() -> [[u8; 3]; 81] {
	let mut table = [[0u8; 3]; 81];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;

			table[offset][0] = row as _;
			table[offset][1] = col as _;

			let g_row = row / 3;
			let g_col = col / 3;
			let g_offset = (g_row * 3) + g_col;
			table[offset][2] = g_offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_rows_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;
			table[row][col] = offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_cols_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let mut row = 0usize;
	while row < 9 {
		let mut col = 0usize;
		while col < 9 {
			let offset = (row * 9) + col;
			table[col][row] = offset as _;

			col += 1;
		}

		row += 1;
	}

	table
}

const fn generate_groups_table() -> [[u8; 9]; 9] {
	let mut table = [[0u8; 9]; 9];

	let groups = [0u8..3, 3..6, 6..9];

	let mut row = 0usize;
	while row < 3 {
		let mut col = 0usize;
		while col < 3 {
			let root_offset = (row * 3) + col;

			let row_group = &groups[row];
			let col_group = &groups[col];

			let mut row = row_group.start;
			while row < row_group.end {
				let mut col = col_group.start;
				while col < col_group.end {
					let board_offset = (row * 9) + col;

					let r = row - row_group.start;
					let c = col - col_group.start;
					let offset = (r * 3) + c;
					table[root_offset][offset as usize] = board_offset;

					col += 1;
				}

				row += 1;
			}

			col += 1;
		}

		row += 1;
	}

	table
}

const fn get_all_offsets() -> [[u8; 9]; 27] {
	let mut all = [[0u8; 9]; 27];
	let mut all_i = 0;

	let mut i = 0;
	while i < 9 {
		all[all_i] = ROW_OFFSETS[i];

		i += 1;
		all_i += 1;
	}

	let mut i = 0;
	while i < 9 {
		all[all_i] = COL_OFFSETS[i];

		i += 1;
		all_i += 1;
	}

	let mut i = 0;
	while i < 9 {
		all[all_i] = GROUP_OFFSETS[i];

		i += 1;
		all_i += 1;
	}

	all
}

/// Encoding/decoding a sudoku board, without specifying if a cell is a given
/// value or not (ie. only stores solution, but not puzzle). This method for
/// storing can do so in 33 bytes per board (16 `u16`s follwed by 1 `u8`).
pub mod solution_encoding {
	use super::*;

	// TODO: there can be better impl when implementing it by hand
	#[derive(Debug)]
	pub struct Encoded {
		inner: [u8; 33]
	}

	impl Encoded {
		#[inline]
		pub const unsafe fn new_unchecked(array: [u8; 33]) -> Self {
			Self { inner: array }
		}

		#[inline]
		pub fn as_bytes(&self) -> &[u8] {
			&self.inner
		}
	}

	/// # Safety
	///
	/// All cells in `bytes` must have a value within `1..=9`.
	pub unsafe fn encode_byte_array_unchecked(bytes: &[u8; 81]) -> Encoded {
		let mut out = ArrayChain::new_uninit();

		let mut out_ptr = out.as_nonchain_mut().as_mut_ptr().cast::<u8>();
		let mut bytes_ptr = bytes.as_ptr();

		for _ in 0usize..16 {
			let mut current = 0u16;

			for _ in 0..5usize {
				// TODO: unchecked math?
				current *= 9;
				current += (*bytes_ptr - 1).into_u16();

				bytes_ptr = bytes_ptr.add(1);
			}

			let current = current.to_le_bytes();
			out_ptr.copy_from_nonoverlapping(current.as_ptr(), 2);

			// wrote that much bytes out
			out_ptr = out_ptr.add(2);
			// // just consumed that much of the board
			// bytes_ptr = bytes_ptr.add(5);
		}

		out_ptr.write(*bytes_ptr);

		Encoded { inner: out.assume_init().into_nonchain() }
	}

	pub unsafe fn decode_board_unchecked(board: &Encoded) -> [u8; 81] {
		let mut out = ArrayChain::new_uninit();

		let mut out_ptr = out.as_nonchain_mut().as_mut_ptr().cast::<u8>();
		let mut board_ptr = board.inner.as_ptr();

		for _ in 0usize..16 {
			let mut current = ArrayChain::new_uninit();
			board_ptr.copy_to_nonoverlapping(current.as_nonchain_mut().as_mut_ptr().cast::<u8>(), 2);
			let mut current = u16::from_le_bytes(current.assume_init().into_nonchain());
			board_ptr = board_ptr.add(2);

			out_ptr = out_ptr.add(5);
			for _ in 0..5usize {
				out_ptr = out_ptr.sub(1);
				out_ptr.write(((current % 9) + 1).into_u8_lossy());

				current /= 9;
			}
			out_ptr = out_ptr.add(5);
		}

		out_ptr.write(*board_ptr);

		out.assume_init().into_nonchain()
	}

	#[inline]
	pub const fn encoded_all_ones() -> Encoded {
		let mut inner = [0u8; 33];
		inner[32] = 1;
		Encoded { inner }
	}

	#[inline]
	pub fn encode_byte_array_checked(board: &[u8; 81]) -> Option<Encoded> {
		for cell in *board {
			// I cannot remember why I used manual compare over range compare, lol
			#[allow(clippy::manual_range_contains)]
			if cell < 1 || cell > 9 { return None }
		}

		Some(unsafe { encode_byte_array_unchecked(board) })
	}

	pub unsafe fn is_valid_sudoku_board(board: &[u8; 81]) -> bool {
		let board_ptr = board.as_ptr();

		for group in ALL_GROUP_OFFSETS {
			let mut accumulator = CellValue::new_empty_nongiven();

			for offset in group {
				let val = unsafe { *board_ptr.add(offset.into_usize()) };
				if accumulator.contains_value(val) { return false }
				accumulator.mark_possible_unchecked(val);
			}
		}

		true
	}

	#[inline]
	pub unsafe fn get_next_valid(board: &mut [u8; 81]) -> Option<Encoded> {
		loop {
			let valid = is_valid_sudoku_board(board);

			// "increment" board
			let incremented = increment_board(board);

			// return if this one was success
			// else we'll let it loop around again
			if valid { return Some(encode_byte_array_unchecked(board)) }

			// if it didn't increment a cell without overflowing, we reached "highest"
			// board combo, return None
			// if caller calls this again with the same (mutating) board, it's just
			// gonna loop again; it is up to the caller to stop now
			if !incremented { return None }
		}
	}

	fn increment_board(board: &mut [u8; 81]) -> bool {
		for cell in board.iter_mut().rev() {
			match *cell + 1 {
				new @ 2..=9 => {
					*cell = new;
					return true
				}
				// let it "wrap" around (let it loop again)
				10 => { *cell = 1 }
				cell => { unreachable!("invalid cell: {cell}") }
			}
		}

		false
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use rand::{ Rng, thread_rng };
		use rand::distributions::{ Distribution, Uniform };

		#[test]
		fn roundtrip_board() {
			let mut rng = thread_rng();
			let dist = Uniform::from(1..=9);

			for _ in 0..1000 {
				let mut board = [0u8; 81];
				for cell in &mut board {
					*cell = dist.sample(&mut rng);
				}

				unsafe {
					assert_eq!(decode_board_unchecked(&encode_byte_array_unchecked(&board)), board);
					// assert_eq!((&encode_byte_array_unchecked(&board).inner as &[u8]), &board as &[u8]);
				}
			}
		}
	}
}

/// Encoding/decoding a sudoku board, as well as if a cell is a given value or
/// not (ie. if it is part of the blank puzzle). This method for storing can do so
/// in 44 bytes per board (5 `u64`'s followed by 1 `u32`).
pub mod puzzle_encoding {}
