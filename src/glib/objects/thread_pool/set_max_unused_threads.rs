use crate::glib::gint;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets the maximal number of unused threads to `max_threads`. If `max_threads` is -1, no
    /// limit is imposed on the number of unused threads.
    ///
    /// The default value is 8 since GLib 2.84. Previously the default value was 2.
    ///
    /// # Parameters
    ///  * `max_threads` - Maximal number of unused threads.
    pub fn g_thread_pool_set_max_unused_threads(max_threads: gint);
}
