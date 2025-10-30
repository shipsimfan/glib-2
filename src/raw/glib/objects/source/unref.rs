use crate::raw::glib::GSource;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Decreases the reference count of a source by one.
    ///
    /// If the resulting reference count is zero the source and associated memory will be
    /// destroyed.
    pub fn g_source_unref(source: *mut GSource);
}
