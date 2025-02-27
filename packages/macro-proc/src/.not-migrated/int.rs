use quote::{ format_ident, quote };
use syn::Ident;

pub fn macro_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut iter = input.into_iter();
	let proc_macro::TokenTree::Literal(input_bits) = iter.next().expect("expected int size") else {
		panic!("expected literal for the int size");
	};
	assert!(iter.next().is_none(), "expected input to consist of just the int size");

	let input_bits = input_bits.to_string().parse().expect("expected number literal for int size");
	let input_bits_literal = proc_macro2::Literal::u16_unsuffixed(input_bits);

	// default ident eg. u25
	let u_ident = format_ident!("u{input_bits}");

	// find the std int type where one single one can contain the whole int
	// used for default ident if it exists
	let min_single_int = match input_bits {
		..=8 => { Some(8usize) }
		9..=16 => { Some(16) }
		17..=32 => { Some(32) }
		33..=64 => { Some(64) }
		65..=128 => { Some(128) }
		129.. => { None }
	};

	// idents for packed
	// use the least amount per std int type to contain the whole int
	// eg. u25packed64, u25packed8
	let u_ident_packed = format_ident!("u{input_bits}packed");

	let amount_u128 = get_amount_of_bits::<128>(input_bits);
	let amount_u64 = get_amount_of_bits::<64>(input_bits);
	let amount_u32 = get_amount_of_bits::<32>(input_bits);
	let amount_u16 = get_amount_of_bits::<16>(input_bits);
	let amount_u8 = get_amount_of_bits::<8>(input_bits);

	let amounts = [
		(amount_u128, 128u16),
		(amount_u64, 64u16),
		(amount_u32, 32u16),
		(amount_u16, 16u16),
		(amount_u8, 8u16),
	];

	// find the int type where multiple of them can contain the whole int,
	// wasting less than 8 bytes (only partial byte)
	// used for default packed struct, as well as default struct (ex u25) if there doesn't
	// exist an std type that can wholly contain it
	// eg. u25packed
	let (size, bits) = amounts.into_iter()
		// .find(|(size, bits)| (size * bits) - 7 > input_bits)
		.find(|(size, bits)| input_bits + 8 > (size * bits))
		.expect("fatal error, couldn't find ideal default packing strategy");

	let default_packed = {
		let size = size as usize;
		let bits = format_ident!("u{bits}");
		quote! { [::std::primitive::#bits; #size] }
	};

	let u_default_inner = if let Some(min) = min_single_int {
		let min = format_ident!("u{min}");
		quote! { ::std::primitive::#min }
	} else {
		default_packed.clone()
	};

	let amounts_interpolaters = amounts.into_iter()
		.map(|(size, bits)| {
			let size_literal = proc_macro2::Literal::u16_unsuffixed(size);
			let size = size as usize;
			let u_ident = format_ident!("u{input_bits}with{bits}");
			let bits_literal = proc_macro2::Literal::u16_unsuffixed(bits);
			let bits = format_ident!("u{bits}");
			let s = proc_macro2::Literal::string(if size == 1 { "" } else { "s" });

			quote! {
				#[doc = concat!(
					stringify!(#input_bits_literal),
					"-bit unsigned integer, using ",
					stringify!(#size_literal),
					" ",
					stringify!(#bits_literal),
					"-bit integer",
					#s
				)]
				#[allow(non_camel_case_types)]
				pub struct #u_ident {
					inner: [::std::primitive::#bits; #size]
				}
			}
		});

	let default_size_literal = proc_macro2::Literal::u16_unsuffixed(size);
	let default_bits_literal = proc_macro2::Literal::u16_unsuffixed(bits);
	let default_s = proc_macro2::Literal::string(if size == 1 { "" } else { "s" });
	let mut out = quote! {
		#[doc = concat!(
			stringify!(#input_bits_literal),
			"-bit unsigned integer"
		)]
		#[allow(non_camel_case_types)]
		pub struct #u_ident {
			inner: #u_default_inner
		}

		// #[doc = concat!(
		// 	stringify!(#input_bits_literal),
		// 	"-bit unsigned integer, packed into ",
		// 	stringify!(#default_size_literal),
		// 	" ",
		// 	stringify!(#default_bits_literal),
		// 	"-bit integer",
		// 	#default_s
		// )]
		// #[allow(non_camel_case_types)]
		// pub struct #u_ident_packed {
		// 	inner: #default_packed
		// }

		// #(#amounts_interpolaters)*
	};

	out.into()
}

fn get_amount_of_bits<const INNER_BITS: u16>(bits: u16) -> u16 {
	let full = bits / INNER_BITS;
	let remainder = (bits % INNER_BITS) != 0;
	full + (remainder as u16)
}
