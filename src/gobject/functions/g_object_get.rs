use crate::{glib::gchar, gobject::GObject};

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Gets properties of an object.
    ///
    /// In general, a copy is made of the property contents and the caller is responsible for
    /// freeing the memory in the appropriate manner for the type, for instance by calling
    /// [`g_free`] or [`g_object_unref`].
    pub fn g_object_get(object: *mut GObject, first_property_name: *const gchar, ...);
}
