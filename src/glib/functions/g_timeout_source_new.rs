use crate::glib::{GSource, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{GMainContext, g_get_monotonic_time, g_source_attach};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Creates a new timeout source.
    ///
    /// The source will not initially be associated with any [`GMainContext`] and must be added to
    /// one with [`g_source_attach`] before it will be executed.
    ///
    /// The interval given is in terms of monotonic time, not wall clock time. See
    /// [`g_get_monotonic_time`].
    ///
    /// # Parameters
    ///  * `interval` - The timeout interval in milliseconds.
    ///
    /// # Return Value
    /// The newly-created timeout source.
    pub fn g_timeout_source_new(interval: guint) -> *mut GSource;
}
