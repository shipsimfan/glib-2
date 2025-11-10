use crate::{glib::GMainLoop, raw, util::AsHandle};

impl GMainLoop {
    /// Get the underlying handle to the main loop
    pub const unsafe fn handle(&self) -> *mut raw::glib::GMainLoop {
        self.handle
    }
}

impl AsHandle for GMainLoop {
    unsafe fn as_handle(&self) -> *mut std::ffi::c_void {
        self.handle
    }
}
