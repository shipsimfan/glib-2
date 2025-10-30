use crate::raw::glib::{GSourceFunc, gpointer, guint};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{
    G_PRIORITY_DEFAULT_IDLE, G_SOURCE_REMOVE, GMainContext, g_idle_add_full, g_idle_source_new,
    g_source_attach,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Adds a function to be called whenever there are no higher priority events pending to the
    /// default main loop.
    ///
    /// The function is given the default idle priority, [`G_PRIORITY_DEFAULT_IDLE`]. If the
    /// function returns [`G_SOURCE_REMOVE`] it is automatically removed from the list of event
    /// sources and will not be called again.
    ///
    /// This internally creates a main loop source using [`g_idle_source_new`] and attaches it to
    /// the global [`GMainContext`] using [`g_source_attach`], so the callback will be invoked in
    /// whichever thread is running that main context. You can do these steps manually if you need
    /// greater control or to use a custom main context.
    ///
    /// The implementation of this function is provided by [`g_idle_add_full`] in language
    /// bindings.
    ///
    /// # Parameters
    ///  * `function` - Function to call.
    ///  * `data` - Data to pass to function. The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// The ID (greater than 0) of the event source.
    pub fn g_idle_add(function: GSourceFunc, data: gpointer) -> guint;
}
