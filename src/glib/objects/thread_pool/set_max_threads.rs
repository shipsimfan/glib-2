use crate::glib::{GError, GThreadPool, gboolean, gint};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{FALSE, TRUE, g_thread_pool_new, g_thread_pool_push};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets the maximal allowed number of threads for pool. A value of -1 means that the maximal
    /// number of threads is unlimited. If `pool` is an exclusive thread pool, setting the maximal
    /// number of threads to -1 is not allowed.
    ///
    /// Setting `max_threads` to 0 means stopping all work for `pool`. It is effectively frozen
    /// until `max_threads` is set to a non-zero value again.
    ///
    /// A thread is never terminated while calling `func`, as supplied by [`g_thread_pool_new`].
    /// Instead the maximal number of threads only has effect for the allocation of new threads in
    /// [`g_thread_pool_push`]. A new thread is allocated, whenever the number of currently running
    /// threads in pool is smaller than the maximal number.
    ///
    /// `error` can be [`null_mut`] to ignore errors, or not [`null_mut`] to report errors. An
    /// error can only occur when a new thread couldn’t be created.
    ///
    /// # Parameters
    ///  * `max_threads` - A new maximal number of threads for `pool`, or -1 for unlimited.
    ///  * `error` - The return location for a recoverable error. The argument can be [`null_mut`].
    ///              If the return location is not [`null_mut`], then you must initialize it to a
    ///              [`null_mut`] `*const GError`. The argument will be left initialized to
    ///              [`null_mut`] by the method if there are no errors. In case of error, the
    ///              argument will be set to a newly allocated GError; the caller will take
    ///              ownership of the data, and be responsible for freeing it.
    ///
    /// # Return Value
    /// [`TRUE`] on success, [`FALSE`] if an error occurred.
    pub fn g_thread_pool_set_max_threads(
        pool: *mut GThreadPool,
        max_threads: gint,
        error: *mut *mut GError,
    ) -> gboolean;
}
