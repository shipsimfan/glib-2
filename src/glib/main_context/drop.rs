use crate::{glib::GMainContext, raw::glib::g_main_context_unref};

impl Drop for GMainContext {
    fn drop(&mut self) {
        unsafe { g_main_context_unref(self.handle) };
    }
}
