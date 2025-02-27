//! The String type

use super::pool::{ GlobalPool, Pool, SlicesWrap };
use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fmt::{ self, Debug, Display };
use std::ops::{ Add, AddAssign, Bound, Deref, RangeBounds };
use std::path::Path;
use std::string::{ self as std_string, String as StdString };
use std::str as std_str;

/// A string, whose contents are stored in a string pool.
pub struct String<P: Pool = GlobalPool> {
	raw: P::Raw,
	pool: P
}

/// constructors in default pool
impl String {
	#[inline]
	pub fn new() -> Self {
		Self::new_in(GlobalPool)
	}

	#[inline]
	pub fn from_utf8(vec: Vec<u8>) -> Result<Self, std_string::FromUtf8Error> {
		Self::from_utf8_in(vec, GlobalPool)
	}

	#[inline]
	pub fn from_utf8_slice(slice: &[u8]) -> Result<Self, std_str::Utf8Error> {
		Self::from_utf8_slice_in(slice, GlobalPool)
	}

	#[inline]
	pub fn from_utf8_lossy(v: &[u8]) -> Self {
		Self::from_utf8_lossy_in(v, GlobalPool)
	}

	#[inline]
	pub fn from_utf16(v: &[u16]) -> Result<Self, std_string::FromUtf16Error> {
		Self::from_utf16_in(v, GlobalPool)
	}

	#[inline]
	pub fn from_utf16_lossy(v: &[u16]) -> Self {
		Self::from_utf16_lossy_in(v, GlobalPool)
	}

	// skipping (nightly, for now): from_utf16le
	// skipping (nightly, for now): from_utf16le_lossy
	// skipping (nightly, for now): from_utf16be
	// skipping (nightly, for now): from_utf16be_lossy
	// skipping (nightly): into_raw_parts
	// skipping: from_raw_parts

	#[inline]
	pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
		Self::from_utf8_unchecked_in(bytes, GlobalPool)
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_slice(slice: &[u8]) -> Self {
		Self::from_utf8_unchecked_slice_in(slice, GlobalPool)
	}
}

/// constructors in custom pool
impl<P: Pool> String<P> {
	#[inline]
	pub fn new_in(pool: P) -> Self {
		let raw = pool.raw_empty();
		Self { raw, pool }
	}

	#[inline]
	pub fn from_str_in(s: &str, pool: P) -> Self {
		let raw = unsafe { pool.raw_from_slice(s.as_bytes()) };
		Self { raw, pool }
	}

	#[inline]
	pub fn from_utf8_in(vec: Vec<u8>, pool: P) -> Result<Self, std_string::FromUtf8Error> {
		// running it through std String because it gives us FromUtf8Error, for
		// compat with std String's from_utf8 function, don't think there is
		// any other way to get it than through this
		let std_string = StdString::from_utf8(vec)?;
		let vec = std_string.into_bytes();
		let raw = unsafe { pool.raw_from_vec(vec) };
		Ok(Self { raw, pool })
	}

	#[inline]
	pub fn from_utf8_slice_in(slice: &[u8], pool: P) -> Result<Self, std_str::Utf8Error> {
		let s = std_str::from_utf8(slice)?;
		let raw = unsafe { pool.raw_from_slice(s.as_bytes()) };
		Ok(Self { raw, pool })
	}

	#[inline]
	pub fn from_utf8_lossy_in(v: &[u8], pool: P) -> Self {
		let s = StdString::from_utf8_lossy(v);
		let raw = unsafe { pool.raw_from_slice(s.as_bytes()) };
		Self { raw, pool }
	}

	#[inline]
	pub fn from_utf16_in(v: &[u16], pool: P) -> Result<Self, std_string::FromUtf16Error> {
		let s = StdString::from_utf16(v)?;
		let raw = unsafe { pool.raw_from_slice(s.as_bytes()) };
		Ok(Self { raw, pool })
	}

