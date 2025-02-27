#![no_implicit_prelude]

#![cfg_attr(docsrs, feature(doc_cfg))]

#![cfg_attr(all(docsrs, kiwingay), doc = "")]
#![cfg_attr(
	all(docsrs, kiwingay),
	doc = concat!(
		"These docs have been built from commit [",
		env!("KIWINGAY_DEPLOY_COMMIT_SHORT"),
		"](https://github.com/meadowsys/wiwi/commit/",
		env!("KIWINGAY_DEPLOY_COMMIT"),
		")."
	)
)]

#![cfg_attr(
	feature = "nightly",
	feature(
		const_trait_impl,
		integer_atomics,
		f16,
		f128
	)
)]

#[cfg(target_pointer_width = "16")]
compile_error!("16-bit platforms are not supported yet (but please do file an issue if for whatever reason you do need it, I would be happy to add support!)");

#[cfg(any(doc, docsrs, kiwingay))]
#[doc = include_str!("../CHANGELOG.md")]
pub mod _changelog {}

pub mod prelude;
pub mod prelude_std;

pub mod aoc;
pub mod builder;
pub mod chain;
pub mod clock_timer;
pub mod encoding;
pub mod filetypes;
pub mod lazy_wrap;
pub mod lsl;
pub mod macro_util;
pub mod mem_use;
pub mod nominal;
pub mod num;
pub mod parser;
pub mod rc;
pub mod serialiser;
pub mod slice;
pub mod string;
pub mod tuple;
pub mod vh;
