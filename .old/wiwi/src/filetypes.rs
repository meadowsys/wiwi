#![allow(
	dead_code,
	reason = "wip"
)]

use crate::prelude::*;
use std::path::Path;

pub struct FileInfo<'h> {
	// TODO: change to our own path type when I get that written maybe?
	filename: Option<&'h Path>,
	// TODO: change to our own path type when I get that written maybe?
	filepath: Option<&'h Path>,
	blob: Option<&'h [u8]>,

	// options
	parse_mode: Option<ParseMode>
}

enum ParseMode {
	MagicNumberOnly,
	FullFile
}
