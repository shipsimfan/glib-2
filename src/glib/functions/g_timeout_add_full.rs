use crate::glib::{GDestroyNotify, GSourceFunc, gint, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{
    G_PRIORITY_DEFAULT, G_PRIORITY_HIGH, G_SOURCE_REMOVE, GMainContext, g_get_monotonic_time,
    g_source_attach, g_timeout_source_new,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets a function to be called at regular intervals, with the given priority.
    ///
    /// The function is called repeatedly until it returns [`G_SOURCE_REMOVE`], at which point the
    /// timeout is automatically destroyed and the function will not be called again. The `notify`
    /// function is called when the timeout is destroyed. The first call to the function will be at
    /// the end of the first interval.
    ///
    /// Note that timeout functions may be delayed, due to the processing of other event sources.
    /// Thus they should not be relied on for precise timing. After each call to the timeout
    /// function, the time of the next timeout is recalculated based on the current time and the
    /// given interval (it does not try to ‘catch up’ time lost in delays).
    ///
    /// This internally creates a main loop source using [`g_timeout_source_new`] and attaches it
    /// to the global [`GMainContext`] using [`g_source_attach`], so the callback will be invoked
    /// in whichever thread is running that main context. You can do these steps manually if you
    /// need greater control or to use a custom main context.
    ///
    /// The interval given is in terms of monotonic time, not wall clock time. See
    /// [`g_get_monotonic_time`].
    ///
    /// # Parameters
    ///  * `priority` - The priority of the timeout source; typically this will be in the range
    ///                 between [`G_PRIORITY_DEFAULT`] and [`G_PRIORITY_HIGH`].
    ///  * `interval` - The time between calls to the function, in milliseconds.
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to `function`. The argument can be [`null_mut`].
    ///  * `notify` - Function to call when the timeout is removed. The argument can be
    ///               [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_timeout_add_full(
        priority: gint,
        interval: guint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    ) -> guint;
}
