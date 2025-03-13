use crate::prelude_internal::*;

crate::impl_chain_conversions!([] String);

crate::chain_fns! {
	impl [] String;

	// as_ascii
	// as_bytes
	// as_bytes_mut
	// as_ptr
	// as_mut_ptr
	// as_mut_str
	// as_mut_vec
	// as_str
	// bytes
	// capacity
	// ceil_char_boundary
	// char_indices
	// chars

	doc "String::clear"
	fn clear(inner) {
		inner.clear();
	}

	// contains
	// drain
	// encode_utf16
	// ends_with
	// eq_ignore_ascii_case
	// escape_debug
	// escape_default
	// escape_unicode
	// extend_from_within
	// find
	// floor_char_boundary
	// from_raw_parts
	// from_utf8
	// from_utf8_lossy
	// from_utf8_lossy_owned
	// from_utf8_unchecked
	// from_utf16
	// from_utf16_lossy
	// from_utf16be
	// from_utf16be_lossy
	// from_utf16le
	// from_utf16le_lossy
	// get
	// get_mut
	// get_unchecked
	// get_unchecked_mut
	// insert
	// insert_str
	// into_boxed_str
	// into_bytes
	// into_chars
	// into_raw_parts
	// is_ascii
	// is_char_boundary
	// is_empty
	// is_empty
	// leak

	doc "String::len"
	fn len(inner, out: impl Output<usize>) {
		out.write(inner.len())
	}

	// lines
	// lines_any
	// make_ascii_lowercase
	// make_ascii_uppercase
	// match_indices
	// matches
	// parse
	// pop

	doc "String::push"
	fn push(inner, ch: char) {
		inner.push(ch)
	}

	doc "String::push_str"
	fn push_str(inner, string: &str) {
		inner.push_str(string)
	}

	// remove
	// remove_matches
	// repeat
	// replace
	// replace_range
	// replacen

	doc "String::reserve"
	fn reserve(inner, additional: usize) {
		inner.reserve(additional)
	}

	doc "String::reserve_exact"
	fn reserve_exact(inner, additional: usize) {
		inner.reserve_exact(additional)
	}

	doc "String::retain"
	fn retain(inner, f: impl FnMut(char) -> bool) {
		inner.retain(f)
	}

	// rfind
	// rmatch_indices
	// rmatches
	// rsplit
	// rsplit_once
	// rsplit_terminator
	// rsplitn
	// shrink_to
	// shrink_to_fit
	// slice_mut_unchecked
	// slice_unchecked
	// split_inclusive
	// split
	// split_ascii_whitespace
	// split_at
	// split_at_checked
	// split_at_mut
	// split_at_mut_checked
	// split_off
	// split_once
	// split_terminator
	// split_whitespace
	// splitn
	// starts_with
	// strip_prefix
	// strip_suffix
	// substr_range
	// to_ascii_lowercase
	// to_ascii_uppercase
	// to_lowercase
	// to_uppercase
	// trim
	// trim_ascii
	// trim_ascii_end
	// trim_ascii_start
	// trim_end
	// trim_end_matches
	// trim_left
	// trim_left_matches
	// trim_matches
	// trim_right
	// trim_right_matches
	// trim_start
	// trim_start_matches
	// truncate
	// try_reserve
	// try_reserve_exact
	// try_with_capacity
	// with_capacity
}

/*
Trait Implementations
Add<&str>
AddAssign<&str>
AsMut<str>
AsRef<OsStr>
AsRef<Path>
AsRef<[u8]>
AsRef<str>
Borrow<str>
BorrowMut<str>
Clone
Debug
Default
Deref
DerefMut
DerefPure
Display
Eq
Extend<&'a AsciiChar>
Extend<&'a char>
Extend<&'a str>
Extend<AsciiChar>
Extend<Box<str, A>>
Extend<Cow<'a, str>>
Extend<String>
Extend<char>
From<&'a String>
From<&String>
From<&mut str>
From<&str>
From<Box<str>>
From<Cow<'a, str>>
From<String>
From<String>
From<String>
From<String>
From<String>
From<String>
From<String>
From<String>
From<String>
From<char>
FromIterator<&'a char>
FromIterator<&'a str>
FromIterator<Box<str, A>>
FromIterator<Cow<'a, str>>
FromIterator<String>
FromIterator<String>
FromIterator<String>
FromIterator<char>
FromStr
Hash
Index<I>
IndexMut<I>
Ord
PartialEq
PartialEq<&'a str>
PartialEq<ByteStr>
PartialEq<ByteString>
PartialEq<Cow<'a, str>>
PartialEq<String>
PartialEq<String>
PartialEq<String>
PartialEq<String>
PartialEq<String>
PartialEq<str>
PartialOrd
Pattern
StructuralPartialEq
ToSocketAddrs
TryFrom<&'a ByteStr>
TryFrom<ByteString>
TryFrom<CString>
TryFrom<Vec<u8>>
Write
*/
