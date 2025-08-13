#![allow(
	clippy::missing_inline_in_public_items,
	reason = "wip"
)]

use hashbrown::HashSet;
use core::mem;

pub use self::coords::Coords;

mod coords;

pub struct GameOfLife<C: Coords> {
	/// current alive cells
	cells: HashSet<C, C::BuildHasher>,
	/// working space building the next set of active cells
	///
	/// this exists to avoid constantly reallocating/deallocating the set memory
	/// every iteration, which could slow down the simulation
	cells_next: HashSet<C, C::BuildHasher>,
	/// set that tracks cells that have already been ticked this iteration, to
	/// avoid doing them multiple times
	cells_have_been_ticked: HashSet<C, C::BuildHasher>,
	neighbouring_cells_queue: HashSet<C, C::BuildHasher>
}

// todo I'm curious about size
// type Test = GameOfLife<i32>;

impl<C: Coords> GameOfLife<C>
where
	C::BuildHasher: Default
{
	#[expect(clippy::new_without_default, reason = "api design")]
	pub fn new() -> Self {
		GameOfLife {
			cells: HashSet::with_hasher(C::BuildHasher::default()),
			cells_next: HashSet::with_hasher(C::BuildHasher::default()),
			cells_have_been_ticked: HashSet::with_hasher(C::BuildHasher::default()),
			neighbouring_cells_queue: HashSet::with_hasher(C::BuildHasher::default())
		}
	}
}
impl<C: Coords> GameOfLife<C> {
	pub fn set(&mut self, coord: C) {
		self.cells.insert(coord);
	}
	pub fn unset(&mut self, coord: C) {
		self.cells.remove(&coord);
	}

	// fn check_and_tick_cell(&mut self, cell: C, is_alive: bool) -> bool {
	// }

	// todo basically this function is in charge of checking before ticking now so it no longer needs to get is_alive cause it gets it itself
	// idk if I want this factored out like it is right now
	// but something that just ticks a singular cell like this I think is fine to get factored out since its a lot of repetitive code
	// but the rest is probably fine to stay in the main `step` function

	#[inline]
	fn get_neighbouring_cells(cell: C) -> NeighbouringCells<C> {
		NeighbouringCells {
			up: cell.coord_up(),
			down: cell.coord_down(),
			left: cell.coord_left(),
			right: cell.coord_right(),

			upleft: cell.coord_upleft(),
			upright: cell.coord_upright(),
			downleft: cell.coord_downleft(),
			downright: cell.coord_downright(),
		}
	}

	#[inline]
	fn get_neighbouring_cell_values(&self, cell: C) -> NeighbouringCellValues {
		let NeighbouringCells { up, down, left, right, upleft, upright, downleft, downright } = Self::get_neighbouring_cells(cell);
		NeighbouringCellValues {
			up: self.cells.contains(&up),
			down: self.cells.contains(&down),
			left: self.cells.contains(&left),
			right: self.cells.contains(&right),
			upleft: self.cells.contains(&upleft),
			upright: self.cells.contains(&upright),
			downleft: self.cells.contains(&downleft),
			downright: self.cells.contains(&downright)
		}
	}

	#[expect(clippy::as_conversions, reason = "numerical cast")]
	#[inline]
	fn get_neighbouring_cell_values_usize(&self, cell: C) -> NeighbouringCellValuesUsize {
		let NeighbouringCellValues { up, down, left, right, upleft, upright, downleft, downright } = self.get_neighbouring_cell_values(cell);
		NeighbouringCellValuesUsize {
			up: up as _,
			down: down as _,
			left: left as _,
			right: right as _,
			upleft: upleft as _,
			upright: upright as _,
			downleft: downleft as _,
			downright: downright as _
		}
	}

	#[inline]
	fn get_neighbour_alive_cell_count(&self, cell: C) -> usize {
		let NeighbouringCellValuesUsize { up, down, left, right, upleft, upright, downleft, downright } = self.get_neighbouring_cell_values_usize(cell);
		up + down + left + right + upleft + upright + downleft + downright
	}

	fn tick_alive_cell(&self, cell: C) -> bool {
		matches!(self.get_neighbour_alive_cell_count(cell), 2 | 3)
	}

	fn tick_dead_cell(&self, cell: C) -> bool {
		self.get_neighbour_alive_cell_count(cell) == 3
	}

	pub fn step(&mut self) {
		self.cells_next.clear();
		self.cells_have_been_ticked.clear();
		self.neighbouring_cells_queue.clear();

		for cell in self.cells.iter().copied() {
			self.cells_have_been_ticked.insert(cell);
			if self.tick_alive_cell(cell) {
				self.cells_next.insert(cell);
			}

			let NeighbouringCells { up, down, left, right, upleft, upright, downleft, downright } = Self::get_neighbouring_cells(cell);
			[up, down, left, right, upleft, upright, downleft, downright]
				.into_iter()
				.for_each(|cell| {
					self.neighbouring_cells_queue.insert(cell);
				});
		}

		for cell in self.neighbouring_cells_queue.iter().copied() {
			// - check if cell has been ticked, if not, tick them
			if !self.cells_have_been_ticked.contains(&cell) && self.tick_dead_cell(cell) {
				self.cells_next.insert(cell);
			}
		}

		mem::swap(&mut self.cells, &mut self.cells_next);
	}
}

struct NeighbouringCells<C> {
	up: C,
	down: C,
	left: C,
	right: C,
	upleft: C,
	upright: C,
	downleft: C,
	downright: C
}

struct NeighbouringCellValues {
	up: bool,
	down: bool,
	left: bool,
	right: bool,
	upleft: bool,
	upright: bool,
	downleft: bool,
	downright: bool
}

struct NeighbouringCellValuesUsize {
	up: usize,
	down: usize,
	left: usize,
	right: usize,
	upleft: usize,
	upright: usize,
	downleft: usize,
	downright: usize
}
