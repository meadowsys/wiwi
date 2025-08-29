//! Prelude exporting types from the Rust standard library (`std`)

pub extern crate alloc as alloc_crate;
pub extern crate core;
pub extern crate std;

pub use std::{
	assert,
	assert_eq,
	assert_ne,
	debug_assert,
	debug_assert_eq,
	debug_assert_ne,

	print,
	println,
	eprint,
	eprintln,
	format,
	format_args,

	panic,
	compile_error,
	todo,
	unreachable,

	cfg,
	file,
	line,
	column,

	concat,
	dbg,
	stringify,
	vec,

	thread_local,

	array,
	env,
	hint,
	ptr,
	str
};

pub use std::alloc::{
	self as alloc_mod,
	alloc,
	alloc_zeroed,
	dealloc,
	realloc
};
pub use std::any::{
	Any,
	TypeId,
	type_name,
	type_name_of_val
};
pub use std::borrow::{
	Borrow,
	BorrowMut,
	Cow,
	ToOwned
};
pub use std::boxed::Box;
pub use std::cell::{ self, UnsafeCell };
pub use std::clone::{ self, Clone };
pub use std::cmp::{
	self,
	Eq,
	Ord,
	PartialEq,
	PartialOrd
};
pub use std::convert::{
	AsMut,
	AsRef,
	From,
	Into,
	TryFrom,
	TryInto,
	Infallible,
	identity
};
pub use std::default::Default;
pub use std::fmt::{ self, Debug, Display };
pub use std::fs::{ self, File };
pub use std::future::{ self, Future, IntoFuture };
pub use std::hash::{ self, Hash, Hasher };
pub use std::iter::{
	self,
	Iterator,
	FromIterator,
	IntoIterator,
	DoubleEndedIterator,
	ExactSizeIterator,
	Extend
};
pub use std::marker::{
	self,
	Copy,
	Send,
	Sync,
	Sized,
	Unpin,
	PhantomData,
	PhantomPinned
};
pub use std::mem::{
	self,
	ManuallyDrop,
	MaybeUninit,
	align_of,
	align_of_val,
	size_of,
	size_of_val,
	transmute,
	transmute_copy,
	drop,
	forget,
	needs_drop,
	replace,
	swap,
	take,
	zeroed
};
pub use std::num::{ self, NonZero, Saturating, Wrapping };
pub use std::ops::{
	self,
	Deref,
	DerefMut,
	Drop,
	Fn,
	FnMut,
	FnOnce
};
pub use std::option::{ self, Option, Option::Some, Option::None };
pub use std::panic::{ UnwindSafe, RefUnwindSafe };
pub use std::path::{ self, Path, PathBuf };
pub use std::result::{ self, Result, Result::Ok, Result::Err };
pub use std::rc::{ Rc, Weak as RcWeak };
pub use std::string::{ self, String, ToString };
pub use std::sync::{ Arc, Weak as ArcWeak };
pub use std::sync::atomic::{
	self,
	AtomicBool,
	AtomicI8,
	AtomicI16,
	AtomicI32,
	AtomicI64,
	// AtomicI128,
	AtomicIsize,
	AtomicU8,
	AtomicU16,
	AtomicU32,
	AtomicU64,
	// AtomicU128,
	AtomicUsize,
	AtomicPtr,
	compiler_fence,
	fence
};
pub use std::vec::Vec;

// "augmented" modules by wiwi
pub use crate::slice;
