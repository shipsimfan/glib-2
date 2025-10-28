use crate::glib::guint;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::g_thread_pool_new;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Determine the approximate number of threads that the system will schedule simultaneously
    /// for this process. This is intended to be used as a parameter to [`g_thread_pool_new`] for
    /// CPU bound tasks and similar cases.
    ///
    /// # Return Value
    /// Number of schedulable threads, always greater than 0.
    pub fn g_get_num_processors() -> guint;
}
