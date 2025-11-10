use std::ptr::null_mut;

use crate::{
    glib::GMainContext,
    raw::glib::{
        g_main_context_default, g_main_context_get_thread_default,
        g_main_context_pop_thread_default, g_main_context_push_thread_default,
    },
};

impl Default for GMainContext {
    fn default() -> Self {
        let handle = unsafe { g_main_context_default() };
        unsafe { GMainContext::new_raw(handle, false) }
    }
}

impl GMainContext {
    /// Gets the thread-default main context for this thread.
    pub fn thread_default() -> Option<Self> {
        let handle = unsafe { g_main_context_get_thread_default() };
        if handle == null_mut() {
            return None;
        }

        Some(unsafe { GMainContext::new_raw(handle, false) })
    }

    /// Acquires context and sets it as the thread-default context for the current thread.
    pub fn push_thread_default(&self) {
        unsafe { g_main_context_push_thread_default(self.handle) };
    }

    /// Pops context off the thread-default context stack (verifying that it was on the top of the
    /// stack).
    pub fn pop_thread_default(&self) {
        unsafe { g_main_context_pop_thread_default(self.handle) };
    }
}
