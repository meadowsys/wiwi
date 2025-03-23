#![no_implicit_prelude]

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

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
		f128,
		// used by `rc_nightly`
		ptr_metadata
	)
)]

#[cfg(target_pointer_width = "16")]
compile_error!("16-bit platforms are not supported yet (but please do file an issue if for whatever reason you do need it, I would be happy to add support!)");

#[doc(hidden)]
pub extern crate wiwi_macro_proc as __internal_proc_macros;

extern crate wiwi_util;
pub use wiwi_util::*;

// #[cfg(feature = "chain")]
pub extern crate wiwi_chain as chain;

pub mod aoc;
pub mod builder;
pub mod clock_timer;
pub mod cron;
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
#[cfg(feature = "nightly")]
pub mod rc_nightly;
pub mod serialiser;
pub mod string;
pub mod tuple;
pub mod vh;
