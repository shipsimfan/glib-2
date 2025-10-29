use crate::glib::GMainContext;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Decreases the reference count on a [`GMainContext`] object by one. If the result is zero,
    /// free the context and free all associated memory.
    pub fn g_main_context_unref(context: *mut GMainContext);
}
