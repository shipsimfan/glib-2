use crate::glib::GMainContext;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::g_main_context_get_thread_default;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the global-default main context.
    ///
    /// This is the main context used for main loop functions when a main loop is not explicitly
    /// specified, and corresponds to the ‘main’ main loop. See also
    /// [`g_main_context_get_thread_default`].
    ///
    /// # Return Value
    /// The global-default main context.
    pub fn g_main_context_default() -> *mut GMainContext;
}
