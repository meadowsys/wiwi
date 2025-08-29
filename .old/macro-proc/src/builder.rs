use proc_macro2::{ Ident, Span, TokenStream, TokenTree };
use quote::{ format_ident, quote };
use std::mem::take;
// use syn::{ Attribute, Expr, Item, ItemFn, ItemStruct, Meta, MetaNameValue };
use syn::Item;

pub fn builder(_attr: TokenStream, _item: Item) -> TokenStream {
	todo!()
}

pub fn __builder_internal_helper_gen_state(input: TokenStream) -> TokenStream {
	struct Field {
		field: Ident,
		field_meta: Vec<TokenStream>,
		init: Ident,
		init_meta: Vec<TokenStream>,
		init_with: Ident,
		init_with_meta: Vec<TokenStream>,
	}

	impl Default for Field {
		fn default() -> Self {
			Field {
				// grrr
				field: Ident::new("a", Span::call_site()),
				field_meta: Default::default(),
				// grrr
				init: Ident::new("a", Span::call_site()),
				init_meta: Default::default(),
				// grrr
				init_with: Ident::new("a", Span::call_site()),
				init_with_meta: Default::default()
			}
		}
	}

	let mut everything = Vec::new();
	let mut current = Field::default();

	let mut input = input.into_iter();
	loop {
		let Some(TokenTree::Ident(ident)) = input.next() else { break };

		let vec = match &*ident.to_string() {
			"__field_meta" => { &mut current.field_meta }
			"__init_meta" => { &mut current.init_meta }
			"__init_with_meta" => { &mut current.init_with_meta }
			_ => {
				current.init = format_ident!("{ident}Init");
				current.init_with = format_ident!("{ident}InitWith");
				current.field = ident;
				everything.push(take(&mut current));

				continue
			}
		};

		let Some(TokenTree::Group(group)) = input.next() else { break };
		vec.push(group.stream());
	}

	let (fields, decl_items, impl_trait_bounds, impl_items) = everything.iter()
		.enumerate()
		.map(|(before_fields, field)| {
			let Field {
				field,
				field_meta,
				init,
				init_meta,
				init_with,
				init_with_meta
			} = field;

			let state_before = everything[..before_fields].iter().map(|f| &f.field).collect::<Vec<_>>();
			let state_before = &*state_before;

			let state_after = everything[before_fields + 1..].iter().map(|f| &f.field).collect::<Vec<_>>();
			let state_after = &*state_after;

			let decl_items = quote! {
				#(#field_meta)*
				type #field: InitStatus;

				#(#init_meta)*
				type #init: State;

				#(#init_with_meta)*
				type #init_with<T: ?Sized>: State;
			};

			let impl_trait_bound = quote!(#field: InitStatus);

			let impl_items = quote! {
				type #field = #field;
				type #init = StateContainer<
					#(#state_before,)*
					Init,
					#(#state_after),*
				>;
				type #init_with<T: ?Sized> = StateContainer<
					#(#state_before,)*
					Init<T>,
					#(#state_after),*
				>;
			};

			(
				field,
				decl_items,
				impl_trait_bound,
				impl_items
			)
		})
		.collect::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

	let uninit_items = (0..everything.len()).map(|_| quote!(Uninit));

	quote! {
		pub trait State {
			#(#decl_items)*
		}

		pub struct StateContainer<#(#fields),*> {
			__marker: PhantomData<fn(#(#fields),*) -> (#(#fields),*)>
		}

		pub type StateUninit = StateContainer<
			#(#uninit_items),*
		>;

		impl<#(#impl_trait_bounds),*> State for StateContainer<#(#fields),*> {
			#(#impl_items)*
		}
	}
}
