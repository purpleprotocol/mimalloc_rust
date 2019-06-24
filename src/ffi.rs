// Copyright 2019 Octavian Oncescu
use libc::{c_void, size_t};

extern "C" {
   fn mi_malloc_aligned(size: size_t, alignment: size_t) -> c_void;
   fn mi_free(p: *const c_void) -> c_void;
}