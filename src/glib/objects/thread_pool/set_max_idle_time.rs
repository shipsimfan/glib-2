use crate::glib::guint;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// This function will set the maximum interval that a thread waiting in the pool for new tasks
    /// can be idle for before being stopped. This function is similar to calling
    /// [`g_thread_pool_stop_unused_threads`] on a regular timeout, except this is done on a per
    /// thread basis.
    ///
    /// By setting interval to 0, idle threads will not be stopped.
    ///
    /// The default value is 15000 (15 seconds).
    ///
    /// # Parameters
    ///  * `interval` - The maximum interval (in milliseconds) a thread can be idle.
    pub fn g_thread_pool_set_max_idle_time(interval: guint);
}
