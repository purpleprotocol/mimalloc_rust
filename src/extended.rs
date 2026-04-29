use crate::MiMalloc;
use core::ffi::c_void;
#[cfg(not(feature = "v2"))]
use core::ffi::{c_char, CStr};

impl MiMalloc {
    /// Get the mimalloc version.
    ///
    /// For mimalloc version 1.8.6, this will return 186.
    pub fn version(&self) -> u32 {
        unsafe { ffi::mi_version() as u32 }
    }

    /// Return the amount of available bytes in a memory block.
    ///
    /// # Safety
    /// `ptr` must point to a memory block allocated by mimalloc, or be null.
    #[inline]
    pub unsafe fn usable_size(&self, ptr: *const u8) -> usize {
        ffi::mi_usable_size(ptr as *const c_void)
    }

    /// Call the given function with a string version of the JSON stats for the whole process
    ///
    /// Allocates (using mimalloc itself) to store the JSON structure.
    #[cfg(not(feature = "v2"))]
    pub fn with_stats_json<F, O>(f: F) -> Result<O, &'static str>
    where
        F: FnOnce(&str) -> O,
    {
        unsafe {
            let buf = ffi::mi_stats_get_json(0, core::ptr::null::<c_char>() as *mut _);
            if buf.is_null() {
                return Err("failed to call mi_stats_get_json");
            }
            let cstr = CStr::from_ptr(buf);
            let slice = cstr
                .to_str()
                .map_err(|_| "mi_stats_get_json contained invalid UTF-8")?;
            let o = f(slice);
            ffi::mi_free(buf as _);
            Ok(o)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::alloc::GlobalAlloc;
    use core::alloc::Layout;

    #[test]
    fn it_gets_version() {
        let version = MiMalloc.version();
        assert!(version != 0);
    }

    #[test]
    fn it_checks_usable_size() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = MiMalloc;

            let ptr = alloc.alloc(layout);
            let usable_size = alloc.usable_size(ptr);
            alloc.dealloc(ptr, layout);
            assert!(usable_size >= 8);
        }
    }

    #[test]
    #[cfg(not(feature = "v2"))]
    fn test_with_stats_json() {
        let (first_char, len) = MiMalloc::with_stats_json(|f| (f.chars().next(), f.len())).unwrap();
        assert_eq!(first_char, Some('{'));
        assert!(len > 1);
    }
}
