use crate::glib::{GQuark, gchar};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the string associated with the given [`GQuark`].
    ///
    /// # Parameters
    ///  * `quark` - A [`GQuark`].
    ///
    /// # Return Value
    /// The string associated with the [`GQuark`]. The value is a NUL terminated UTF-8 string.
    pub fn g_quark_to_string(quark: GQuark) -> *const gchar;
}
