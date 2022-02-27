#![allow(bad_style, deprecated, clippy::all)]

use libmimalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
