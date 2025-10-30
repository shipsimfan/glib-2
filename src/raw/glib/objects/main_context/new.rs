use crate::raw::glib::GMainContext;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Creates a new GMainContext structure.
    ///
    /// # Return Value
    /// The new main context.
    pub fn g_main_context_new() -> *mut GMainContext;
}
