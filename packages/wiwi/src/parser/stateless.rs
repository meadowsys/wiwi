use crate::prelude::*;
use crate::num::*;
use super::{ max_init_cap, util, Error, Input, Needle, ParserPhantom, Result, Success };
use std::num::NonZero;

pub trait Parser<I, O, E = ()>
where
	I: Input
{
	fn parse(&self, input: I) -> Result<I, O, E>;

	#[inline]
	fn map<F, O2>(self, f: F) -> util::Map<Self, F, O>
	where
		Self: Sized,
		F: Fn(O) -> O2
	{
		util::map(self, f)
	}
}

impl<F, I, O, E> Parser<I, O, E> for F
where
	I: Input,
	F: Fn(I) -> Result<I, O, E>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, O, E> {
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
	PBefore: Parser<I, OBefore, EBefore>,
	P: Parser<I, O, E>,
	PAfter: Parser<I, OAfter, EAfter>,
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
				let inner = Number { inner: take_array(), __marker: PhantomData };
				$struct { inner }
			}
		)*
	};
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
pub fn repeat<P, I, O, E>(parser: P, count: usize) -> Repeat<P, I, O, E>
where
	I: Clone + Input,
	P: Parser<I, O, E>
{
	Repeat { parser, count, __marker: PhantomData }
}

#[inline]
pub fn spin<P, I, O, E>(parser: P) -> Spin<P, I, O, E>
where
	I: Clone + Input,
	P: Parser<I, O, E>
{
	Spin { parser, __marker: PhantomData }
}

#[inline]
pub fn tag<T>(tag: T) -> Tag<T> {
	Tag { tag }
}

#[inline]
pub fn take<N>(amount: N) -> Take
where
	N: IntoUsize
{
	Take { amount: amount.into_usize() }
}

#[inline]
pub fn take_array<const N: usize>() -> TakeArray<N> {
	TakeArray { __private: () }
}

#[inline]
pub fn take_const<const N: usize>() -> TakeConst<N> {
	TakeConst { __private: () }
}

#[inline]
pub fn void<P, I, O, E>(parser: P) -> Void<P, I, O, E>
where
	I: Input,
	P: Parser<I, O, E>
{
	Void { parser, __marker: PhantomData }
}

// #[repr(transparent)]
// pub struct ApplyMany<P, I, O, E>
// where
// 	I: Input
// {
// 	parser: P,
// 	__marker: ParserPhantom<I, O, E>
// }
//
// impl<P, I, O, E> Parser<I, Vec<O>, E> for ApplyMany<P, I, O, E>
// where
// 	I: Clone + Input,
// 	P: Parser<I, O, E>
// {
// 	#[inline]
// 	fn parse(&self, input: I) -> Result<I, Vec<O>, E> {
// 		let mut out = Vec::new();
// 		let mut remaining = input;
//
// 		loop {
// 			// todo: i dont like this
// 			match self.parser.parse(remaining.clone()) {
// 				Ok(Success { output, remaining_input }) => {
// 					out.push(output);
// 					remaining = remaining_input;
// 				}
// 				Err(super::Error::NotEnoughData { .. }) => {
// 					return Ok(Success { output: out, remaining_input: remaining })
// 				}
// 				Err(e) => { return Err(e) }
// 			}
// 		}
// 	}
// }

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