	#[inline]
	pub fn from_utf16_lossy_in(v: &[u16], pool: P) -> Self {
		let s = StdString::from_utf16_lossy(v);
		let raw = unsafe { pool.raw_from_slice(s.as_bytes()) };
		Self { raw, pool }
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_in(bytes: Vec<u8>, pool: P) -> Self {
		let raw = pool.raw_from_vec(bytes);
		Self { raw, pool }
	}

	#[inline]
	pub unsafe fn from_utf8_unchecked_slice_in(slice: &[u8], pool: P) -> Self {
		let raw = pool.raw_from_slice(slice);
		Self { raw, pool }
	}

	#[inline]
	pub fn to_other_pool<P2: Pool>(&self, pool: P2) -> String<P2> {
		let slice = self.pool.raw_to_slice(&self.raw);
		let raw = unsafe { pool.raw_from_slice(slice) };
		String { raw, pool }
	}

	#[inline]
	pub fn into_other_pool<P2: Pool>(self, pool: P2) -> String<P2> {
		let vec = self.pool.raw_into_vec(self.raw);
		let raw = unsafe { pool.raw_from_vec(vec) };
		String { raw, pool }
	}

	#[inline]
	pub fn clone_to<P2: Pool>(&self, pool: P2) -> String<P2> {
		self.to_other_pool(pool)
	}
}

/// methods that work with any pool
impl<P: Pool> String<P> {
	#[inline]
	pub fn into_bytes(self) -> Vec<u8> {
		self.pool.raw_into_vec(self.raw)
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { std_str::from_utf8_unchecked(self.as_bytes()) }
	}

	// skipping: as_mut_str

	#[inline]
	pub fn push_str(&mut self, string: &str) {
		let new_raw = unsafe {
			self.pool.raw_from_slices(SlicesWrap(&[
				self.as_bytes(),
				string.as_bytes()
			]))
		};

		self.raw = new_raw;
	}

	// skipping (nightly, for now): extend_from_within
	// skipping: capacity
	// skipping: reserve
	// skipping: reserve_exact
	// skipping: try_reserve
	// skipping: try_reserve_exact
	// skipping: shrink_to_fit
	// skipping: shrink_to

	#[inline]
	pub fn push(&mut self, ch: char) {
		self.push_str(ch.encode_utf8(&mut [0u8; 4]));
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		self.pool.raw_to_slice(&self.raw)
	}

	#[inline]
	pub fn truncate(&mut self, new_len: usize) {
		if new_len > self.len() { return }

		assert!(self.is_char_boundary(new_len));
		let new_slice = &self.as_bytes()[..new_len];
		let new_raw = unsafe { self.pool.raw_from_slice(new_slice) };

		self.raw = new_raw;
	}

	#[inline]
	pub fn pop(&mut self) -> Option<char> {
		let ch = self.chars().next_back()?;
		let new_len = self.len() - ch.len_utf8();

		let new_slice = &self.as_bytes()[..new_len];
		let new_raw = unsafe { self.pool.raw_from_slice(new_slice) };

		self.raw = new_raw;
		Some(ch)
	}

	#[inline]
	pub fn remove(&mut self, i: usize) -> char {
		// let slice =
		let ch = self[i..].chars().next()
			.expect("cannot remove a char from the end of a string");
		let next = i + ch.len_utf8();

		let slice = self.as_bytes();
		let new_raw = unsafe {
			self.pool.raw_from_slices(SlicesWrap(&[
				&slice[..i],
				&slice[next..]
			]))
		};

		self.raw = new_raw;
		ch
	}

	// skipping (nightly, for now): remove_matches

	pub fn retain<F>(&mut self, mut f: F)
	where
		F: FnMut(char) -> bool
	{
		// reason for capacity:
		// worst case is true, false, true, false, etc
		// which is keeping half of 1 byte chars
		// so if chars are longer or longer sequences of no switching, it'll be less
		// +1 is for odd ones, like. true false true, len 3, (3 / 2) + 1 == 2, will fit
		// i mean, worst case is worse performance, it doesn't lead to correctness
		// issues, so its not the most critical :p
		let mut retained = Vec::with_capacity((self.len() / 2) + 1);
		let mut state = None;

		for (i, char) in self.char_indices() {
			match (f(char), state) {
				// start new streak
				(true, None) => {
					state = Some(i);
				}

				// end streak
				(false, Some(i_start)) => {
					retained.push(self[i_start..i].as_bytes());
					state = None;
				}

				// continue true streak
				(true, Some(_)) => { /* noop */ }

				// continue false streak
				(false, None) => { /* noop */ }
			}
		}

		if let Some(i_start) = state {
			retained.push(self[i_start..].as_bytes());
		}

		let new_raw = unsafe { self.pool.raw_from_slices(SlicesWrap(&retained)) };
		self.raw = new_raw;
	}

	#[inline]
	pub fn insert(&mut self, i: usize, ch: char) {
		self.insert_str(i, ch.encode_utf8(&mut [0u8; 4]));
	}

