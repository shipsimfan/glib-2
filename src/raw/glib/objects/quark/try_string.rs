use crate::raw::glib::{GQuark, gchar};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{g_quark_from_static_string, g_quark_from_string};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the [`GQuark`] associated with the given string, or 0 if string is [`null`] or it has
    /// no associated [`GQuark`].
    ///
    /// If you want the [`GQuark`] to be created if it doesn’t already exist, use
    /// [`g_quark_from_string`] or [`g_quark_from_static_string`].
    ///
    /// This function must not be used before library constructors have finished running.
    ///
    /// # Parameters
    ///  * `string` - A string. The argument can be [`null`]. The value is a NUL terminated UTF-8
    ///               string.
    ///
    /// # Return Value
    /// The [`GQuark`] associated with the string, or 0 if string is [`null`] or there is no
    /// [`GQuark`] associated with it.
    pub fn g_quark_try_string(string: *const gchar) -> GQuark;
}
