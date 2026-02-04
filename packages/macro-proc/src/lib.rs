use proc_macro::TokenStream;
use proc_macro2::{ Span, TokenTree };
use quote::{
	ToTokens,
	format_ident,
	quote,
	quote_spanned
};
use syn::{
	Attribute,
	FnArg,
	ImplItem,
	ItemImpl,
	Meta,
	Pat,
	PatIdent,
	PatType,
	Path,
	Receiver,
	ReturnType,
	Signature,
	Token,
	TraitItemFn,
	Type,
	TypePath,
	parse2,
	parse_macro_input
};
use syn::spanned::Spanned as _;

#[proc_macro_attribute]
pub fn chain_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
	let _ = attr;

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
		self_ty: _,
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
			mut attrs,
			mut sig,
			default,
			semi_token
		} = parse_macro_input!(parsed_item);

		compile_error_if_some!(default, "custom chain fn implementations are not supported in chain API");

		let Signature {
			constness: _,
			asyncness,
			unsafety,
			abi,
			fn_token: _,
			ident,
			generics: _,
			paren_token: _,
			inputs,
			variadic,
			output
		} = &mut sig;

		compile_error_if_some!(variadic, "C variadic functions are not supported in chain API");

		let asyncness = match asyncness {
			Some(asyncness) => {
				quote_spanned! { asyncness.span() => .await }
			}
			None => { quote! {} }
		};

		let arg_names = inputs.iter()
			.cloned()
			.filter_map(|arg| match arg {
				FnArg::Typed(arg) => { Some(arg) }
				FnArg::Receiver(_) => { None }
			})
			.filter_map(|arg| match *arg.pat {
				Pat::Ident(pat) => { Some(pat.ident) }
				pat => {
					errors.push(error(pat, "this pat type is currently unsupported??"));
					None
				}
			})
			.collect::<Vec<_>>();
		return_errors!();

		// relying on the fact that rust syntax errors itself when any arg other
		// than the first one is a "receiver"
		let self_param = inputs.first_mut()
			.and_then(|arg| match arg {
				FnArg::Receiver(receiver) => { Some(receiver) }
				FnArg::Typed(_) => { None }
			}).and_then(|receiver| match receiver {
				// `self: Ty`
				Receiver { colon_token: Some(_), .. } => {
					errors.push(error(receiver, "explicit receiver type is not supported in chain API"));
					None
				}

				// `mut self`
				Receiver {
					reference: None,
					mutability: Some(mutability),
					..
				} => {
					errors.push(error(mutability, "explicit mutability not allowed"));
					None
				}

				// `self`
				Receiver {
					reference: None,
					mutability: None,
					self_token,
					..
				} => {
					Some((
						SelfParam::Owned,
						quote_spanned! { self_token.span() => self.into_inner() },
						receiver
					))
				}

				// `&self`
				Receiver {
					reference: Some(_),
					mutability: None,
					self_token,
					..
				} => {
					Some((
						SelfParam::Ref,
						quote_spanned! { self_token.span() => self.as_inner() },
						receiver
					))
				}

				// `&mut self`
				Receiver {
					reference: Some(_),
					mutability: Some(_),
					self_token,
					..
				} => {
					Some((
						SelfParam::Mut,
						quote_spanned! { self_token.span() => self.as_inner_mut() },
						receiver
					))
				}
			});
		return_errors!();

		let (self_param_ty, self_arg, mut self_param) = self_param
			.map(|(self_param_ty, self_arg, self_param)| (
				self_param_ty,
				quote! { #self_arg, },
				Some(self_param)
			))
			.unwrap_or_else(|| (
				SelfParam::None,
				quote! {},
				None
			));

		let mut needs_output_arg = false;
		let output_orig = output.clone();
		match (&mut *output, &self_param_ty) {
			(ReturnType::Default, SelfParam::None | SelfParam::Owned) => {
				*output = ReturnType::Type(
					Token![->](semi_token.span()),
					Box::new(Type::Verbatim(quote_spanned! { semi_token.span() =>
						crate::Chain<()>
					}))
				);
			}

			(ReturnType::Type(_, ty), SelfParam::None | SelfParam::Owned) => {
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

			(ReturnType::Default, SelfParam::Ref | SelfParam::Mut) => {
				*output = ReturnType::Type(
					Token![->](semi_token.span()),
					Box::new(Type::Verbatim(quote! { Self }))
				);

				let self_param = self_param.as_mut().unwrap();
				self_param.reference = None;
				*self_param.ty = Type::Verbatim(quote! { Self });
			}

			(ReturnType::Type(..), SelfParam::Ref | SelfParam::Mut) => {
				*output = ReturnType::Type(
					Token![->](semi_token.span()),
					Box::new(Type::Verbatim(quote! { Self }))
				);

				let self_param = self_param.as_mut().unwrap();
				self_param.reference = None;
				*self_param.ty = Type::Verbatim(quote! { Self });

				needs_output_arg = true;
			}
		};

		let mut inner_fn_call = quote_spanned! { semi_token.span() =>
			<<Self as crate::chain::ChainInnerType>::Inner>::#ident(
				#self_arg
				#(#arg_names),*
			)
			#asyncness
		};

		if let Some(unsafety) = unsafety {
			inner_fn_call = quote_spanned! { unsafety.span() =>
				unsafe { #inner_fn_call }
			}
		}

		let (fn_call, output_fn_call) = if matches!(self_param_ty, SelfParam::Ref | SelfParam::Mut) {
			(
				quote! {
					let _ = #inner_fn_call;
					self
				},
				needs_output_arg.then(|| quote! {
					chain_output.write(#inner_fn_call);
					self
				})
			)
		} else {
			(
				quote_spanned! { semi_token.span() =>
					crate::Chain::from_inner(#inner_fn_call)
				},
				None
			)
		};

		*abi = None;

		handle_item_doc_attrs(&mut attrs, &mut errors);
		return_errors!();

		*item = quote! {
			#(#attrs)*
			#[allow(
				unknown_lints,
				clippy::needless_arbitrary_self_type,
				clippy::undocumented_unsafe_blocks,
				reason = "macro output"
			)]
			#sig {
				#fn_call
			}
		};

		if needs_output_arg {
			let sig_output = match output_orig {
				ReturnType::Type(_, ty) => { *ty }
				ReturnType::Default => { unreachable!() }
			};

			sig.ident = format_ident!("{}_output", sig.ident);
			sig.inputs.push(FnArg::Typed(PatType {
				attrs: Vec::new(),
				pat: Box::new(Pat::Ident(PatIdent {
					attrs: Vec::new(),
					by_ref: None,
					mutability: None,
					ident: format_ident!("chain_output"),
					subpat: None
				})),
				colon_token: Token![:](Span::call_site()),
				ty: Box::new(Type::Verbatim(quote! {
					impl Output<#sig_output>
				}))
			}));

			*item = quote! {
				#item

				#(#attrs)*
				#[allow(
					unknown_lints,
					clippy::needless_arbitrary_self_type,
					clippy::undocumented_unsafe_blocks,
					reason = "macro output"
				)]
				#sig {
					#output_fn_call
				}
			};
		}
	}

	item.into_token_stream().into()
}

