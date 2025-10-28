use crate::glib::{GThreadPool, guint};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the number of currently unused threads.
    ///
    /// # Return Value
    /// The number of currently unused threads.
    pub fn g_thread_pool_get_num_unused_threads(pool: *mut GThreadPool) -> guint;
}
