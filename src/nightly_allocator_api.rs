use crate::MiMalloc;
use core::alloc::{AllocError, Allocator, Layout};
use core::ffi::c_void;
use core::ptr::{write_bytes, NonNull};
use ffi::*;

impl MiMalloc {
    /// Interprets a raw pointer returned from a memory allocation call.
    /// Tags the pointer with the associated allocation size,
    /// or returns [`AllocError`] if the pointer was null.
    ///
    /// # Safety
    ///
    /// The `raw_ptr` must have been returned by mimalloc.
    /// It should either refer to a live allocation or be null.
    #[inline]
    unsafe fn tag_allocation(raw_ptr: *mut c_void) -> Result<NonNull<[u8]>, AllocError> {
        if let Some(ptr) = NonNull::new(raw_ptr as *mut _) {
            // Safety: `raw_ptr` was previously allocated with mimalloc
            let len = unsafe { mi_usable_size(raw_ptr) };
            Ok(NonNull::from_raw_parts(ptr, len))
        } else {
            Err(AllocError)
        }
    }
}

unsafe impl Allocator for MiMalloc {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling_ptr(), 0)),
            // Safety: the pointer passed to `tag_allocation` is either null or comes from mimalloc
            _ => unsafe { Self::tag_allocation(mi_malloc_aligned(layout.size(), layout.align())) },
        }
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        match layout.size() {
            // Do nothing
            0 => {}
            // Safety: by the function preconditions, `ptr` came from this allocator
            _ => unsafe { mi_free(ptr.as_ptr() as *mut _) },
        }
    }

    #[inline]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling_ptr(), 0)),
            // Safety: the pointer passed to `tag_allocation` is either null or comes from mimalloc
            _ => unsafe { Self::tag_allocation(mi_zalloc_aligned(layout.size(), layout.align())) },
        }
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        match (old_layout.size(), new_layout.size()) {
            // Do nothing
            (0, 0) => Ok(NonNull::slice_from_raw_parts(new_layout.dangling_ptr(), 0)),
            // Safety: by the function preconditions, `ptr` came from this allocator
            (0, _) => self.allocate(new_layout),
            // Safety: by the function preconditions, `ptr` came from this allocator
            (_, _) => unsafe {
                Self::tag_allocation(mi_realloc_aligned(
                    ptr.as_ptr() as *mut _,
                    new_layout.size(),
                    new_layout.align(),
                ))
            },
        }
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let old_usable_size = match old_layout.size() {
            0 => 0,
            // Safety: `ptr` refers to a valid allocation from mimalloc
            _ => unsafe { mi_usable_size(ptr.as_ptr() as *mut _) },
        };

        // Safety: by the function preconditions, `ptr` came from this allocator
        let result = unsafe { self.grow(ptr, old_layout, new_layout)? };

        // Safety: only bytes within the bounds of the new allocation are written
        unsafe {
            write_bytes(
                result.cast::<u8>().add(old_usable_size).as_ptr(),
                0,
                result.len() - old_usable_size,
            );
        }

        Ok(result)
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        match (old_layout.size(), new_layout.size()) {
            // Do nothing
            (0, 0) => Ok(NonNull::slice_from_raw_parts(new_layout.dangling_ptr(), 0)),
            // Safety: by the function preconditions, `ptr` came from this allocator
            (_, 0) => unsafe {
                self.deallocate(ptr, old_layout);
                Ok(NonNull::slice_from_raw_parts(new_layout.dangling_ptr(), 0))
            },
            // Safety: by the function preconditions, `ptr` came from this allocator
            (_, _) => unsafe {
                Self::tag_allocation(mi_realloc_aligned(
                    ptr.as_ptr() as *mut _,
                    new_layout.size(),
                    new_layout.align(),
                ))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frees_allocated_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        unsafe { MiMalloc.deallocate(ptr.cast(), layout) };
    }

    #[test]
    fn it_frees_allocated_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        unsafe { MiMalloc.deallocate(ptr.cast(), layout) };
    }

    #[test]
    fn it_frees_zero_allocated_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let ptr = MiMalloc.allocate_zeroed(layout).unwrap();
        unsafe { MiMalloc.deallocate(ptr.cast(), layout) };
    }

    #[test]
    fn it_frees_zero_allocated_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let ptr = MiMalloc.allocate_zeroed(layout).unwrap();
        unsafe { MiMalloc.deallocate(ptr.cast(), layout) };
    }

    #[test]
    fn it_frees_grown_memory() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let new_layout = Layout::from_size_align(16, 8).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        let ptr = unsafe { MiMalloc.grow(ptr.cast(), layout, new_layout).unwrap() };
        unsafe { MiMalloc.deallocate(ptr.cast(), new_layout) };
    }

    #[test]
    fn it_frees_grown_big_memory() {
        let layout = Layout::from_size_align(1 << 20, 32).unwrap();
        let new_layout = Layout::from_size_align(2 << 20, 32).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        let ptr = unsafe { MiMalloc.grow(ptr.cast(), layout, new_layout).unwrap() };
        unsafe { MiMalloc.deallocate(ptr.cast(), new_layout) };
    }

    #[test]
    fn empty_allocation_has_size_zero() {
        let layout = Layout::from_size_align(0, 1).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        assert_eq!(ptr.len(), 0);
        unsafe { MiMalloc.deallocate(ptr.cast(), layout) };
    }

    #[test]
    fn shrink_to_empty_has_size_zero() {
        let layout = Layout::from_size_align(8, 8).unwrap();
        let empty_layout = Layout::from_size_align(0, 8).unwrap();
        let ptr = MiMalloc.allocate(layout).unwrap();
        let ptr = unsafe { MiMalloc.shrink(ptr.cast(), layout, empty_layout).unwrap() };
        assert_eq!(ptr.len(), 0);
        unsafe { MiMalloc.deallocate(ptr.cast(), empty_layout) };
    }
}
