use crate::{gobject::GObject, raw, util::AsHandle};

impl GObject {
    /// Get the underlying handle to the object
    pub unsafe fn handle(&self) -> *mut raw::gobject::GObject {
        self.handle
    }
}

impl AsHandle for GObject {
    unsafe fn as_handle(&self) -> *mut std::ffi::c_void {
        self.handle
    }
}
