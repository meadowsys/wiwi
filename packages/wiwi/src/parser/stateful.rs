use crate::prelude::*;
use crate::num::*;
use super::{ stateless, util, Error, Input, Needle, Parser as _, ParserPhantom, Result, Success };

pub trait ParserStateful<I, O, E = ()>
where
	I: Input
{
	fn parse(&mut self, input: I) -> Result<I, O, E>;

	#[inline]
	fn map<F, O2>(self, f: F) -> util::Map<Self, F, O>
	where
		Self: Sized,
		F: FnMut(O) -> O2
	{
		util::map(self, f)
	}
}

impl<F, I, O, E> ParserStateful<I, O, E> for F
where
	I: Input,
	F: FnMut(I) -> Result<I, O, E>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, O, E> {
		self(input)
	}
}

#[inline]
pub fn delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>(
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter
) -> Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: ParserStateful<I, OBefore, EBefore>,
	P: ParserStateful<I, O, E>,
	PAfter: ParserStateful<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	Delimited {
		parser_before,
		parser,
		parser_after,
		__marker: PhantomData
	}
}

macro_rules! decl_num_fn {
	{ $($(#[$meta:meta])* $struct:ident $fn:ident)* } => {
		$(
			$(#[$meta])*
			#[inline]
			pub fn $fn() -> $struct {
				$struct { inner: stateless::$fn() }
			}
		)*
	}
}

decl_num_fn! {
	NumU8LE num_u8_le
	NumU8BE num_u8_be
	NumU8NE num_u8_ne

	NumU16LE num_u16_le
	NumU16BE num_u16_be
	NumU16NE num_u16_ne

	NumU32LE num_u32_le
	NumU32BE num_u32_be
	NumU32NE num_u32_ne

	NumU64LE num_u64_le
	NumU64BE num_u64_be
	NumU64NE num_u64_ne

	NumU128LE num_u128_le
	NumU128BE num_u128_be
	NumU128NE num_u128_ne

	NumI8LE num_i8_le
	NumI8BE num_i8_be
	NumI8NE num_i8_ne

	NumI16LE num_i16_le
	NumI16BE num_i16_be
	NumI16NE num_i16_ne

	NumI32LE num_i32_le
	NumI32BE num_i32_be
	NumI32NE num_i32_ne

	NumI64LE num_i64_le
	NumI64BE num_i64_be
	NumI64NE num_i64_ne

	NumI128LE num_i128_le
	NumI128BE num_i128_be
	NumI128NE num_i128_ne

	NumF32LE num_f32_le
	NumF32BE num_f32_be
	NumF32NE num_f32_ne

	NumF64LE num_f64_le
	NumF64BE num_f64_be
	NumF64NE num_f64_ne

	NumUsizeLE num_usize_le
	NumUsizeBE num_usize_be
	NumUsizeNE num_usize_ne
	NumIsizeLE num_isize_le
	NumIsizeBE num_isize_be
	NumIsizeNE num_isize_ne
}

#[inline]
pub fn tag<T>(tag: T) -> Tag<T> {
	Tag { inner: stateless::tag(tag) }
}

#[inline]
pub fn take<N>(amount: N) -> Take
where
	N: IntoUsize
{
	Take { inner: stateless::take(amount) }
}

#[inline]
pub fn take_array<const N: usize>() -> TakeArray<N> {
	TakeArray { inner: stateless::take_array() }
}

#[inline]
pub fn take_const<const N: usize>() -> TakeConst<N> {
	TakeConst { inner: stateless::take_const() }
}

#[inline]
pub fn void<P, I, O, E>(parser: P) -> Void<P, I, O, E>
where
	I: Input,
	P: ParserStateful<I, O, E>
{
	Void { parser, __marker: PhantomData }
}

#[expect(
	clippy::type_complexity,
	reason = "good naming makes it look alright I guess lol"
)]
pub struct Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input
{
	parser_before: PBefore,
	parser: P,
	parser_after: PAfter,
	__marker: PhantomData<(
		ParserPhantom<I, OBefore, EBefore>,
		ParserPhantom<I, O, E>,
		ParserPhantom<I, OAfter, EAfter>,
		fn() -> EReal
	)>
}

impl<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> ParserStateful<I, O, EReal>
for Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: ParserStateful<I, OBefore, EBefore>,
	P: ParserStateful<I, O, E>,
	PAfter: ParserStateful<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, O, EReal> {
		let Success {
			output: _output_before,
			remaining_input: input
		} = self.parser_before.parse(input).map_err(Error::into)?;

		let Success {
			output,
			remaining_input: input
		} = self.parser.parse(input).map_err(Error::into)?;

		let Success {
			output: _output_after,
			remaining_input
		} = self.parser_after.parse(input).map_err(Error::into)?;

		Ok(Success { output, remaining_input })
	}
}

