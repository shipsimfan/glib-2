use crate::glib::{GSourceFunc, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{
    G_PRIORITY_DEFAULT, G_SOURCE_REMOVE, g_get_monotonic_time, g_source_attach, g_timeout_add,
    g_timeout_add_seconds_full, g_timeout_source_new_seconds,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets a function to be called at regular intervals with the default priority,
    /// [`G_PRIORITY_DEFAULT`].
    ///
    /// The function is called repeatedly until it returns [`G_SOURCE_REMOVE`], at which point the
    /// timeout is automatically destroyed and the function will not be called again.
    ///
    /// This internally creates a main loop source using [`g_timeout_source_new_seconds`] and
    /// attaches it to the main loop context using [`g_source_attach`]. You can do these steps
    /// manually if you need greater control. Also see [`g_timeout_add_seconds_full`].
    ///
    /// It is safe to call this function from any thread.
    ///
    /// Note that the first call of the timer may not be precise for timeouts of one second. If you
    /// need finer precision and have such a timeout, you may want to use [`g_timeout_add`]
    /// instead.
    ///
    /// The interval given is in terms of monotonic time, not wall clock time. See
    /// [`g_get_monotonic_time`].
    ///
    /// The implementation of this function is provided by [`g_timeout_add_seconds_full`] in
    /// language bindings.
    ///
    /// # Parameters
    ///  * `interval` - The time between calls to the function, in seconds.
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to function. The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_timeout_add_seconds(interval: guint, function: GSourceFunc, data: gpointer) -> guint;
}
