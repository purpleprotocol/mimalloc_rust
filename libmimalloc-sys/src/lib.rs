// Copyright 2019 Octavian Oncescu

use core::ffi::c_void;

extern "C" {
    pub fn mi_zalloc(size: usize) -> *mut c_void;
    pub fn mi_malloc(size: usize) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, size: usize) -> *mut c_void;
    pub fn mi_zalloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_malloc_aligned(size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_realloc_aligned(p: *mut c_void, size: usize, alignment: usize) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);
    pub fn mi_usable_size(p: *mut c_void) -> usize;
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
