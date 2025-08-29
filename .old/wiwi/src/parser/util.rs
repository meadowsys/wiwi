use crate::prelude::*;
use super::{ stateful, stateless, Input, ParserPhantom, Result, Success };

/// Wraps an implementor of [`Parser`](stateless::Parser) and provides an implementation
/// of [`ParserStateful`](stateful::ParserStateful)
#[inline]
pub fn adapt_stateful<P, I, O, E>(parser: P) -> AdaptStateful<P, I, O, E>
where
	I: Input,
	P: stateless::Parser<I, O, E>
{
	AdaptStateful { parser, __marker: PhantomData }
}

#[inline]
pub(super) fn map<P, F, O>(parser: P, map: F) -> Map<P, F, O> {
	Map { parser, map, __marker: PhantomData }
}

#[repr(transparent)]
pub struct AdaptStateful<P, I, O, E>
where
	I: Input
{
	parser: P,
	__marker: ParserPhantom<I, O, E>
}

impl<P, I, O, E> stateful::ParserStateful<I, O, E> for AdaptStateful<P, I, O, E>
where
	I: Input,
	P: stateless::Parser<I, O, E>
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, O, E> {
		self.parser.parse(input)
	}
}

pub struct Map<P, F, O> {
	parser: P,
	map: F,
	__marker: PhantomData<fn(O) -> O>
}

impl<P, I, O, E, F, O2> stateless::Parser<I, O2, E> for Map<P, F, O>
where
	I: Input,
	P: stateless::Parser<I, O, E>,
	F: Fn(O) -> O2
{
	#[inline]
	fn parse(&self, input: I) -> Result<I, O2, E> {
		self.parser.parse(input).map(|Success { output, remaining_input }| {
			Success { output: (self.map)(output), remaining_input }
		})
	}
}

impl<P, I, O, E, F, O2> stateful::ParserStateful<I, O2, E> for Map<P, F, O>
where
	I: Input,
	P: stateful::ParserStateful<I, O, E>,
	F: FnMut(O) -> O2
{
	#[inline]
	fn parse(&mut self, input: I) -> Result<I, O2, E> {
		self.parser.parse(input).map(|Success { output, remaining_input }| {
			Success { output: (self.map)(output), remaining_input }
		})
	}
}
