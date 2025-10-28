use crate::glib::GMainContext;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{GSource, g_main_context_ref_thread_default};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the thread-default main context for this thread.
    ///
    /// Asynchronous operations that want to be able to be run in contexts other than the default
    /// one should call this method or [`g_main_context_ref_thread_default`] to get a
    /// [`GMainContext`] to add their [`GSource`]s to. (Note that even in single-threaded programs
    /// applications may sometimes want to temporarily push a non-default context, so it is not
    /// safe to assume that this will always return [`null_mut`] if you are running in the default
    /// thread.)
    ///
    /// If you need to hold a reference on the context, use [`g_main_context_ref_thread_default`]
    /// instead.
    ///
    /// # Return Value
    /// The thread-default main context, or [`null_mut`] if the thread-default context is the
    /// global-default main context.
    pub fn g_main_context_get_thread_default() -> *mut GMainContext;
}
