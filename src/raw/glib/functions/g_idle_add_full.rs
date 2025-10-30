use crate::raw::glib::{GDestroyNotify, GSourceFunc, gint, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{
    G_PRIORITY_DEFAULT_IDLE, G_PRIORITY_HIGH_IDLE, G_SOURCE_REMOVE, GMainContext,
    g_idle_source_new, g_source_attach,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Adds a function to be called whenever there are no higher priority events pending.
    ///
    /// If the function returns [`G_SOURCE_REMOVE`] it is automatically removed from the list of
    /// event sources and will not be called again.
    ///
    /// This internally creates a main loop source using [`g_idle_source_new`] and attaches it to
    /// the global [`GMainContext`] using [`g_source_attach`], so the callback will be invoked in
    /// whichever thread is running that main context. You can do these steps manually if you need
    /// greater control or to use a custom main context.
    ///
    /// # Parameters
    ///  * `priority` - The priority of the idle source; typically this will be in the range
    ///                 between [`G_PRIORITY_DEFAULT_IDLE`] and [`G_PRIORITY_HIGH_IDLE`].
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to `function`. The argument can be [`null_mut`].
    ///  * `notify` - Function to call when the idle is removed. The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_idle_add_full(
        priority: gint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    ) -> guint;
}
