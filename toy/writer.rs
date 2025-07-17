use core::{
    fmt::{Result, Write},
    ptr,
};

pub struct Writer {
    ptr: *mut u8,
}

impl Writer {
    /// Creates a new instance from a raw pointer.
    ///
    #[inline]
    pub fn new(ptr: *mut u8) -> Self {
        Self { ptr }
    }

    /// Returns the underlying raw pointer.
    #[inline]
    pub fn ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Advances the pointer by `count` bytes.
    #[inline]
    pub fn add(&mut self, count: usize) {
        self.ptr = unsafe { self.ptr.add(count) }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        let count = s.len();
        unsafe { ptr::copy_nonoverlapping(s.as_ptr(), self.ptr, count) };
        self.add(count);
        Ok(())
    }
}
