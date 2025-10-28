use crate::glib::{GMainLoop, gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::g_main_loop_run;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Checks to see if the main loop is currently being run via [`g_main_loop_run`].
    ///
    /// # Return Value
    /// True if the main loop is currently being run, false otherwise.
    pub fn g_main_loop_is_running(r#loop: *mut GMainLoop) -> gboolean;
}
