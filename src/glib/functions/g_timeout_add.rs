use crate::glib::{GSourceFunc, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{
    G_PRIORITY_DEFAULT, G_SOURCE_REMOVE, GMainContext, g_get_monotonic_time, g_source_attach,
    g_timeout_add_full, g_timeout_add_seconds, g_timeout_source_new,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets a function to be called at regular intervals, with the default priority,
    /// [`G_PRIORITY_DEFAULT`].
    ///
    /// The given function is called repeatedly until it returns [`G_SOURCE_REMOVE`], at which
    /// point the timeout is automatically destroyed and the function will not be called again. The
    /// first call to the function will be at the end of the first interval.
    ///
    /// Note that timeout functions may be delayed, due to the processing of other event sources.
    /// Thus they should not be relied on for precise timing. After each call to the timeout
    /// function, the time of the next timeout is recalculated based on the current time and the
    /// given interval (it does not try to ‘catch up’ time lost in delays).
    ///
    /// If you want to have a timer in the ‘seconds’ range and do not care about the exact time of
    /// the first call of the timer, use the [`g_timeout_add_seconds`] function; this function
    /// allows for more optimizations and more efficient system power usage.
    ///
    /// This internally creates a main loop source using [`g_timeout_source_new`] and attaches it
    /// to the global [`GMainContext`] using [`g_source_attach`], so the callback will be invoked
    /// in whichever thread is running that main context. You can do these steps manually if you
    /// need greater control or to use a custom main context.
    ///
    /// It is safe to call this function from any thread.
    ///
    /// The interval given is in terms of monotonic time, not wall clock time. See
    /// [`g_get_monotonic_time`].
    ///
    /// This function is not directly available to language bindings.
    ///
    /// The implementation of this function is provided by [`g_timeout_add_full`] in language
    /// bindings.
    ///
    /// # Parameters
    ///  * `interval` - The time between calls to the function, in milliseconds.
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to function. The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_timeout_add(interval: guint, function: GSourceFunc, data: gpointer) -> guint;
}
