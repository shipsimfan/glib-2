use crate::glib::{GQuark, gchar};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the [`GQuark`] identifying the given string. If the string does not currently have an
    /// associated [`GQuark`], a new [`GQuark`] is created, using a copy of the string.
    ///
    /// This function must not be used before library constructors have finished running. In
    /// particular, this means it cannot be used to initialize global variables in C++.
    ///
    /// # Parameters
    ///  * `string` - A string. The argument can be [`null`]. The value is a NUL terminated UTF-8
    ///               string.
    ///
    /// # Return Value
    /// The [`GQuark`] identifying the string, or 0 if string is [`null`].
    pub fn g_quark_from_string(string: *const gchar) -> GQuark;
}