impl<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter> Parser<I, O, EReal>
for Delimited<P, I, O, E, EReal, PBefore, OBefore, EBefore, PAfter, OAfter, EAfter>
where
	I: Input,
	PBefore: Parser<I, OBefore, EBefore>,
	P: Parser<I, O, E>,
	PAfter: Parser<I, OAfter, EAfter>,
	EBefore: Into<EReal>,
	E: Into<EReal>,
	EAfter: Into<EReal>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, O, EReal> {
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

#[repr(transparent)]
struct Number<Num, E, const N: usize>
where
	Num: ArrayConversions<N>,
	E: Endian<Num, N>
{
	inner: TakeArray<N>,
	__marker: PhantomData<(fn() -> Num, E)>
}

impl<'h, Num, E, const N: usize> Parser<&'h [u8], Num> for Number<Num, E, N>
where
	Num: ArrayConversions<N>,
	E: Endian<Num, N>
{
	#[inline]
	fn parse(&self, input: &'h [u8]) -> Result<&'h [u8], Num> {
		self.inner.parse(input).map(|Success { output, remaining_input }| Success {
			output: E::from_bytes(output),
			remaining_input
		})
	}
}

macro_rules! decl_num_struct {
	{ $($(#[$meta:meta])* $num:ident $n:literal $struct_le:ident $struct_be:ident $struct_ne:ident )* } => {
		$(
			decl_num_struct! { @impl $(#[$meta])* $num EndianLittle $n $struct_le }
			decl_num_struct! { @impl $(#[$meta])* $num EndianBig $n $struct_be }
			decl_num_struct! { @impl $(#[$meta])* $num EndianNative $n $struct_ne }
		)*
	};

	{ @impl $(#[$meta:meta])* $num:ident $endian:ident $n:literal $struct:ident } => {
		$(#[$meta])*
		#[repr(transparent)]
		pub struct $struct {
			inner: Number<$num, $endian, $n>
		}

		$(#[$meta])*
		impl<'h> Parser<&'h [u8], $num> for $struct {
			#[inline]
			fn parse(&self, input: &'h [u8]) -> Result<&'h [u8], $num> {
				self.inner.parse(input)
			}
		}
	};
}

decl_num_struct! {
	u8 1 NumU8LE NumU8BE NumU8NE
	u16 2 NumU16LE NumU16BE NumU16NE
	u32 4 NumU32LE NumU32BE NumU32NE
	u64 8 NumU64LE NumU64BE NumU64NE
	u128 16 NumU128LE NumU128BE NumU128NE

	i8 1 NumI8LE NumI8BE NumI8NE
	i16 2 NumI16LE NumI16BE NumI16NE
	i32 4 NumI32LE NumI32BE NumI32NE
	i64 8 NumI64LE NumI64BE NumI64NE
	i128 16 NumI128LE NumI128BE NumI128NE

	f32 4 NumF32LE NumF32BE NumF32NE
	f64 8 NumF64LE NumF64BE NumF64NE

	#[cfg(target_pointer_width = "16")]
	usize 2 NumUsizeLE NumUsizeBE NumUsizeNE
	#[cfg(target_pointer_width = "16")]
	isize 2 NumIsizeLE NumIsizeBE NumIsizeNE

	#[cfg(target_pointer_width = "32")]
	usize 4 NumUsizeLE NumUsizeBE NumUsizeNE
	#[cfg(target_pointer_width = "32")]
	isize 4 NumIsizeLE NumIsizeBE NumIsizeNE

	#[cfg(target_pointer_width = "64")]
	usize 8 NumUsizeLE NumUsizeBE NumUsizeNE
	#[cfg(target_pointer_width = "64")]
	isize 8 NumIsizeLE NumIsizeBE NumIsizeNE
}

pub struct Repeat<P, I, O, E>
where
	I: Input
{
	parser: P,
	count: usize,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> Parser<I, Vec<O>, E> for Repeat<P, I, O, E>
where
	I: Clone + Input,
	P: Parser<I, O, E>
{
	#[inline]
	fn parse(&self, mut input: I) -> Result<I, Vec<O>, E> {
		let mut out = Vec::with_capacity(usize::max(const { max_init_cap::<O>() }, self.count));

		for _ in 0..self.count {
			let Success { output, remaining_input } = self.parser.parse(input.clone())?;
			out.push(output);
			input = remaining_input;
		}

		Ok(Success { output: out, remaining_input: input })
	}
}

pub struct Spin<P, I, O, E>
where
	I: Input
{
	parser: P,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> Parser<I, Vec<O>, E> for Spin<P, I, O, E>
where
	I: Clone + Input,
	P: Parser<I, O, E>
{
	#[inline]
	fn parse(&self, mut input: I) -> Result<I, Vec<O>, E> {
		let mut out = Vec::new();

		loop {
			match self.parser.parse(input.clone()) {
				Ok(Success { output, remaining_input }) => {
					out.push(output);
					input = remaining_input;
				}
				Err(Error::NotEnoughData { missing: _ }) => {
					return Ok(Success { output: out, remaining_input: input })
				}
				Err(e) => { return Err(e) }
			}
		}
	}
}

#[repr(transparent)]
pub struct Tag<T> {
	tag: T
}

impl<I, T> Parser<I, ()> for Tag<T>
where
	I: Input,
	T: Needle<I>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, ()> {
		let true = input.len() >= self.tag.len() else {
			return Err(Error::NotEnoughData {
				missing: NonZero::new(self.tag.len() - input.len())
			})
		};

		input.starts_with(&self.tag)
			.then(|| input.take_first(self.tag.len()))
			.flatten()
			.map(|(_, remaining_input)| Success { output: (), remaining_input })
			.ok_or_else(|| Error::Error { error: () })
	}
}

#[repr(transparent)]
pub struct Take {
	amount: usize
}

impl<I> Parser<I, I> for Take
where
	I: Input
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, I> {
		input.take_first(self.amount)
			.map(|(output, remaining_input)| Success { output, remaining_input })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(self.amount - input.len())
			})
	}
}

#[repr(transparent)]
pub struct TakeArray<const N: usize> {
	__private: ()
}

impl<I, const N: usize> Parser<I, I::ConstSizeOwned<N>> for TakeArray<N>
where
	I: Input
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, I::ConstSizeOwned<N>> {
		input.take_first_const_owned()
			.map(|(output, remaining_input)| Success { output, remaining_input })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(N - input.len())
			})
	}
}

#[repr(transparent)]
pub struct TakeConst<const N: usize> {
	__private: ()
}

impl<I, const N: usize> Parser<I, I::ConstSize<N>> for TakeConst<N>
where
	I: Input
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, I::ConstSize<N>> {
		input.take_first_const()
			.map(|(output, remaining_input)| Success { output, remaining_input })
			.ok_or_else(|| Error::NotEnoughData {
				missing: NonZero::new(N - input.len())
			})
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

impl<P, I, O, E> Parser<I, (), E> for Void<P, I, O, E>
where
	I: Input,
	P: Parser<I, O, E>
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, (), E> {
		let Success { output: _void, remaining_input } = self.parser.parse(input)?;
		Ok(Success { output: (), remaining_input })
	}
}
