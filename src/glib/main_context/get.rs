use crate::{glib::GMainContext, raw, util::AsHandle};

impl GMainContext {
    /// Get the underlying handle to the main context
    pub const unsafe fn handle(&self) -> *mut raw::glib::GMainContext {
        self.handle
    }
}

impl AsHandle for GMainContext {
    unsafe fn as_handle(&self) -> *mut std::ffi::c_void {
        self.handle
    }
}
