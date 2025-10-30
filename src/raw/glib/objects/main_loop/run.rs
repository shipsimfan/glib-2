use crate::raw::glib::GMainLoop;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{GMainContext, g_main_loop_quit};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Runs a main loop until [`g_main_loop_quit`] is called on the loop.
    ///
    /// If this is called from the thread of the loop’s [`GMainContext`], it will process events
    /// from the loop, otherwise it will simply wait.
    pub fn g_main_loop_run(r#loop: *mut GMainLoop);
}
