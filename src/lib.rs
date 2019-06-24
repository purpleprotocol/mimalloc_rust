// Copyright 2019 Octavian Oncescu

use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;
use ffi::*;

pub struct MimAlloc;

unsafe impl GlobalAlloc for MimAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

mod ffi;