use crate::raw::glib::GMainLoop;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::g_main_loop_run;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Stops a [`GMainLoop`] from running. Any calls to [`g_main_loop_run`] for the loop will
    /// return.
    ///
    /// Note that sources that have already been dispatched when [`g_main_loop_quit`] is called
    /// will still be executed.
    pub fn g_main_loop_quit(r#loop: *mut GMainLoop);
}
