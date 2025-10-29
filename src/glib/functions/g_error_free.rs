use crate::glib::GError;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Frees a [`GError`] and associated resources.
    pub fn g_error_free(error: *mut GError);
}
