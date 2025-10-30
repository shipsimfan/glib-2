use crate::raw::glib::{GThreadPool, gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::TRUE;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Frees all resources allocated for pool.
    ///
    /// If `immediate` is [`TRUE`], no new task is processed for pool. Otherwise pool is not freed
    /// before the last task is processed. Note however, that no thread of this pool is interrupted
    /// while processing a task. Instead at least all still running threads can finish their tasks
    /// before the pool is freed.
    ///
    /// If `wait` is [`TRUE`], this function does not return before all tasks to be processed
    /// (dependent on immediate, whether all or only the currently running) are ready. Otherwise
    /// this function returns immediately.
    ///
    /// After calling this function pool must not be used anymore.
    ///
    /// # Parameters
    ///  * `immediate` - Should pool shut down immediately?
    ///  * `wait` - Should the function wait for all tasks to be finished?
    pub fn g_thread_pool_free(pool: *mut GThreadPool, immediate: gboolean, wait: gboolean);
}
