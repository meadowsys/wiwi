// use proc_macro2::{ Group, Ident, Spacing, Span, TokenStream, TokenTree };
use proc_macro2::{ Spacing, Span, TokenStream, TokenTree };
// use quote::quote;
use syn::Error;
// use syn::parse::Parse;

pub fn with_cloned(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut iter = input.into_iter();

	loop {
		match iter.next() {
			Some(TokenTree::Ident(ident)) => {
				match &*ident.to_string() {
					"_" => {
						match iter.next() {
							Some(TokenTree::Punct(punct)) if punct.as_char() == '=' && punct.spacing() == Spacing::Joint => { /* ok */ }
							Some(tt) => {
								return Err(Error::new_spanned(tt, "unexpected token").into_compile_error())
							}
							None => {
								return Err(Error::new(Span::call_site(), "unexpected end of macro input").into_compile_error())
							}
						}
						match iter.next() {
							Some(TokenTree::Punct(punct)) if punct.as_char() == '>' && punct.spacing() == Spacing::Alone => { /* ok */ }
							Some(tt) => {
								return Err(Error::new_spanned(tt, "unexpected token").into_compile_error())
							}
							None => {
								return Err(Error::new(Span::call_site(), "unexpected end of macro input").into_compile_error())
							}
						}
					}
					"mut" => {
					}
					_ident => {}
				}
				// return Err(Error::new_spanned(ident, &*hh).into_compile_error())
			}

			Some(TokenTree::Group(_group)) => {}

			Some(TokenTree::Punct(_punct)) => {
				// return Error::new_spanned(
				// 	punct,
				// 	"unexpected token"
				// ).into_compile_error()
			}

			Some(TokenTree::Literal(_literal)) => {
				// return Err(Error::new_spanned(
				// 	literal,
				// 	"unexpected literal"
				// ).into_compile_error())
			}

			None => {
				return Err(Error::new(
					Span::call_site(),
					"unexpected end of macro invocation"
				).into_compile_error())
			}
		}
	}
}
