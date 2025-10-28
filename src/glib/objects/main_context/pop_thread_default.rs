use crate::glib::GMainContext;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Pops `context` off the thread-default context stack (verifying that it was on the top of
    /// the stack).
    ///
    /// # Parameters
    ///  * `context` - A main context, or [`null_mut`] for the global-default main context.
    pub fn g_main_context_pop_thread_default(context: *mut GMainContext);
}
