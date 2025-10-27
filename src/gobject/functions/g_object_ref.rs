use crate::gobject::GObject;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Increases the reference count of object.
    ///
    /// Since GLib 2.56, if [`GLIB_VERSION_MAX_ALLOWED`] is 2.56 or greater, the type of object
    /// will be propagated to the return type, so any casting the caller needs to do on the return
    /// type must be explicit.
    ///
    /// # Return Value
    /// The caller of the method takes ownership of the returned data, and is responsible for
    /// freeing it.
    pub fn g_object_ref(object: *mut GObject) -> *mut GObject;
}
