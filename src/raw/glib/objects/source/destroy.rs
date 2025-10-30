use crate::raw::glib::GSource;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{GDestroyNotify, GMainContext, g_source_set_callback, g_source_unref};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Removes a source from its [`GMainContext`], if any, and marks it as destroyed.
    ///
    /// The source cannot be subsequently added to another context. It is safe to call this on
    /// sources which have already been removed from their context.
    ///
    /// This does not unref the [`GSource`]: if you still hold a reference, use [`g_source_unref`]
    /// to drop it.
    ///
    /// This function is safe to call from any thread, regardless of which thread the
    /// [`GMainContext`] is running in.
    ///
    /// If the source is currently attached to a [`GMainContext`], destroying it will effectively
    /// unset the callback similar to calling [`g_source_set_callback`]. This can mean, that the
    /// data’s [`GDestroyNotify`] gets called right away.
    pub fn g_source_destroy(source: *mut GSource);
}
