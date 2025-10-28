use crate::glib::GMainContext;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{g_main_context_get_thread_default, g_main_context_ref};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets a reference to the thread-default [`GMainContext`] for this thread
    ///
    /// This is the same as [`g_main_context_get_thread_default`], but it also adds a reference to
    /// the returned main context with [`g_main_context_ref`]. In addition, unlike
    /// [`g_main_context_get_thread_default`], if the thread-default context is the global-default
    /// context, this will return that [`GMainContext`] (with a ref added to it) rather than
    /// returning [`null_mut`].
    ///
    /// # Return Value
    /// The thread-default main context.
    pub fn g_main_context_ref_thread_default() -> *mut GMainContext;
}
