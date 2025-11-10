use crate::raw::glib::{GMainContext, GSourceFunc, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{
    G_PRIORITY_DEFAULT, G_SOURCE_CONTINUE, G_SOURCE_REMOVE, g_main_context_default,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Invokes a function in such a way that context is owned during the invocation of function.
    ///
    /// If `context` is [`null_mut`] then the global-default main context — as returned by
    /// [`g_main_context_default`] — is used.
    ///
    /// If `context` is owned by the current thread, function is called directly. Otherwise, if
    /// `context` is the thread-default main context of the current thread and
    /// [`g_main_context_acquire`] succeeds, then function is called and [`g_main_context_release`]
    /// is called afterwards.
    ///
    /// In any other case, an idle source is created to call function and that source is attached
    /// to context (presumably to be run in another thread). The idle source is attached with
    /// [`G_PRIORITY_DEFAULT`] priority. If you want a different priority, use
    /// [`g_main_context_invoke_full`].
    ///
    /// Note that, as with normal idle functions, function should probably return
    /// [`G_SOURCE_REMOVE`]. If it returns [`G_SOURCE_CONTINUE`], it will be continuously run in a
    /// loop (and may prevent this call from returning).
    pub fn g_main_context_invoke(context: *mut GMainContext, function: GSourceFunc, data: gpointer);
}
