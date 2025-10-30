use crate::raw::glib::GMainContext;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Increases the reference count on a [`GMainContext`] object by one.
    ///
    /// # Return Value
    /// The `context` that was passed in.
    pub fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
}
