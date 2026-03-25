use core::error::Error;
use core::panic::Location;
use core::fmt::{ self, Debug, Display };

pub struct Err<E> {
	frame: TypedFrame<E>
}

impl<E> Err<E>
where
	E: Error + Send + Sync + 'static
{
	#[inline]
	#[track_caller]
	pub fn from_err(err: E) -> Self {
		let children = walk(&err);

		Err {
			frame: TypedFrame {
				err,
				location: Location::caller(),
				children
			}
		}
	}

	#[inline]
	#[track_caller]
	pub fn from_errs<C>(err: E, children: &mut dyn Iterator<Item = C>) -> Self
	where
		C: Error + Send + Sync + 'static
	{
		let children = children.into_iter()
			.map(|err| {
				let children = walk(&err);

				Frame {
					err: Box::new(err),
					location: Location::caller(),
					children
				}
			}).collect();

		Err {
			frame: TypedFrame {
				err,
				location: Location::caller(),
				children
			}
		}
	}

	#[inline]
	#[track_caller]
	pub fn raise<E2>(self, err: E2) -> Err<E2>
	where
		E2: Error + Send + Sync + 'static
	{
		let mut new = Err::from_err(err);
		new.frame.children.push(self.frame.into());
		new
	}
}

impl<E> Debug for Err<E>
where
	E: Error + Send + Sync + 'static
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		print_err_dbg(f, (&self.frame).into(), 0, &mut String::new())
	}
}

impl<E> Display for Err<E>
where
	E: Error + Send + Sync + 'static
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&self.frame.err, f)
	}
}

struct TypedFrame<E> {
	err: E,
	location: &'static Location<'static>,
	children: Vec<Frame>
}

struct Frame {
	err: Box<dyn Error + Send + Sync + 'static>,
	location: &'static Location<'static>,
	children: Vec<Frame>
}

impl<E> From<TypedFrame<E>> for Frame
where
	E: Error + Send + Sync + 'static
{
	#[inline]
	fn from(frame: TypedFrame<E>) -> Self {
		let TypedFrame { err, location, children } = frame;
		let err = Box::new(err);
		Frame { err, location, children }
	}
}

struct BorrowedFrame<'h> {
	err: &'h (dyn Error + Send + Sync),
	location: &'static Location<'static>,
	children: &'h [Frame]
}

impl<'h, E> From<&'h TypedFrame<E>> for BorrowedFrame<'h>
where
	E: Error + Send + Sync
{
	fn from(frame: &'h TypedFrame<E>) -> Self {
		let TypedFrame { err, location, children } = frame;
		Self { err, location, children }
	}
}

impl<'h> From<&'h Frame> for BorrowedFrame<'h> {
	fn from(frame: &'h Frame) -> Self {
		let Frame { err, location, children } = frame;
		Self { err: &**err, location, children }
	}
}

struct StringError {
	debug: Box<str>,
	display: Box<str>
}

impl Debug for StringError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.debug)
	}
}

impl Display for StringError {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.display)
	}
}

impl Error for StringError {}

#[track_caller]
fn walk(err: &(dyn Error + 'static)) -> Vec<Frame> {
	err.source()
		.map(|err| vec![Frame {
			err: Box::new(StringError {
				debug: format!("{err:?}").into_boxed_str(),
				display: format!("{err}").into_boxed_str()
			}),
			location: Location::caller(),
			children: walk(err)
		}])
		.unwrap_or_default()
}

fn print_err_dbg(
	f: &mut fmt::Formatter<'_>,
	frame: BorrowedFrame<'_>,
	level: usize,
	prefix_buf: &mut String
) -> fmt::Result {
	let prefix_buf_len = prefix_buf.len();
	let level_in_spaces = level * 2;
	if prefix_buf_len < level_in_spaces {
		let additional = level_in_spaces - prefix_buf_len;

		prefix_buf.reserve(additional);
		(0..additional).for_each(|_| prefix_buf.push(' '));
	}
	let prefix = &prefix_buf[..level_in_spaces];

	writeln!(
		f,
		"{prefix}- err in {file}, at {line}:{col}",
		file = frame.location.file(),
		line = frame.location.line(),
		col = frame.location.column()
	)?;

	writeln!(f, "{prefix}  {}", frame.err)?;

	if !frame.children.is_empty() {
		writeln!(f, "{prefix}  caused by:")?;
	}

	for child in frame.children {
		print_err_dbg(f, child.into(), level + 1, prefix_buf)?;
	}

	Ok(())
}

pub trait ErrorExt {
	fn raise(self) -> Err<Self>
	where
		Self: Sized;
}

impl<E> ErrorExt for E
where
	E: Error + Send + Sync + 'static
{
	#[inline]
	#[track_caller]
	fn raise(self) -> Err<Self> {
		Err::from_err(self)
	}
}

pub trait Ext {
	type Ok;
	type Err;

	fn or_raise<E>(self, err: impl FnOnce() -> E) -> Result<Self::Ok, Err<E>>
	where
		E: Error + Send + Sync + 'static;
}

impl<T> Ext for Option<T> {
	type Ok = T;
	type Err = ();

	#[inline]
	#[track_caller]
	fn or_raise<E>(self, err: impl FnOnce() -> E) -> Result<Self::Ok, Err<E>>
	where
		E: Error + Send + Sync + 'static
	{
		match self {
			Some(value) => { Ok(value) }
			None => { Err(Err::from_err(err())) }
		}
	}
}

impl<T, E> Ext for Result<T, E>
where
	E: Error + Send + Sync + 'static
{
	type Ok = T;
	type Err = E;

	#[inline]
	#[track_caller]
	fn or_raise<E2>(self, err: impl FnOnce() -> E2) -> Result<<Self as Ext>::Ok, Err<E2>>
	where
		E2: Error + Send + Sync + 'static
	{
		match self {
			Ok(value) => { Ok(value) }
			Err(prev_err) => { Err(Err::from_err(prev_err).raise(err())) }
		}
	}
}

impl<T, E> Ext for Result<T, Err<E>>
where
	E: Error + Send + Sync + 'static
{
	type Ok = T;
	type Err = E;

	#[inline]
	#[track_caller]
	fn or_raise<E2>(self, err: impl FnOnce() -> E2) -> Result<<Self as Ext>::Ok, Err<E2>>
	where
		E2: Error + Send + Sync + 'static
	{
		match self {
			Ok(value) => { Ok(value) }
			Err(prev_err) => { Err(prev_err.raise(err())) }
		}
	}
}
