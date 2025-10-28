use crate::glib::{GThreadPool, guint};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the maximal allowed number of unused threads.
    ///
    /// # Return Value
    /// The maximal number of unused threads.
    pub fn g_thread_pool_get_max_unused_threads(pool: *mut GThreadPool) -> guint;
}