enum SelfParam {
	Owned,
	Ref,
	Mut,
	None
}

fn handle_item_doc_attrs(attrs: &mut [Attribute], errors: &mut Vec<syn::Error>) {
	for attr in attrs {
		let Meta::List(meta) = &attr.meta else { continue };
		let Some(ident) = meta.path.get_ident() else { continue };
		if ident != "chain_doc" { continue };

		let mut tokens = meta.tokens.clone().into_iter();
		let Some(TokenTree::Literal(display)) = tokens.next() else { continue };
		let link = match (tokens.next(), tokens.next()) {
			(Some(TokenTree::Punct(punct)), Some(TokenTree::Literal(link))) if punct.as_char() == ',' => {
				Some(link)
			}
			(Some(TokenTree::Punct(punct)), None) if punct.as_char() == ',' => {
				errors.push(error(punct, "*eats your trailing comma cutely*"));
				continue
			}
			(Some(token), _) => {
				errors.push(error(token, "invalid syntax, expected `,`"));
				continue
			}
			(None, None) => { None }
			(None, Some(_)) => { unreachable!("proc_macro2 has some serious bugs") }
		};

		match tokens.next() {
			None => {
				// ok
			}
			Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => {
				errors.push(error(punct, "*eats your trailing comma cutely*"));
				continue
			}
			Some(token) => {
				errors.push(error(token, "invalid syntax, expected `,`"));
				continue
			}
		}

		let link = if let Some(link) = link {
			quote! { "(", #link, ")", }
		} else {
			quote! {}
		};

		*attr = Attribute {
			pound_token: attr.pound_token,
			style: attr.style,
			bracket_token: attr.bracket_token,
			meta: parse2(quote! {
				doc = concat!(
					"See documentation for [`",
					#display,
					"`]",
					#link
					" for details on the underlying function"
				)
			}).unwrap()
		};
	}
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
