#![allow(bad_style, unused_imports, unused_macros, clippy::all)]

use libmimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
