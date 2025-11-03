use crate::{glib::GMainLoop, raw::glib::g_main_loop_quit};

impl GMainLoop {
    /// Stops a [`GMainLoop`] from running
    pub fn quit(&self) {
        unsafe { g_main_loop_quit(self.handle) };
    }
}
