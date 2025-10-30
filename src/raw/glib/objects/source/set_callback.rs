use crate::raw::glib::{GDestroyNotify, GSource, GSourceFunc, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    G_SOURCE_FUNC,
    raw::glib::{g_idle_add, g_source_destroy, g_timeout_add},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets the callback function for a source. The callback for a source is called from the
    /// source’s dispatch function.
    ///
    /// The exact type of `func` depends on the type of source; ie. you should not count on `func`
    /// being called with data as its first parameter. Cast `func` with [`G_SOURCE_FUNC`] to avoid
    /// warnings about incompatible function types.
    ///
    /// Typically, you won’t use this function. Instead use functions specific to the type of
    /// source you are using, such as [`g_idle_add`] or [`g_timeout_add`].
    ///
    /// It is safe to call this function multiple times on a source which has already been attached
    /// to a context. The changes will take effect for the next time the source is dispatched after
    /// this call returns.
    ///
    /// Note that [`g_source_destroy`] for a currently attached source has the effect of also
    /// unsetting the callback.
    ///
    /// # Parameters
    ///  * `func` - A callback function.
    ///  * `data` - The data to pass to callback function. The argument can be [`null_mut`].
    ///  * `notify` - A function to call when data is no longer in use. The argument can be
    ///               [`null_mut`].
    pub fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
}
