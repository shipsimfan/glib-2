use crate::raw::glib::{GMainContext, GMainLoop};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the [`GMainContext`] of loop.
    ///
    /// # Return Value
    /// The [`GMainContext`] of loop.
    pub fn g_main_loop_get_context(r#loop: *mut GMainLoop) -> *mut GMainContext;
}
