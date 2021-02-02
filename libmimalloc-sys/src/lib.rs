// Copyright 2019 Octavian Oncescu

use core::ffi::c_void;

#[cfg(feature = "extended")]
mod extended;
#[cfg(feature = "extended")]
pub use extended::*;

extern "C" {
    /// Allocate zero-initialized `size` bytes.
    ///
    /// Returns a pointer to newly allocated zero-initialized memory, or null if
    /// out of memory.
    pub fn mi_zalloc(size: usize) -> *mut c_void;

    /// Allocate `size` bytes.
    ///
    /// Returns pointer to the allocated memory or null if out of memory.
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_malloc(size: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as [`mi_malloc`]. If `newsize` is larger than
    /// the original `size` allocated for `p`, the bytes after `size` are
    /// uninitialized.
    pub fn mi_realloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment`, initialized to zero.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    ///
    /// Returns a unique pointer if called with `size` 0.
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes, aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory. If null
    /// is returned, the pointer `p` is not freed. Otherwise the original
    /// pointer is either freed or returned as the reallocated result (in case
    /// it fits in-place with the new size).
    ///
    /// If `p` is null, it behaves as [`mi_malloc_aligned`]. If `newsize` is
    /// larger than the original `size` allocated for `p`, the bytes after
    /// `size` are uninitialized.
    pub fn mi_realloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;

    /// Free previously allocated memory.
    ///
    /// The pointer `p` must have been allocated before (or be null).
    pub fn mi_free(p: *mut c_void);

    /// Return the available bytes in a memory block.
    ///
    /// The returned size can be used to call `mi_expand` successfully.
    pub fn mi_usable_size(p: *const c_void) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frees_memory_malloc() {
        let ptr = unsafe { mi_malloc_aligned(8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *mut c_void) };
    }

    #[test]
    fn it_frees_memory_zalloc() {
        let ptr = unsafe { mi_zalloc_aligned(8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *mut c_void) };
    }

    #[test]
    fn it_frees_memory_realloc() {
        let ptr = unsafe { mi_malloc_aligned(8, 8) } as *mut u8;
        let ptr = unsafe { mi_realloc_aligned(ptr as *mut c_void, 8, 8) } as *mut u8;
        unsafe { mi_free(ptr as *mut c_void) };
    }

    #[test]
    fn it_calculates_usable_size() {
        let ptr = unsafe { mi_malloc(32) } as *mut u8;
        let usable_size = unsafe { mi_usable_size(ptr as *mut c_void) };
        assert!(
            usable_size >= 32,
            "usable_size should at least equal to the allocated size"
        );
    }
}
