use crate::raw::glib::{GMainContext, GSource, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::g_source_destroy;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Adds a [`GSource`] to a context so that it will be executed within that context.
    ///
    /// Remove it by calling [`g_source_destroy`].
    ///
    /// This function is safe to call from any thread, regardless of which thread the context is
    /// running in.
    ///
    /// # Parameters
    ///  * `context` - A main context (if [`null_mut`], the global-default main context will be
    ///                used).
    ///
    /// # Return Value
    /// The ID (greater than 0) for the source within the [`GMainContext`].
    pub fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
}
