use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[inline]
pub fn bin() {
	let mut args = env::args();
	let _ = args.next();
	let mut command;
	let mut invocation_type = InvocationType::None;
	let mut override_vars = false;

	macro_rules! require_next_arg {
		($($err_fmt:tt)*) => {
			{
				let Some(arg) = args.next() else {
					eprintln!($($err_fmt)*);
					return
				};

				arg
			}
		}
	}

	loop {
		command = args.next();
		match command.as_deref() {
			Some("--help" | "-h") => {
				print_help(true);
				return
			}
			Some("--version" | "-v") => {
				println!(concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")));
				return
			}
			Some(arg @ "--filename" | arg @ "-f") => {
				let arg = require_next_arg!("expected filename argument for {arg}");

				if !invocation_type.new_invocation(InvocationType::Filename(arg)) {
					return
				}
			}
			Some(arg @ "--path" | arg @ "-p") => {
				let arg = require_next_arg!("expected path argument for {arg}");

				if !invocation_type.new_invocation(InvocationType::Path(arg)) {
					return
				}
			}
			Some("--override" | "-o") => { override_vars = true }
			Some("--") => {
				command = args.next();
				break
			}
			Some(arg) if arg.starts_with("-") => {
				println!("unknown arg {arg}");
				println!();
				println!("If you're meaning to run the command `{arg}`, try adding `--` before the command");
				return
			}
			Some(_) | None => { break }
		}
	}

	let env_vars = match invocation_type {
		InvocationType::None => {
			dotenvy::dotenv_iter()
				.unwrap_or_else(|err| panic!("error reading .env file: {err}"))
		}
		InvocationType::Filename(filename) => {
			dotenvy::from_filename_iter(&*filename)
				.unwrap_or_else(|err| panic!("error reading {filename}: {err}"))
		}
		InvocationType::Path(path) => {
			dotenvy::from_path_iter(&*path)
				.unwrap_or_else(|err| panic!("error reading {path}: {err}"))
		}
	};

	match command {
		Some(command) => {
			let mut cmd = Command::new(command);
			cmd.args(args);

			for var in env_vars {
				let (k, v) = var.unwrap_or_else(|err| panic!("error getting var: {err}"));

				// either we override vars, so we set it no matter what
				// or the var doesn't exists, so we can set it
				// this took me way too long to figure out. :sifSuffering:
				if override_vars || env::var_os(&*k).is_none() {
					cmd.env(k, v);
				}
			}

			let err = cmd.exec();
			panic!("errored running the command: {err}");
		}
		None => {
			eprintln!("command not found");
			eprintln!();
			print_help(false);
		}
	}
}

enum InvocationType {
	Filename(String),
	Path(String),
	None
}

impl InvocationType {
	pub fn as_str(&self) -> &'static str {
		use InvocationType::*;

		match self {
			Filename(_) => { "filename" }
			Path(_) => { "path" }
			None => { "none" }
		}
	}

	pub fn new_invocation(&mut self, new: Self) -> bool {
		use InvocationType::*;

		match (&*self, &new) {
			(None, _) => {
				*self = new;
				return true
			}
			(Filename(_), Filename(_)) | (Path(_), Path(_)) => {
				println!("cannot specify {} more than once", self.as_str());
			}
			(Filename(_), Path(_)) | (Path(_), Filename(_))=> {
				println!("cannot specify both {} and {}", self.as_str(), new.as_str());
			}
			(_, None) => {
				println!("squints at you funny how did you manage this");
			}
		}

		false
	}
}

fn print_help(success: bool) {
	let err_msg = "\
		pretend this is a useful help command\n\
		man rust multiline strings are not ergonomic whatsoever\
	";

	if success {
		println!("{err_msg}");
	} else {
		eprintln!("{err_msg}");
	}
}
