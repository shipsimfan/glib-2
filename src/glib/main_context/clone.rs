use crate::{glib::GMainContext, raw::glib::g_main_context_ref};

impl Clone for GMainContext {
    fn clone(&self) -> Self {
        let handle = unsafe { g_main_context_ref(self.handle) };
        unsafe { GMainContext::new_raw(handle, true) }
    }
}
