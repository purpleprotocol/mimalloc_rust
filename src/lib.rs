// Copyright 2019 Octavian Oncescu

//! A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
//! Mimalloc is a general purpose, performance oriented allocator built by Microsoft.
//! 
//! ## Usage
//! ```rust
//! use mimalloc::MiMalloc;
//!
//! #[global_allocator]
//! static GLOBAL: MiMalloc = MiMalloc;
//! ```
//! 
//! ## Usage without secure mode
//! By default this library builds mimalloc in safe-mode. This means that
//! heap allocations are encrypted, but this results in a 3% increase in overhead.
//! 
//! In `Cargo.toml`:
//! ```rust
//! [dependencies]
//! mimalloc = { version = "*", features = ["no_secure"] }
//! ```

extern crate libmimalloc_sys as ffi;

use std::alloc::{GlobalAlloc, Layout};
use std::ptr::null_mut;
use libc::c_void;
use ffi::*;

/// Drop-in mimalloc global allocator.
/// 
/// ## Usage
/// ```rust
/// use mimalloc::MiMalloc;
///
/// #[global_allocator]
/// static GLOBAL: MiMalloc = MiMalloc;
/// ```
pub struct MiMalloc;

unsafe impl GlobalAlloc for MiMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { 
        mi_malloc_aligned(layout.size(), layout.align());
        null_mut()
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr as *const c_void);
    }
}