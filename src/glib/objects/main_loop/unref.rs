use crate::glib::GMainLoop;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Decreases the reference count on a [`GMainLoop`] object by one.
    ///
    /// If the result is zero, the loop and all associated memory are freed.
    pub fn g_main_loop_unref(r#loop: *mut GMainLoop);
}
