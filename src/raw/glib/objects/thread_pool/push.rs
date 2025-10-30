use crate::raw::glib::{GError, GThreadPool, gboolean, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{FALSE, TRUE, g_thread_pool_new};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Inserts data into the list of tasks to be executed by pool.
    ///
    /// When the number of currently running threads is lower than the maximal allowed number of
    /// threads, a new thread is started (or reused) with the properties given to
    /// [`g_thread_pool_new`]. Otherwise, `data` stays in the queue until a thread in this pool
    /// finishes its previous task and processes `data`.
    ///
    /// `error` can be [`null_mut`] to ignore errors, or not [`null_mut`] to report errors. An
    /// error can only occur when a new thread couldn’t be created. In that case `data` is simply
    /// appended to the queue of work to do.
    ///
    /// # Parameters
    ///  * `data` - A new task for pool. The argument can be [`null_mut`].
    ///  * `error` - The return location for a recoverable error. The argument can be [`null_mut`].
    ///              If the return location is not [`null_mut`], then you must initialize it to a
    ///              [`null_mut`] `*mut GError`. The argument will be left initialized to
    ///              [`null_mut`] by the method if there are no errors. In case of error, the
    ///              argument will be set to a newly allocated [`GError`]; the caller will take
    ///              ownership of the data, and be responsible for freeing it.
    ///
    /// # Return Value
    /// [`TRUE`] on success, [`FALSE`] if an error occurred.
    pub fn g_thread_pool_push(
        pool: *mut GThreadPool,
        data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean;
}
