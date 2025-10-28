use crate::glib::{GThreadPool, guint};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the number of threads currently running in `pool`.
    ///
    /// # Return Value
    /// The number of threads currently running.
    pub fn g_thread_pool_get_num_threads(pool: *mut GThreadPool) -> guint;
}
