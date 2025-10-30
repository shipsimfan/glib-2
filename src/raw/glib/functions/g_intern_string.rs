use crate::raw::glib::gchar;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns a canonical representation for string. Interned strings can be compared for
    /// equality by comparing the pointers, instead of using `strcmp`.
    ///
    /// This function must not be used before library constructors have finished running. In
    /// particular, this means it cannot be used to initialize global variables in C++.
    ///
    /// # Parameters
    ///  * `string` - A string. The argument can be [`null`]. The value is a NUL terminated UTF-8
    ///               string.
    ///
    /// # Return Value
    /// A canonical representation for the string. The value is a NUL terminated UTF-8 string.
    pub fn g_intern_string(string: *const gchar) -> *const gchar;
}
