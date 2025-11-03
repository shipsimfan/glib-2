use crate::{glib::GMainContext, raw::glib::g_main_context_default};

impl Default for GMainContext {
    fn default() -> Self {
        let handle = unsafe { g_main_context_default() };
        unsafe { GMainContext::new_raw(handle) }
    }
}
