use crate::{gio::GFile, glib::gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{FALSE, g_main_context_push_thread_default};

#[link(name = "gio-2.0")]
unsafe extern "C" {
    /// Checks if `file` supports thread-default main contexts (see
    /// [`g_main_context_push_thread_default`]). If this returns [`FALSE`], you cannot perform
    /// asynchronous operations on file in a thread that has a thread-default context.
    ///
    /// # Return Value
    /// Whether or not `file` supports thread-default contexts.
    pub fn g_file_supports_thread_contexts(file: *mut GFile) -> gboolean;
}
