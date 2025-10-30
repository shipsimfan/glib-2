// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::g_timeout_add;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Stops all currently unused threads. This does not change the maximal number of unused
    /// threads. This function can be used to regularly stop all unused threads e.g. from
    /// [`g_timeout_add`].
    pub fn g_thread_pool_stop_unused_threads();
}
