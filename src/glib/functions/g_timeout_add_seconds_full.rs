use crate::glib::{GDestroyNotify, GSourceFunc, gint, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{
    G_PRIORITY_DEFAULT, G_PRIORITY_HIGH, G_SOURCE_REMOVE, g_get_monotonic_time, g_source_attach,
    g_timeout_add, g_timeout_add_seconds, g_timeout_source_new_seconds,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets a function to be called at regular intervals, with priority.
    ///
    /// The function is called repeatedly until it returns [`G_SOURCE_REMOVE`], at which point the
    /// timeout is automatically destroyed and the function will not be called again.
    ///
    /// Unlike [`g_timeout_add`], this function operates at whole second granularity. The initial
    /// starting point of the timer is determined by the implementation and the implementation is
    /// expected to group multiple timers together so that they fire all at the same time. To allow
    /// this grouping, the interval to the first timer is rounded and can deviate up to one second
    /// from the specified interval. Subsequent timer iterations will generally run at the
    /// specified interval.
    ///
    /// Note that timeout functions may be delayed, due to the processing of other event sources.
    /// Thus they should not be relied on for precise timing. After each call to the timeout
    /// function, the time of the next timeout is recalculated based on the current time and the
    /// given interval
    ///
    /// If you want timing more precise than whole seconds, use [`g_timeout_add`] instead.
    ///
    /// The grouping of timers to fire at the same time results in a more power and CPU efficient
    /// behavior so if your timer is in multiples of seconds and you don’t require the first timer
    /// exactly one second from now, the use of [`g_timeout_add_seconds`] is preferred over
    /// [`g_timeout_add`].
    ///
    /// This internally creates a main loop source using [`g_timeout_source_new_seconds`] and
    /// attaches it to the main loop context using [`g_source_attach`]. You can do these steps
    /// manually if you need greater control.
    ///
    /// It is safe to call this function from any thread.
    ///
    /// The interval given is in terms of monotonic time, not wall clock time. See
    /// [`g_get_monotonic_time`].
    ///
    /// # Parameters
    ///  * `priority` - The priority of the timeout source; typically this will be in the range
    ///                 between [`G_PRIORITY_DEFAULT`] and [`G_PRIORITY_HIGH`].
    ///  * `interval` - The time between calls to the function, in seconds.
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to `function`. The argument can be [`null_mut`].
    ///  * `notify` - Function to call when the timeout is removed. The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_timeout_add_seconds_full(
        priority: gint,
        interval: guint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    ) -> guint;
}
