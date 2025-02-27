use crate::macro_util::macro_recurse;

pub trait Tuple {}

macro_rules! impl_tuple {
	($($idents:ident)*) => {
		macro_recurse!(impl_tuple {} { $($idents)* });
	};

	(@wiwi_macro_recurse {} { $($idents:ident)* }) => {
		impl<$($idents),*> Tuple for ($($idents,)*) {}
	}
}

impl_tuple! {
	A1 A2 A3 A4
	A5 A6 A7 A8
	A9 A10 A11 A12
	A13 A14 A15 A16
}
// // pub trait Flatten<Tuple> {}
