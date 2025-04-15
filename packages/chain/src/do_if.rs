pub trait DoIf<Wrapped>: Sized {
	type Unwrapped: Sized;

	fn run<C>(
		chain: C,
		wrapped: Wrapped,
		f: impl FnOnce(C, Self::Unwrapped) -> C
	) -> C;
}

pub enum IsTrue {}

impl DoIf<bool> for IsTrue {
	type Unwrapped = ();

	#[inline]
	fn run<C>(
		chain: C,
		wrapped: bool,
		f: impl FnOnce(C, ()) -> C
	) -> C {
		if wrapped {
			f(chain, ())
		} else {
			chain
		}
	}
}

pub enum IsSome {}

impl<T> DoIf<Option<T>> for IsSome {
	type Unwrapped = T;

	#[inline]
	fn run<C>(
		chain: C,
		wrapped: Option<T>,
		f: impl FnOnce(C, T) -> C
	) -> C {
		if let Some(unwrapped) = wrapped {
			f(chain, unwrapped)
		} else {
			chain
		}
	}
}

pub enum IsOk {}

impl<T, E> DoIf<Result<T, E>> for IsOk {
	type Unwrapped = T;

	#[inline]
	fn run<C>(
		chain: C,
		wrapped: Result<T, E>,
		f: impl FnOnce(C, T) -> C
	) -> C {
		if let Ok(unwrapped) = wrapped {
			f(chain, unwrapped)
		} else {
			chain
		}
	}
}

pub enum IsErr {}

impl<T, E> DoIf<Result<T, E>> for IsErr {
	type Unwrapped = E;

	#[inline]
	fn run<C>(
		chain: C,
		wrapped: Result<T, E>,
		f: impl FnOnce(C, E) -> C
	) -> C {
		if let Err(unwrapped) = wrapped {
			f(chain, unwrapped)
		} else {
			chain
		}
	}
}
