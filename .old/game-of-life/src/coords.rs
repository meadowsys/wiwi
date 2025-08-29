use core::hash::{ BuildHasher, Hash, Hasher };

/// use a coordinate system where y is vertical with positive up, x is
/// horizontal with positive right, and all functions that return new coordinates should use
/// saturating operations to assume never go out of bounds
pub trait Coords: Copy + Eq + Hash {
	type BuildHasher: BuildHasher;

	fn coord_up(self) -> Self;
	fn coord_down(self) -> Self;
	fn coord_left(self) -> Self;
	fn coord_right(self) -> Self;

	fn coord_upleft(self) -> Self;
	fn coord_upright(self) -> Self;
	fn coord_downleft(self) -> Self;
	fn coord_downright(self) -> Self;
}

impl Coords for (i32, i32) {
	type BuildHasher = hashbrown::DefaultHashBuilder;

	fn coord_up(self) -> Self {
		let (x, y) = self;
		(x, y + 1)
	}
	fn coord_down(self) -> Self {
		let (x, y) = self;
		(x, y - 1)
	}
	fn coord_left(self) -> Self {
		let (x, y) = self;
		(x - 1, y)
	}
	fn coord_right(self) -> Self {
		let (x, y) = self;
		(x + 1, y)
	}

	fn coord_upleft(self) -> Self {
		let (x, y) = self;
		(x - 1, y + 1)
	}
	fn coord_upright(self) -> Self {
		let (x, y) = self;
		(x + 1, y + 1)
	}
	fn coord_downleft(self) -> Self {
		let (x, y) = self;
		(x - 1, y - 1)
	}
	fn coord_downright(self) -> Self {
		let (x, y) = self;
		(x + 1, y - 1)
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct I128Coords {
	pub x: i128,
	pub y: i128
}

impl Coords for I128Coords {
	type BuildHasher = I128BuildHasher;

	coords_neighbouring_impl!();
}

impl Hash for I128Coords {
	fn hash<H: Hasher>(&self, state: &mut H) {
		state.write_i128(self.x);
		state.write_i128(self.y);
	}
}

// impl Coord for i128 {
// 	type BuildHasher = I128Hasher;
// }

// // // todo impls
// // pub struct I32Hasher;

// // // todo impls
// // pub struct I32HasherInstance {}

// // // todo impls
// // pub struct I64Hasher;

// // // todo impls
// // pub struct I64HasherInstance {}

// todo impls
pub struct I128BuildHasher;

// todo impls
pub struct I128Hasher {
	state: u128,
	flag: bool
}

impl BuildHasher for I128BuildHasher {
	type Hasher = I128Hasher;

	fn build_hasher(&self) -> I128Hasher {
		I128Hasher { state: 0, flag: false }
	}
}

impl Hasher for I128Hasher {
	#[expect(clippy::as_conversions, reason = "numerical cast")]
	fn finish(&self) -> u64 {
		self.state as u64 ^ (self.state >> 64) as u64
	}

	fn write(&mut self, bytes: &[u8]) {
		let _ = bytes;
		unimplemented!()
	}

	#[expect(clippy::as_conversions, reason = "numerical cast")]
	fn write_i128(&mut self, i: i128) {
		let i = i as u128;
		self.flag = !self.flag;

		self.state ^= if self.flag {
			i
		} else {
			(i >> 64) & (i << 64)
		}
	}
}

macro_rules! coords_neighbouring_impl {
	() => {
		#[inline]
		fn coord_up(self) -> Self {
			let Self { x, y } = self;
			Self { x, y: y + 1 }
		}

		#[inline]
		fn coord_down(self) -> Self {
			let Self { x, y } = self;
			Self { x, y: y - 1 }
		}

		#[inline]
		fn coord_left(self) -> Self {
			let Self { x, y } = self;
			Self { x: x - 1, y }
		}

		#[inline]
		fn coord_right(self) -> Self {
			let Self { x, y } = self;
			Self { x: x + 1, y }
		}

		#[inline]
		fn coord_upleft(self) -> Self {
			let Self { x, y } = self;
			Self { x: x - 1, y: y + 1 }
		}

		#[inline]
		fn coord_upright(self) -> Self {
			let Self { x, y } = self;
			Self { x: x + 1, y: y + 1 }
		}

		#[inline]
		fn coord_downleft(self) -> Self {
			let Self { x, y } = self;
			Self { x: x - 1, y: y - 1 }
		}

		#[inline]
		fn coord_downright(self) -> Self {
			let Self { x, y } = self;
			Self { x: x + 1, y: y - 1 }
		}
	}
}
use coords_neighbouring_impl;
