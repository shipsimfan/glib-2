use crate::raw::glib::GMainLoop;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Increases the reference count on a [`GMainLoop`] object by one.
    ///
    /// # Return Value
    /// `loop`.
    pub fn g_main_loop_ref(r#loop: *mut GMainLoop) -> *mut GMainLoop;
}
