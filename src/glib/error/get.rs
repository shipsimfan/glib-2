use crate::{
    glib::{GError, GQuark},
    raw,
};
use std::ffi::CStr;

impl GError {
    /// Get the domain of this error
    pub fn domain(&self) -> GQuark {
        unsafe { GQuark::new_raw(self.raw().domain) }
    }

    /// Get the code describing this error
    pub fn code(&self) -> raw::glib::gint {
        unsafe { self.raw().code }
    }

    /// Get the message as a [`CStr`]
    pub fn message(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw().message) }
    }

    /// Get a reference to the raw underlying error
    pub unsafe fn raw(&self) -> &raw::glib::GError {
        unsafe { &*self.handle }
    }

    /// Get a mutable reference to the raw underlying error
    pub unsafe fn raw_mut(&mut self) -> &mut raw::glib::GError {
        unsafe { &mut *self.handle }
    }

    /// Get the underlying handle to the error
    pub unsafe fn handle(&self) -> *mut raw::glib::GError {
        self.handle
    }
}
