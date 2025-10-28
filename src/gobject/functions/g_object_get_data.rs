use crate::{
    glib::{gchar, gpointer},
    gobject::GObject,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::gobject::g_object_set_data;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Gets a named field from the objects table of associations (see [`g_object_set_data`]).
    ///
    /// # Parameters
    ///  * `key` - Name of the key for that association. The value is a NUL terminated UTF-8
    ///            string.
    ///
    /// # Return Value
    /// The data if found, or [`null_mut`] if no such data exists.
    pub fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
}
