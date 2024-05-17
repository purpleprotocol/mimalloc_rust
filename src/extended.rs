use crate::MiMalloc;

impl MiMalloc {
    /// Get the mimalloc version.
    /// 
    /// For mimalloc version 1.8.6, this will return 186.
    pub fn version(&self) -> u32 {
        unsafe { ffi::mi_version() as u32 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn it_gets_version() {
        let version = MiMalloc.version();
        assert!(version != 0);
    }
}