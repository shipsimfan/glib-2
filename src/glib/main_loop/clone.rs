use crate::{glib::GMainLoop, raw::glib::g_main_loop_ref};

impl Clone for GMainLoop {
    fn clone(&self) -> Self {
        let handle = unsafe { g_main_loop_ref(self.handle) };
        unsafe { GMainLoop::new_raw(handle, true) }
    }
}
