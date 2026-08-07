// Copyright 2019 Octavian Oncescu

#![no_std]
#![cfg_attr(feature = "nightly_allocator_api", feature(allocator_api))]
#![cfg_attr(feature = "nightly_allocator_api", feature(ptr_metadata))]

//! A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
//! Mimalloc is a general purpose, performance oriented allocator built by Microsoft.
//!
//! ## Usage
//! ```rust,ignore
//! use mimalloc::MiMalloc;
//!
//! #[global_allocator]
//! static GLOBAL: MiMalloc = MiMalloc;
//! ```
//!
//! ## Usage with secure mode
//! Using secure mode adds guard pages,
//! randomized allocation, encrypted free lists, etc. The performance penalty is usually
//! around 10% according to [mimalloc's](https://github.com/microsoft/mimalloc)
//! own benchmarks.
//!
//! To enable secure mode, put in `Cargo.toml`:
//! ```toml
//! [dependencies]
//! mimalloc = { version = "*", features = ["secure"] }
//! ```

extern crate libmimalloc_sys as ffi;

#[cfg(feature = "extended")]
mod extended;

#[cfg(feature = "nightly_allocator_api")]
mod nightly_allocator_api;

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use ffi::*;

/// Drop-in mimalloc global allocator.
///
/// ## Usage
/// ```rust,ignore
/// use mimalloc::MiMalloc;
///
/// #[global_allocator]
/// static GLOBAL: MiMalloc = MiMalloc;
/// ```
pub struct MiMalloc;

/// The minimum alignment that mimalloc's default allocation functions
/// (`mi_malloc` / `mi_zalloc`) guarantee for every block, regardless of the
/// requested `Layout::align()`.
///
/// This mirrors mimalloc's `MI_MAX_ALIGN_SIZE` constant (`sizeof(max_align_t)`,
/// 16 bytes on most platforms), despite the "MAX" in its C name actually
/// denoting a guaranteed *minimum* alignment.
const MIN_MIMALLOC_ALIGNMENT: usize = 16;

/// The alignment threshold below which mimalloc's `realloc` internally
/// falls back to the unaligned reallocation path.
///
/// Corresponds to mimalloc's internal check
/// `alignment <= sizeof(uintptr_t)` inside
/// `mi_theap_realloc_zero_aligned_at`, which delegates to
/// `_mi_theap_realloc_zero` when the alignment does not exceed the size of
/// a pointer (8 bytes on 64-bit platforms, 4 on 32-bit).
///
/// Note this differs from [`MIN_MIMALLOC_ALIGNMENT`], which is 16: mimalloc
/// does not use the same threshold for `realloc` as it does for `malloc`.
const MIN_REALLOC_ALIGNMENT: usize = size_of::<usize>();

unsafe impl GlobalAlloc for MiMalloc {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_MIMALLOC_ALIGNMENT {
            mi_malloc(layout.size()) as *mut u8
        } else {
            mi_malloc_aligned(layout.size(), layout.align()) as *mut u8
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        mi_free(ptr as *mut c_void);
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= MIN_MIMALLOC_ALIGNMENT {
            mi_zalloc(layout.size()) as *mut u8
        } else {
            mi_zalloc_aligned(layout.size(), layout.align()) as *mut u8
        }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if layout.align() <= MIN_REALLOC_ALIGNMENT {
            mi_realloc(ptr as *mut c_void, new_size) as *mut u8
        } else {
            mi_realloc_aligned(ptr as *mut c_void, new_size, layout.align()) as *mut u8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frees_allocated_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc(layout) };
        unsafe { alloc.dealloc(ptr, layout) };
    }

    #[test]
    fn it_frees_allocated_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc(layout) };
        unsafe { alloc.dealloc(ptr, layout) };
    }

    #[test]
    fn it_frees_zero_allocated_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc_zeroed(layout) };
        unsafe { alloc.dealloc(ptr, layout) };
    }

    #[test]
    fn it_frees_zero_allocated_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc_zeroed(layout) };
        unsafe { alloc.dealloc(ptr, layout) };
    }

    #[test]
    fn it_frees_reallocated_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc(layout) };
        let ptr = unsafe { alloc.realloc(ptr, layout, 16) };
        unsafe { alloc.dealloc(ptr, layout) };
    }

    #[test]
    fn it_frees_reallocated_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let alloc = MiMalloc;

        let ptr = unsafe { alloc.alloc(layout) };
        let ptr = unsafe { alloc.realloc(ptr, layout, 2 << 20) };
        unsafe { alloc.dealloc(ptr, layout) };
    }
}
