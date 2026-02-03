use proc_macro::TokenStream;
use quote::{ ToTokens, quote };
use syn::{ FnArg, ImplItem, ImplItemFn, ItemImpl, Pat, Path, ReturnType, Signature, Token, TraitItemFn, Type, TypePath, parse_macro_input };
use syn::spanned::Spanned as _;

#[proc_macro_attribute]
pub fn chain_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
	let _ = dbg!(attr);

	let mut errors = Vec::new();
	macro_rules! return_errors {
		() => {
			if !errors.is_empty() {
				return compile_error_all(errors)
			}
		}
	}

	let mut item = parse_macro_input!(item);
	let ItemImpl {
		attrs: _,
		defaultness,
		unsafety,
		impl_token: _,
		generics: _,
		trait_,
		self_ty,
		brace_token: _,
		items
	} = &mut item;

	compile_error_if_some!(defaultness, "chain impls cannot be default");
	compile_error_if_some!(unsafety, "chain impls shouldn't need to be unsafe");
	compile_error_if_some!(trait_.as_ref().map(|(_, path, _)| path), "chain impls aren't trait impls");

	for item in &mut **items {
		match item {
			ImplItem::Verbatim(_) => {}
			item => {
				errors.push(error(item, "items that aren't function stubs currently not supported in chain API"))
			}
		}
	}
	return_errors!();

	// (currently) the function stubs that we need without a body aren't
	// parsed and only show up in Verbatim, so we use these then parse
	// TraitItemFn manually after
	let items_filtered = items.iter_mut()
		.filter_map(|item| match item {
			ImplItem::Verbatim(item) => { Some(item) }
			_ => { None }
		});

	for item in items_filtered {
		let parsed_item = item.clone().into();
		let TraitItemFn {
			attrs,
			mut sig,
			default,
			semi_token
		} = parse_macro_input!(parsed_item);

		compile_error_if_some!(default, "chain impls cannot be default");

		let Signature {
			constness,
			asyncness,
			unsafety,
			abi,
			fn_token,
			ident,
			generics,
			paren_token,
			inputs,
			variadic,
			output
		} = &mut sig;

		// // todo this was supposed to be for Option<block> being some i think
		// compile_error_if_some!(defaultness, "custom chain fn implementations are not supported in chain API");
		// compile_error_if_some!(abi, "non default ABIs are not supported in chain API");
		// compile_error_if_some!(variadic, "C variadic functions are not supported in chain API");
		// compile_error_if_some!(
		// 	inputs.first().filter(|arg| matches!(arg, FnArg::Receiver(_))),
		// 	"todo allow method receiver"
		// );

		// let uses_self = inputs.first().filter(|arg| matches!(arg, FnArg::Receiver(_)));

		// let arg_names = inputs.iter()
		// 	.filter_map(|arg| match arg {
		// 		FnArg::Typed(arg) => { Some(arg) }
		// 		FnArg::Receiver(_) => { None }
		// 	})
		// 	.filter_map(|arg| match &*arg.pat {
		// 		Pat::Ident(pat) => { Some(&pat.ident) }
		// 		pat => {
		// 			errors.push(error(pat, "this pat type is currently unsupported??"));
		// 			None
		// 		}
		// 	})
		// 	.collect::<Vec<_>>();

		// return_errors!();

		// quote::quote! {
		// 	#(#attrs)*
		// 	#sig {
		// 		use std::io;
		// 	}
		// }.into()

		match output {
			ReturnType::Default => {
				*output = ReturnType::Type(
					Token![->](semi_token.span()),
					Box::new(Type::Verbatim(quote! {
						crate::Chain<()>
					}))
				);
			}

			ReturnType::Type(_, ty) => {
				match &**ty {
					Type::Path(TypePath {
						qself: None,
						path: Path {
							leading_colon: None,
							segments
						}
					}) if
						segments.len() == 1 &&
						segments.first().unwrap().ident == "Self"
					=> {
						// .3
						// i hated this
					}
					ty => {
						*output = ReturnType::Type(
							Token![->](semi_token.span()),
							Box::new(Type::Verbatim(quote! {
								crate::Chain<#ty>
							}))
						);
					}
				}
			}
		};

		let fn_call = quote! {
			let inner = <<Self as crate::chain::ChainInnerType>::Inner>::#ident();
			crate::Chain::from_inner(inner)
		};

		*item = quote! {
			#(#attrs)*
			#sig {
				#fn_call
			}
		};
		// *item = "".parse().unwrap();
	}

	item.into_token_stream().into()
}

fn error(tokens: impl ToTokens, msg: &str) -> syn::Error {
	syn::Error::new_spanned(tokens, msg)
}

fn compile_error(tokens: impl ToTokens, msg: &str) -> TokenStream {
	error(tokens, msg)
		.into_compile_error()
		.into()
}

fn compile_error_all(errors: impl IntoIterator<Item = syn::Error>) -> TokenStream {
	let mut iter = errors.into_iter();

	let Some(mut error) = iter.next() else {
		return TokenStream::new();
	};

	error.extend(iter);
	error.into_compile_error().into()
}

macro_rules! compile_error_if_some {
	($tokens:expr, $msg:expr) => {
		if let Some(tokens) = $tokens {
			return compile_error(tokens, $msg)
		}
	}
}
use compile_error_if_some;
