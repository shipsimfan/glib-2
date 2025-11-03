use crate::{
    glib::GMainContext,
    raw::{self, glib::g_main_context_new},
};
use std::ptr::null_mut;

impl GMainContext {
    /// Creates a new [`GMainContext`] structure
    pub fn new() -> Self {
        let handle = unsafe { g_main_context_new() };
        unsafe { GMainContext::new_raw(handle) }
    }

    /// Create a new [`GMainContext`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::glib::GMainContext) -> Self {
        assert_ne!(handle, null_mut());
        GMainContext { handle }
    }
}
