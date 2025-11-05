use crate::{glib::GError, raw};
use std::ptr::null_mut;

impl GError {
    /// Create a new [`GError`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::glib::GError, owned: bool) -> Self {
        assert_ne!(handle, null_mut());
        GError { handle, owned }
    }
}
