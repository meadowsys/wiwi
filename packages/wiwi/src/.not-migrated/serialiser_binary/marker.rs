//! Marker constants

use super::consts;

macro_rules! unsigned_smallint_range {
	() => { 0x00..=0x7f }
}
pub(crate) use unsigned_smallint_range;

macro_rules! signed_smallint_range {
	() => { unsigned_smallint_range!() | 0xc0..=0xff }
}
pub(crate) use signed_smallint_range;

consts! {
	const type u8

	/// Marker constants
	pub mod markers

	MARKER_SMALLINT_RANGE_START = 0x00
	MARKER_SMALLINT_RANGE_END = 0x7f

	MARKER_U8 = 0x80
	MARKER_I8 = 0x81

	MARKER_U16 = 0x82
	MARKER_I16 = 0x83

	MARKER_U24 = 0x84
	MARKER_I24 = 0x85

	MARKER_U32 = 0x86
	MARKER_I32 = 0x87

	MARKER_U40 = 0x88
	MARKER_I40 = 0x89

	MARKER_U48 = 0x8a
	MARKER_I48 = 0x8b

	MARKER_U56 = 0x8c
	MARKER_I56 = 0x8d

	MARKER_U64 = 0x8e
	MARKER_I64 = 0x8f

	MARKER_U72 = 0x90
	MARKER_I72 = 0x91

	MARKER_U80 = 0x92
	MARKER_I80 = 0x93

	MARKER_U88 = 0x94
	MARKER_I88 = 0x95

	MARKER_U96 = 0x96
	MARKER_I96 = 0x97

	MARKER_U104 = 0x98
	MARKER_I104 = 0x99

	MARKER_U112 = 0x9a
	MARKER_I112 = 0x9b

	MARKER_U120 = 0x9c
	MARKER_I120 = 0x9d

	MARKER_U128 = 0x9e
	MARKER_I128 = 0x9f

	/// For single type arrays of bools, this serves as the marker for the
	/// bool type
	MARKER_BOOL = 0xa0
	MARKER_BOOL_FALSE = 0xa0
	MARKER_BOOL_TRUE = 0xa1

	/// The upper bits that are the same across all `bool` markers, shifted down
	/// by 1 (0x50)
	///
	/// No other marker has this property, so checking
	/// `byte >> 1 == MARKER_BOOL_UPPER_BITS_COMMON` is sufficient to determine
	/// if a byte represents a bool value.
	MARKER_BOOL_UPPER_BITS_COMMON = MARKER_BOOL_FALSE >> 1

	/// Reserved for currently unimplemented IEEE754-2008 binary16 type
	MARKER_F16 = 0xa2
	MARKER_F32 = 0xa3
	MARKER_F64 = 0xa4
	/// Reserved for currently unimplemented IEEE754-2008 binary128 type
	MARKER_F128 = 0xa5
	/// Reserved for currently unimplemented IEEE754-2008 binary256 type
	MARKER_F256 = 0xa6

	MARKER_NULL = 0xa7

	MARKER_STR_8 = 0xa8
	MARKER_STR_XL = 0xa9

	MARKER_ARRAY_8 = 0xaa
	MARKER_ARRAY_XL = 0xab

	MARKER_MAP_8 = 0xac
	MARKER_MAP_XL = 0xad

	MARKER_SINGLE_TYPE_ARRAY_8 = 0xae
	MARKER_SINGLE_TYPE_ARRAY_XL = 0xaf

	MARKER_BINARY_8 = 0xb0
	MARKER_BINARY_XL = 0xb1

	MARKER_SET_8 = 0xb2 // TODO: impl this
	MARKER_SET_XL = 0xb3 // TODO: impl this

	MARKER_UNASSIGNED_B4 = 0xb4
	MARKER_UNASSIGNED_B5 = 0xb5
	MARKER_UNASSIGNED_B6 = 0xb6
	MARKER_UNASSIGNED_B7 = 0xb7
	MARKER_UNASSIGNED_B8 = 0xb8
	MARKER_UNASSIGNED_B9 = 0xb9
	MARKER_UNASSIGNED_BA = 0xba
	MARKER_UNASSIGNED_BB = 0xbb

	MARKER_2_BYTE_SECTION_1 = 0xbc
	MARKER_2_BYTE_SECTION_2 = 0xbd
	MARKER_3_BYTE = 0xbe
	MARKER_4_BYTE = 0xbf

	MARKER_SMALLINT_NEGATIVE_RANGE_START = 0xc0
	MARKER_SMALLINT_NEGATIVE_RANGE_END = 0xff
}
