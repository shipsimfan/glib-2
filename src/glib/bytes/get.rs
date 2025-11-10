use crate::{glib::GBytes, raw, util::AsHandle};

impl<'a> GBytes<'a> {
    /// Get the underlying handle to the [`raw::glib::GBytes`]
    pub const unsafe fn handle(&self) -> *mut raw::glib::GBytes {
        self.handle
    }
}

impl<'a> AsHandle for GBytes<'a> {
    unsafe fn as_handle(&self) -> *mut std::ffi::c_void {
        self.handle
    }
}
