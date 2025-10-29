use crate::{glib::gchar, gobject::GObject};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Sets properties on an object.
    ///
    /// The same caveats about passing integer literals as varargs apply as with [`g_object_new`].
    /// In particular, any integer literals set as the values for properties of type `#gint64` or
    /// `#guint64` must be 64 bits wide, using the [`G_GINT64_CONSTANT`] or [`G_GUINT64_CONSTANT`]
    /// macros.
    ///
    /// Note that the “notify” signals are queued and only emitted (in reverse order) after all
    /// properties have been set. See [`g_object_freeze_notify`].
    ///
    /// # Parameters
    ///  * `first_property_name` - Name of the first property to set. The value is a NUL terminated
    ///                            UTF-8 string.
    ///  * `...` - Value for the first property, followed optionally by more name/value pairs,
    ///            followed by [`null_mut`].
    pub fn g_object_set(object: *mut GObject, first_property_name: *const gchar, ...);
}
