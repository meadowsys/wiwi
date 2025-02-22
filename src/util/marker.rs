use sealed::Sealed;

/// Marker trait for... markers
pub trait TypeMarker: Sealed {}

impl TypeMarker for () {}
impl Sealed for () {}

macro_rules! gen_markers {
	{
		$(
			$(#[$meta:meta])*
			$($descriptive_words:literal)? $marker:ident
		)*
	} => {
		$(
			$(
				#[doc = concat!(
					"Marker for ",
					$descriptive_words,
					" types"
				)]
				#[doc = ""]
			)?
			$(#[$meta])*
			pub struct $marker {
				__private: ()
			}

			impl TypeMarker for $marker {}
			impl Sealed for $marker {}
		)*
	}
}

gen_markers! {
	"string"
	StringMarker

	"number"
	NumberMarker

	"bigint"
	BigIntMarker

	"boolean"
	BooleanMarker

	"symbol"
	SymbolMarker

	"undefined"
	UndefinedMarker

	"null"
	NullMarker
}

mod sealed {
	pub trait Sealed {}
}
