use crate::glib::{GFunc, gboolean, gpointer};

mod free;
mod get_max_threads;
mod get_max_unused_threads;
mod get_num_threads;
mod get_num_unused_threads;
mod new;
mod push;
mod set_max_idle_time;
mod set_max_threads;
mod set_max_unused_threads;
mod stop_unused_threads;
mod unprocessed;

pub use free::g_thread_pool_free;
pub use get_max_threads::g_thread_pool_get_max_threads;
pub use get_max_unused_threads::g_thread_pool_get_max_unused_threads;
pub use get_num_threads::g_thread_pool_get_num_threads;
pub use get_num_unused_threads::g_thread_pool_get_num_unused_threads;
pub use new::g_thread_pool_new;
pub use push::g_thread_pool_push;
pub use set_max_idle_time::g_thread_pool_set_max_idle_time;
pub use set_max_threads::g_thread_pool_set_max_threads;
pub use set_max_unused_threads::g_thread_pool_set_max_unused_threads;
pub use stop_unused_threads::g_thread_pool_stop_unused_threads;
pub use unprocessed::g_thread_pool_unprocessed;

/// The [`GThreadPool`] struct represents a thread pool.
///
/// A thread pool is useful when you wish to asynchronously fork out the execution of work and
/// continue working in your own thread. If that will happen often, the overhead of starting and
/// destroying a thread each time might be too high. In such cases reusing already started threads
/// seems like a good idea. And it indeed is, but implementing this can be tedious and error-prone.
///
/// Therefore GLib provides thread pools for your convenience. An added advantage is, that the
/// threads can be shared between the different subsystems of your program, when they are using
/// GLib.
///
/// To create a new thread pool, you use [`g_thread_pool_new`]. It is destroyed by
/// [`g_thread_pool_free`].
///
/// If you want to execute a certain task within a thread pool, use [`g_thread_pool_push`].
///
/// To get the current number of running threads you call [`g_thread_pool_get_num_threads`]. To get
/// the number of still unprocessed tasks you call [`g_thread_pool_unprocessed`]. To control the
/// maximum number of threads for a thread pool, you use [`g_thread_pool_get_max_threads`] and
/// [`g_thread_pool_set_max_threads`].
///
/// Finally you can control the number of unused threads, that are kept alive by GLib for future
/// use. The current number can be fetched with [`g_thread_pool_get_num_unused_threads`]. The
/// maximum number can be controlled by [`g_thread_pool_get_max_unused_threads`] and
/// [`g_thread_pool_set_max_unused_threads`]. All currently unused threads can be stopped by
/// calling [`g_thread_pool_stop_unused_threads`].
#[repr(C)]
pub struct GThreadPool {
    /// The function to execute in the threads of this pool.
    pub func: GFunc,

    /// The user data for the threads of this pool.
    pub user_data: gpointer,

    /// Are all threads exclusive to this pool.
    pub exclusive: gboolean,
}
