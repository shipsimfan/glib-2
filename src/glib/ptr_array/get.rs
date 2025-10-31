use crate::{glib::GPtrArray, raw};

impl<'a, T> GPtrArray<'a, T> {
    /// Get the array as a slice
    pub const fn as_slice(&self) -> &'a [*mut T] {
        self.slice
    }

    /// Get the underlying handle to the pointer array
    pub const unsafe fn handle(&self) -> *mut raw::glib::GPtrArray {
        self.handle
    }
}
