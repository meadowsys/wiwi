// 1. this is just a script, efficiency doesn't matter that much
// 2. keeping everything consistently `push_str` and (double quoted) string literals is nicer
#![allow(clippy::single_char_add_str)]

fn main() {
	let decl_args_inner = "($inner:ident $($args:tt)*)";
	let decl_args_self = "($self:ident $($args:tt)*)";

	let impl_fn_header_safe = "pub fn";
	let impl_fn_header_unsafe = "pub unsafe fn";

	let impl_param_list_inner = "(mut self $($args)*)";
	let impl_param_list_self = "($self $($args)*)";
	let impl_param_list_self_mut = "(mut $self $($args)*)";

	let impl_body_inner = &[
		"let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);",
		"$crate::prelude::identity::<()>($body);",
		"self"
	];
	let impl_body_self = &[
		"$crate::prelude::identity::<()>($body);",
		"$self"
	];
	let impl_body_move = &[
		"let mut $inner = <Self as $crate::chain::Chain>::into_inner(self);",
		"<Self as $crate::chain::Chain>::from_inner($body)"
	];
	let impl_body_move_self = &[
		"$self"
	];
	let impl_body_void = &[
		"let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);",
		"let _ = $body;",
		"self"
	];
	let impl_body_void_self = &[
		"let _ = $body",
		"$self"
	];

	let cases = [
		Case {
			decl_keywords: "",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_inner,
			impl_body: impl_body_inner
		},
		Case {
			decl_keywords: "self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_self_mut,
			impl_body: impl_body_self
		},
		Case {
			decl_keywords: "unsafe",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_inner,
			impl_body: impl_body_inner
		},
		Case {
			decl_keywords: "unsafe self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_self_mut,
			impl_body: impl_body_self
		},
		Case {
			decl_keywords: "move",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_self,
			impl_body: impl_body_move
		},
		Case {
			decl_keywords: "move self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_self,
			impl_body: impl_body_move_self
		},
		Case {
			decl_keywords: "unsafe move",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_self,
			impl_body: impl_body_move
		},
		Case {
			decl_keywords: "unsafe move self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_self,
			impl_body: impl_body_move_self
		},
		Case {
			decl_keywords: "void",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_inner,
			impl_body: impl_body_void
		},
		Case {
			decl_keywords: "void self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_safe,
			impl_param_list: impl_param_list_self_mut,
			impl_body: impl_body_void_self
		},
		Case {
			decl_keywords: "unsafe void",
			decl_args: decl_args_inner,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_inner,
			impl_body: impl_body_void
		},
		Case {
			decl_keywords: "unsafe void self",
			decl_args: decl_args_self,
			impl_fn_header: impl_fn_header_unsafe,
			impl_param_list: impl_param_list_self_mut,
			impl_body: impl_body_void_self
		},
	];

	let cases = cases.iter()
		.map(process_case)
		.collect::<Vec<_>>();

	let [first, rest @ ..] = &*cases else { unreachable!() };
	println!("macro_rules! chain_fn {{");
	print!("{first}");
	rest.iter().for_each(|case| print!("\n\n{case}"));
	println!("\n}}");
}

struct Case {
	decl_keywords: &'static str,
	decl_args: &'static str,
	impl_fn_header: &'static str,
	impl_param_list: &'static str,
	impl_body: &'static [&'static str]
}

fn process_case(case: &Case) -> String {
	let mut output = String::new();

	output.push_str("\t{\n");

	output.push_str("\t\t$(#[$meta:meta])*\n");

	if !case.decl_keywords.is_empty() {
		output.push_str("\t\t");
		output.push_str(case.decl_keywords);
		output.push_str("\n");
	}

	output.push_str("\t\t$fn_name:ident\n");

	output.push_str("\t\t$([$($generics:tt)*])?\n");

	output.push_str("\t\t");
	output.push_str(case.decl_args);
	output.push_str("\n");

	output.push_str("\t\t$(where { $($where_clause:tt)* })?\n");

	output.push_str("\t\t=> $body:expr\n");

	output.push_str("\t} => {\n");

	output.push_str("\t\t$(#[$meta])*\n");
	output.push_str("\t\t#[inline]\n");

	output.push_str("\t\t");
	output.push_str(case.impl_fn_header);
	output.push_str(" $fn_name$(<$($generics)*>)?");
	output.push_str(case.impl_param_list);
	output.push_str(" -> Self\n");

	output.push_str("\t\t$(where $($where_clause)*)?\n");

	output.push_str("\t\t{\n");

	case.impl_body.iter().for_each(|line| {
		output.push_str("\t\t\t");
		output.push_str(line);
		output.push_str("\n");
	});

	output.push_str("\t\t}\n");

	output.push_str("\t};");

	output
}
