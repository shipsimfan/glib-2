use crate::{glib::GMainLoop, raw::glib::g_main_loop_unref};

impl Drop for GMainLoop {
    fn drop(&mut self) {
        unsafe { g_main_loop_unref(self.handle) };
    }
}
