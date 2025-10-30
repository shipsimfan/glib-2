use crate::raw::glib::{GError, GFunc, GThreadPool, gboolean, gint, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{
    FALSE, TRUE, g_get_num_processors, g_thread_pool_free, g_thread_pool_push,
    g_thread_pool_set_max_idle_time,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// This function creates a new thread pool.
    ///
    /// Whenever you call [`g_thread_pool_push`], either a new thread is created or an unused one
    /// is reused. At most `max_threads` threads are running concurrently for this thread pool.
    /// `max_threads = -1` allows unlimited threads to be created for this thread pool. The newly
    /// created or reused thread now executes the function `func` with the two arguments. The first
    /// one is the parameter to [`g_thread_pool_push`] and the second one is `user_data`.
    ///
    /// Pass [`g_get_num_processors`] to `max_threads` to create as many threads as there are
    /// logical processors on the system. This will not pin each thread to a specific processor.
    ///
    /// The parameter exclusive determines whether the thread pool owns all threads exclusive or
    /// shares them with other thread pools. If `exclusive` is [`TRUE`], `max_threads` threads are
    /// started immediately and they will run exclusively for this thread pool until it is
    /// destroyed by [`g_thread_pool_free`]. If `exclusive` is [`FALSE`], threads are created when
    /// needed and shared between all non-exclusive thread pools. This implies that `max_threads`
    /// may not be -1 for exclusive thread pools. Besides, exclusive thread pools are not affected
    /// by [`g_thread_pool_set_max_idle_time`] since their threads are never considered idle and
    /// returned to the global pool.
    ///
    /// Note that the threads used by exclusive thread pools will all inherit the scheduler
    /// settings of the current thread while the threads used by non-exclusive thread pools will
    /// inherit the scheduler settings from the first thread that created such a thread pool.
    ///
    /// At least one thread will be spawned when this function is called, either to create the
    /// `max_threads` exclusive threads, or to preserve the scheduler settings of the current
    /// thread for future spawns.
    ///
    /// `error` can be [`null_mut`] to ignore errors, or not [`null_mut`] to report errors. An
    /// error can only occur when `exclusive` is set to [`TRUE`] and not all `max_threads` threads
    /// could be created. Note, even in case of error a valid [`GThreadPool`] is returned.
    ///
    /// # Parameters
    ///  * `func` - A function to execute in the threads of the new thread pool.
    ///  * `user_data` - User data that is handed over to `func` every time it is called. The
    ///                  argument can be [`null_mut`].
    ///  * `max_threads` - The maximal number of threads to execute concurrently in the new thread
    ///                    pool, -1 means no limit.
    ///  * `exclusive` - Should this thread pool be exclusive?
    ///  * `error` - The return location for a recoverable error. The argument can be [`null_mut`].
    ///              If the return location is not [`null_mut`], then you must initialize it to a
    ///              [`null_mut`] `*mut GError`. The argument will be left initialized to
    ///              [`null_mut`] by the function if there are no errors. In case of error, the
    ///              argument will be set to a newly allocated [`GError`]; the caller will take
    ///              ownership of the data, and be responsible for freeing it.
    ///
    /// # Return Value
    /// The new [`GThreadPool`].
    pub fn g_thread_pool_new(
        func: GFunc,
        user_data: gpointer,
        max_threads: gint,
        exclusive: gboolean,
        error: *mut *mut GError,
    ) -> *mut GThreadPool;
}
