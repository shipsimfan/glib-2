use crate::{
    glib::{gchar, gpointer},
    gobject::GObject,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{GQuark, g_quark_from_string};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Each object carries around a table of associations from strings to pointers. This function
    /// lets you set an association.
    ///
    /// If the object already had an association with that name, the old association will be
    /// destroyed.
    ///
    /// Internally, the key is converted to a [`GQuark`] using [`g_quark_from_string`]. This means
    /// a copy of key is kept permanently (even after object has been finalized) — so it is
    /// recommended to only use a small, bounded set of values for key in your program, to avoid
    /// the [`GQuark`] storage growing unbounded.
    ///
    /// # Parameters
    ///  * `key` - Name of the key. The value is a NUL terminated UTF-8 string.
    ///  * `data` - Data to associate with that key. The argument can be [`null`].
    pub fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
}
