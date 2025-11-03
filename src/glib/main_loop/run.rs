use crate::{glib::GMainLoop, raw::glib::g_main_loop_run};

impl GMainLoop {
    /// Runs a main loop until [`GMainLoop::quit`] is called on the loop
    pub fn run(&self) {
        unsafe { g_main_loop_run(self.handle) };
    }
}
