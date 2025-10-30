use crate::raw::glib::{GQuark, gchar};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::g_quark_from_string;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the [`GQuark`] identifying the given (static) string. If the string does not currently
    /// have an associated [`GQuark`], a new [`GQuark`] is created, linked to the given string.
    ///
    /// Note that this function is identical to [`g_quark_from_string`] except that if a new
    /// [`GQuark`] is created the string itself is used rather than a copy. This saves memory, but
    /// can only be used if the string will continue to exist until the program terminates. It can
    /// be used with statically allocated strings in the main program, but not with statically
    /// allocated memory in dynamically loaded modules, if you expect to ever unload the module
    /// again (e.g. do not use this function in GTK theme engines).
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
    pub fn g_quark_from_static_string(string: *const gchar) -> GQuark;
}