	#[inline]
	pub fn insert_str(&mut self, i: usize, string: &str) {
		assert!(self.is_char_boundary(i));
		let slice = self.as_bytes();

		let new_raw = unsafe {
			self.pool.raw_from_slices(SlicesWrap(&[
				&slice[..i],
				string.as_bytes(),
				&slice[i..]
			]))
		};

		self.raw = new_raw;
	}

	// skipping: as_mut_vec

	#[inline]
	pub fn split_off(&mut self, at: usize) -> Self {
		self.split_off_in(at, self.pool.clone())
	}

	#[inline]
	pub fn split_off_in<P2: Pool>(&mut self, at: usize, pool: P2) -> String<P2> {
		assert!(self.is_char_boundary(at));

		let self_raw = unsafe { self.pool.raw_from_slice(self[..at].as_bytes()) };
		let other_raw = unsafe { pool.raw_from_slice(self[at..].as_bytes()) };

		self.raw = self_raw;
		String { raw: other_raw, pool }
	}

	#[inline]
	pub fn clear(&mut self) {
		self.raw = self.pool.raw_empty();
	}

	// skipping (for now): drain
	// skipping (for now): replace_range

	#[inline]
	pub fn into_boxed_str(self) -> Box<str> {
		let boxed = self.pool.raw_into_boxed_slice(self.raw);
		unsafe { std_str::from_boxed_utf8_unchecked(boxed) }
	}

	#[inline]
	pub fn leak<'h>(self) -> &'h mut str {
		let slice = self.pool.raw_into_vec(self.raw).leak();
		unsafe { std_str::from_utf8_unchecked_mut(slice) }
	}
}

impl<P: Pool> Add<&str> for String<P> {
	type Output = Self;

	#[inline]
	fn add(mut self, rhs: &str) -> Self {
		// delegates to AddAssign<&str> for String<P>
		self += rhs;
		self
	}
}

impl<P: Pool> Add<&str> for &String<P> {
	type Output = String<P>;

	#[inline]
	fn add(self, rhs: &str) -> String<P> {
		let raw = unsafe {
			self.pool.raw_from_slices(SlicesWrap(&[
				self.as_bytes(),
				rhs.as_bytes()
			]))
		};
		let pool = self.pool.clone();

		String { raw, pool }
	}
}

impl<P: Pool, P2: Pool> Add<String<P2>> for String<P> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: String<P2>) -> Self {
		// delegates to Add<&str> for String<P>
		self + &*rhs
	}
}

impl<P: Pool, P2: Pool> Add<String<P2>> for &String<P> {
	type Output = String<P>;

	#[inline]
	fn add(self, rhs: String<P2>) -> String<P> {
		// delegates to Add<&str> for &String<P>
		self + &*rhs
	}
}

impl<P: Pool, P2: Pool> Add<&String<P2>> for String<P> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: &String<P2>) -> Self {
		// delegates to Add<&str> for String<P>
		self + &**rhs
	}
}

impl<P: Pool, P2: Pool> Add<&String<P2>> for &String<P> {
	type Output = String<P>;

	#[inline]
	fn add(self, rhs: &String<P2>) -> String<P> {
		// delegates to Add<&str> for &String<P>
		self + &**rhs
	}
}

impl<P: Pool> Add<StdString> for String<P> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: StdString) -> Self {
		// delegates to Add<&str> for String<P>
		self + &*rhs
	}
}

impl<P: Pool> Add<&StdString> for String<P> {
	type Output = Self;

	#[inline]
	fn add(self, rhs: &StdString) -> Self {
		// delegates to Add<&str> for String<P>
		self + &**rhs
	}
}

impl<P: Pool> Add<StdString> for &String<P> {
	type Output = String<P>;

	#[inline]
	fn add(self, rhs: StdString) -> String<P> {
		// delegates to Add<&str> for &String<P>
		self + &*rhs
	}
}

impl<P: Pool> Add<&StdString> for &String<P> {
	type Output = String<P>;

	#[inline]
	fn add(self, rhs: &StdString) -> String<P> {
		// delegates to Add<&str> for &String<P>
		self + &**rhs
	}
}

impl<P: Pool> AddAssign<&str> for String<P> {
	#[inline]
	fn add_assign(&mut self, rhs: &str) {
		self.push_str(rhs);
	}
}

impl<P: Pool, P2: Pool> AddAssign<String<P2>> for String<P> {
	#[inline]
	fn add_assign(&mut self, rhs: String<P2>) {
		// delegates to AddAssign<&str> for String<P>
		*self += &*rhs
	}
}

