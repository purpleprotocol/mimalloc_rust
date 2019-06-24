// Copyright 2019 Octavian Oncescu

use libc::{c_void, size_t};

extern "C" {
   pub fn mi_malloc_aligned(size: size_t, alignment: size_t) -> *const c_void;
   pub fn mi_free(p: *const c_void) -> c_void;
}