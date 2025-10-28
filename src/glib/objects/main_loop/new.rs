use crate::glib::{GMainContext, GMainLoop, gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::g_main_loop_run;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Creates a new [`GMainLoop`] structure.
    ///
    /// # Parameters
    ///  * `context` - A main context (if [`null_mut`], the global-default main context will be
    ///                used). The argument can be [`null_mut`].
    ///  * `is_running` - Set to true to indicate that the loop is running. This is not very
    ///                   important since calling [`g_main_loop_run`] will set this to true anyway.
    pub fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
}
