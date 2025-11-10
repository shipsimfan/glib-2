use std::ffi::c_void;

/// An object which has a handle
pub trait AsHandle {
    /// Get a raw handle from an object
    unsafe fn as_handle(&self) -> *mut c_void;
}

impl AsHandle for *mut c_void {
    unsafe fn as_handle(&self) -> *mut c_void {
        *self
    }
}
