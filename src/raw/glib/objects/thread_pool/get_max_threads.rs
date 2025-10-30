use crate::raw::glib::{GThreadPool, guint};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the maximal number of threads for `pool`.
    ///
    /// # Return Value
    /// The maximal number of threads.
    pub fn g_thread_pool_get_max_threads(pool: *mut GThreadPool) -> guint;
}