macro_rules! decl_num_struct {
	{ $($(#[$meta:meta])* $num:ident $struct_le:ident $struct_be:ident $struct_ne:ident)* } => {
		$(
			decl_num_struct! { $(#[$meta])* $num $struct_le }
			decl_num_struct! { $(#[$meta])* $num $struct_be }
			decl_num_struct! { $(#[$meta])* $num $struct_ne }
		)*
	};

	{ $(#[$meta:meta])* $num:ident $struct:ident } => {
		$(#[$meta])*
		#[repr(transparent)]
		pub struct $struct {
			inner: stateless::$struct
		}

		$(#[$meta])*
		impl<'h> ParserStateful<&'h [u8], $num> for $struct {
			#[inline]
			fn parse(&mut self, input: &'h [u8]) -> Result<&'h [u8], $num> {
				self.inner.parse(input)
			}
		}
	};
}

decl_num_struct! {
	u8 NumU8LE NumU8BE NumU8NE
	u16 NumU16LE NumU16BE NumU16NE
	u32 NumU32LE NumU32BE NumU32NE
	u64 NumU64LE NumU64BE NumU64NE
	u128 NumU128LE NumU128BE NumU128NE

	i8 NumI8LE NumI8BE NumI8NE
	i16 NumI16LE NumI16BE NumI16NE
	i32 NumI32LE NumI32BE NumI32NE
	i64 NumI64LE NumI64BE NumI64NE
	i128 NumI128LE NumI128BE NumI128NE

	f32 NumF32LE NumF32BE NumF32NE
	f64 NumF64LE NumF64BE NumF64NE

	usize NumUsizeLE NumUsizeBE NumUsizeNE
	isize NumIsizeLE NumIsizeBE NumIsizeNE
}

#[repr(transparent)]
pub struct Tag<T> {
	inner: stateless::Tag<T>
}

impl<I, T> ParserStateful<I, ()> for Tag<T>
where
	I: Input,
	T: Needle<I>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, ()> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct Take {
	inner: stateless::Take
}

impl<I> ParserStateful<I, I> for Take
where
	I: Input
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, I> {
		self.inner.parse(input)
	}
}

pub struct TakeArray<const N: usize> {
	inner: stateless::TakeArray<N>
}

impl<I, const N: usize> ParserStateful<I, I::ConstSizeOwned<N>> for TakeArray<N>
where
	I: Input
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, I::ConstSizeOwned<N>> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct TakeConst<const N: usize> {
	inner: stateless::TakeConst<N>
}

impl<I, const N: usize> ParserStateful<I, I::ConstSize<N>> for TakeConst<N>
where
	I: Input
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, I::ConstSize<N>> {
		self.inner.parse(input)
	}
}

#[repr(transparent)]
pub struct Void<P, I, O, E>
where
	I: Input
{
	parser: P,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> ParserStateful<I, (), E> for Void<P, I, O, E>
where
	I: Input,
	P: ParserStateful<I, O, E>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, (), E> {
		let Success { output: _void, remaining_input } = self.parser.parse(input)?;
		Ok(Success { output: (), remaining_input })
	}
}
