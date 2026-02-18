#![allow(
	bad_style,
	unused_imports,
	unused_macros,
	function_casts_as_integer,
	clippy::all
)]

use libmimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
