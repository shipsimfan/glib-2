use crate::{
    glib::{GMainContext, GMainLoop},
    raw::{
        self,
        glib::{FALSE, g_main_loop_new},
    },
};
use std::ptr::null_mut;

impl GMainLoop {
    /// Creates a new [`GMainLoop`] structure
    pub fn new(context: Option<&GMainContext>) -> Self {
        let handle = unsafe {
            g_main_loop_new(
                context
                    .map(|context| context.handle())
                    .unwrap_or(null_mut()),
                FALSE,
            )
        };
        unsafe { GMainLoop::new_raw(handle, true) }
    }

    /// Create a new [`GMainLoop`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::glib::GMainLoop, owned: bool) -> Self {
        assert_ne!(handle, null_mut());
        GMainLoop { handle, owned }
    }
}
