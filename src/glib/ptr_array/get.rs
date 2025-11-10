use crate::{glib::GPtrArray, raw, util::AsHandle};

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

impl<'a, T> AsHandle for GPtrArray<'a, T> {
    unsafe fn as_handle(&self) -> *mut std::ffi::c_void {
        self.handle.cast()
    }
}
