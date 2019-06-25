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
use libc::c_void;
use ffi::*;

// Copied from https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/alloc.rs
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
const MIN_ALIGN: usize = 8;

#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
const MIN_ALIGN: usize = 16;

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
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = if layout.align() > MIN_ALIGN {
            layout.align()
        } else {
            MIN_ALIGN
        };

        mi_malloc_aligned(layout.size(), align) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let align = if layout.align() > MIN_ALIGN {
            layout.align()
        } else {
            MIN_ALIGN
        };
        
        mi_zalloc_aligned(layout.size(), align) as *mut u8
    }
    
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr as *const c_void);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let align = if layout.align() > MIN_ALIGN {
            layout.align()
        } else {
            MIN_ALIGN
        };
        
        mi_realloc_aligned(ptr as *const c_void, new_size, align) as *mut u8
    }
}