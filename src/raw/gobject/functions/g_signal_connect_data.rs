use crate::raw::{
    glib::{gchar, gpointer, gulong},
    gobject::{GCallback, GClosureNotify, GConnectFlags, GObject},
};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Connects a [`GCallback`] function to a signal for a particular object. Similar to
    /// [`g_signal_connect`], but allows to provide a [`GClosureNotify`] for the data which will be
    /// called when the signal handler is disconnected and no longer used. Specify `connect_flags`
    /// if you need `..._after()` or `..._swapped()` variants of this function.
    ///
    /// This function cannot fail. If the given signal name doesn’t exist, a critical warning is
    /// emitted. No validation is performed on the ‘detail’ string when specified in
    /// `detailed_signal`, other than a non-empty check.
    ///
    /// # Parameters
    ///  * `instance` - The instance to connect to.
    ///  * `detailed_signal` - A string of the form “signal-name::detail”. The value is a NUL
    ///                        terminated UTF-8 string.
    ///  * `c_handler` - The [`GCallback`] to connect.
    ///  * `data` - Data to pass to `c_handler` calls. The argument can be [`null_mut`].
    ///  * `destroy_data` - A [`GClosureNotify`] for `data`. The argument can be [`null_mut`].
    ///  * `connect_flags` - A combination of [`GConnectFlags`].
    ///
    /// # Return Value
    /// The handler ID (always greater than 0).
    pub fn g_signal_connect_data(
        instance: *mut GObject,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: Option<GClosureNotify>,
        connect_flags: GConnectFlags,
    ) -> gulong;
}