impl<P: Pool, P2: Pool> AddAssign<&String<P2>> for String<P> {
	#[inline]
	fn add_assign(&mut self, rhs: &String<P2>) {
		// delegates to AddAssign<&str> for String<P>
		*self += &**rhs
	}
}

impl<P: Pool> AsRef<[u8]> for String<P> {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<P: Pool> AsRef<OsStr> for String<P> {
	#[inline]
	fn as_ref(&self) -> &OsStr {
		self.as_str().as_ref()
	}
}

impl<P: Pool> AsRef<Path> for String<P> {
	#[inline]
	fn as_ref(&self) -> &Path {
		self.as_str().as_ref()
	}
}

impl<P: Pool> AsRef<str> for String<P> {
	#[inline]
	fn as_ref(&self) -> &str {
		self
	}
}

impl<P: Pool> Borrow<str> for String<P> {
	#[inline]
	fn borrow(&self) -> &str {
		self
	}
}

impl<P: Pool> Clone for String<P> {
	#[inline]
	fn clone(&self) -> Self {
		let raw = self.pool.raw_clone(&self.raw);
		let pool = self.pool.clone();
		Self { raw, pool }
	}
}

impl<P: Pool> Debug for String<P> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("String")
			.field("string", &self.as_str())
			.field("pool", &self.pool)
			.finish()
	}
}

impl<P: Pool> Default for String<P> {
	#[inline]
	fn default() -> Self {
		let pool = P::default();
		let raw = pool.raw_empty();
		Self { raw, pool }
	}
}

impl<P: Pool> Deref for String<P> {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

impl<P: Pool> Display for String<P> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&**self, f)
	}
}

// impl<'h, P: Pool> Extend<&'h char> for String<P> {}
// impl<'h, P: Pool> Extend<&'h str> for String<P> {}
// impl<P: Pool> Extend<Box<str>> for String<P> {}
// impl<'h, P: Pool> Extend<Cow<'h, str>> for String<P> {}
// impl<P: Pool, P2: Pool> Extend<String<P2>> for String<P> {}
// impl<P: Pool> Extend<StdString> for String<P> {}
// impl<P: Pool> Extend<char> for String<P> {}

impl From<&str> for String {
	#[inline]
	fn from(s: &str) -> Self {
		Self::from_str_in(s, GlobalPool)
	}
}

impl<P: Pool> From<(&str, P)> for String<P> {
	#[inline]
	fn from((s, pool): (&str, P)) -> Self {
		Self::from_str_in(s, pool)
	}
}

// impl<'h, P: Pool> From<&'h String<P>> for Cow<'h, str> {}
// impl<P: Pool, P2: Pool> From<&String<P2>> for String<P> {}
// impl<P: Pool> From<&mut str> for String<P> {}
// impl<P: Pool> From<&str> for String<P> {}
// impl<P: Pool> From<Box<str>> for String<P> {}
// impl<'h, P: Pool> From<Cow<'h, str>> for String<P> {}
// impl<P: Pool> From<String<P>> for Arc<str> {}
// impl<P: Pool> From<String<P>> for Box<dyn Error> {}
// impl<P: Pool> From<String<P>> for Box<dyn Error + Send + Sync> {}
// impl<P: Pool> From<String<P>> for Box<str> {}
// impl<'h, P: Pool> From<String<P>> for Cow<'h, str> {}
// impl<P: Pool> From<String<P>> for OsString {}
// impl<P: Pool> From<String<P>> for PathBuf {}
// impl<P: Pool> From<String<P>> for Rc<str> {}
// impl<P: Pool> From<String<P>> for Vec<u8> {}
// impl<P: Pool> From<char> for String<P> {}

// impl<'h, P: Pool> FromIterator<&'h char> for String<P> {}
// impl<'h, P: Pool> FromIterator<&'h str> for String<P> {}
// impl<P: Pool> FromIterator<Box<str>> for String<P> {}
// impl<'h, P: Pool> FromIterator<Cow<'h, str>> for String<P> {}
// impl<'h, P: Pool> FromIterator<String<P>> for Cow<'h, str> {}
// impl<'h, P: Pool, P2: Pool> FromIterator<String<P2>> for String<P> {}
// impl<P: Pool> FromIterator<char> for String<P> {}

// impl<P: Pool> FromStr for String<P> {}
// impl<P: Pool> Hash for String<P> {}

// impl<P: Pool> Index<Range<usize>> for String<P> {}
// impl<P: Pool> Index<RangeFull> for String<P> {}
// impl<P: Pool> Index<RangeInclusive<usize>> for String<P> {}
// impl<P: Pool> Index<RangeTo<usize>> for String<P> {}
// impl<P: Pool> Index<RangeToInclusive<usize>> for String<P> {}

