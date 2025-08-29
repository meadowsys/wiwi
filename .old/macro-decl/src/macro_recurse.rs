#[macro_export]
macro_rules! macro_recurse {
	($($stuff:tt)*) => {
		// hide potential distracting implementation details in docs
		$crate::__macro_recurse_impl! { $($stuff)* }
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __macro_recurse_impl {
	{
		// macro to call with looped
		$macro:ident
		// to pass as is
		{ $($stuff:tt)* }
		// idents to recurse
		{ $($idents:ident)* }
	} => {
		$crate::__macro_recurse_impl! {
			@impl
			$macro
			{ $($stuff)* }
			[$($idents)*] []
		}
	};

	{
		@exclude_zero

		// macro to call with looped
		$macro:ident
		// to pass as is
		{ $($stuff:tt)* }
		// idents to recurse
		{ $first:ident $($idents:ident)* }
	} => {
		$crate::__macro_recurse_impl! {
			@impl
			$macro
			{ $($stuff)* }
			[$($idents)*] [$first]
		}
	};

	{
		@impl
		$macro:ident
		{ $($stuff:tt)* }
		[$next:ident $($remaining:ident)*] [$($rest:ident)*]
	} => {
		$macro! {
			@wiwi_macro_recurse
			{ $($stuff)* }
			{ $($rest)* }
		}
		$crate::__macro_recurse_impl! {
			@impl
			$macro
			{ $($stuff)* }
			[$($remaining)*] [$($rest)* $next]
		}
	};

	{
		@impl
		$macro:ident
		{ $($stuff:tt)* }
		[] [$($rest:ident)*]
	} => {
		$macro! {
			@wiwi_macro_recurse
			{ $($stuff)* }
			{ $($rest)* }
		}
	};
}
