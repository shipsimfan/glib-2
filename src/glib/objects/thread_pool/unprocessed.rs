use crate::glib::{GThreadPool, guint};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Returns the number of tasks still unprocessed in `pool`.
    ///
    /// # Return Value
    /// The number of unprocessed tasks.
    pub fn g_thread_pool_unprocessed(pool: *mut GThreadPool) -> guint;
}