// impl Ord for String {}

// impl<'h, P: Pool> PartialEq<&'h str> for String<P> {}
// impl<'h, P: Pool> PartialEq<Cow<'h, str>> for String<P> {}
// impl<'h, P: Pool> PartialEq<String<P>> for &'h str {}
// impl<'h, P: Pool> PartialEq<String<P>> for Cow<'h, str> {}
// impl<P: Pool> PartialEq<String<P>> for str {}
// impl<P: Pool> PartialEq<str> for String<P> {}
// impl<P: Pool> PartialEq for String<P> {}

// impl<P: Pool> PartialOrd for String<P> {}

// impl<'h, 'h2, P: Pool> Pattern<'h2> for &'h String<P> {}
// impl<P: Pool> ToSocketAddrs for String<P> {}
// impl<P: Pool> Write for String<P> {}

// impl<P: Pool> Eq for String<P> {}
// impl<P: Pool> StructuralEq for String<P> {}
// impl<P: Pool> StructuralPartialEq for String<P> {}

#[cfg(test)]
mod tests {
	use super::Pool;
	use super::*;
	use rand::{ Rng, thread_rng };
	use std::fmt::Debug;

	#[test]
	fn new() {
		let empty = "";
		let new = String::new();
		let new_custom_pool = String::new_in(TestPool);

		assert_eq!(empty, new.as_str());
		assert_eq!(empty, new_custom_pool.as_str());
	}

	#[test]
	fn from_utf8_and_slice() {
		fn assert_ok(s: &str, vec: Vec<u8>) {
			let res = String::from_utf8(vec.clone());
			assert!(res.is_ok());
			assert_eq!(&*res.unwrap(), s);

			let res = String::from_utf8_in(vec.clone(), TestPool);
			assert!(res.is_ok());
			assert_eq!(&*res.unwrap(), s);

			let res = String::from_utf8_slice(&vec);
			assert!(res.is_ok());
			assert_eq!(&*res.unwrap(), s);

			let res = String::from_utf8_slice_in(&vec, TestPool);
			assert!(res.is_ok());
			assert_eq!(&*res.unwrap(), s);
		}

		fn assert_err(vec: Vec<u8>) {
			assert!(String::from_utf8(vec.clone()).is_err());
			assert!(String::from_utf8_in(vec.clone(), TestPool).is_err());
			assert!(String::from_utf8_slice(&vec).is_err());
			assert!(String::from_utf8_slice_in(&vec, TestPool).is_err());
		}

		// from macOS's text thingie
		// 🫐
		// bosbessen
		// Unicode: U+1FAD0, UTF-8: 0xF0, 0x9F, 0xAB, 0x90,
		assert_ok("🫐", vec![0xF0u8, 0x9F, 0xAB, 0x90]);

		let complex_string = "programmering er gøy 烏龜很喜歡吃藍莓 ik weet het niet, ik schrijf gewoon willekeurig zinnen lol okay this is a good \n\n test of unicode right? ✨ ÙwÚ ✨";
		assert_ok(complex_string, complex_string.as_bytes().to_vec());

		// stolen from std String's from_utf8 doc
		assert_err(vec![0u8, 159, 146, 150]);
	}

	#[test]
	fn push() {
		let mut string_std = StdString::new();
		let mut string = String::new();
		let mut string_custom_pool = String::new_in(TestPool);

		for _ in 0..20 {
			let s = rand_std_string();
			string_std.push_str(&s);
			string.push_str(&s);
			string_custom_pool.push_str(&s);
			assert_eq!(&*string_std, &*string);
			assert_eq!(&*string_std, &*string_custom_pool);
		}
	}

	#[derive(Clone, Debug, Default)]
	struct TestPool;

	impl Pool for TestPool {
		type Raw = StdString;

		unsafe fn raw_from_slices(&self, slices: SlicesWrap) -> Self::Raw {
			let vec = slices.to_boxed_slice().into_vec();
			unsafe { StdString::from_utf8_unchecked(vec) }
		}

		fn raw_to_slice<'r>(&self, raw: &'r Self::Raw) -> &'r [u8] {
			raw.as_bytes()
		}
	}

	fn rand_std_string() -> StdString {
		let mut rng = thread_rng();
		let mut vec = vec![' '; rng.gen_range(50..100)];
		rng.fill(&mut *vec);
		vec.into_iter().collect()
	}
}
