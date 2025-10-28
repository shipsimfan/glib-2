use crate::{gio::GAsyncResult, glib::gpointer, gobject::GObject};

// rustdoc imports
#[allow(unused_imports)]
use crate::{gio::GTask, glib::g_main_context_push_thread_default};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Type definition for a function that will be called back when an asynchronous operation within
/// GIO has been completed. [`GAsyncReadyCallback`] callbacks from [`GTask`] are guaranteed to be
/// invoked in a later iteration of the thread-default main context (see
/// [`g_main_context_push_thread_default`]) where the [`GTask`] was created. All other users of
/// [`GAsyncReadyCallback`] must likewise call it asynchronously in a later iteration of the main
/// context.
///
/// The asynchronous operation is guaranteed to have held a reference to `source_object` from the
/// time when the `*_async` function was called, until after this callback returns.
///
/// # Parameters
///  * `source_object` - The object the asynchronous operation was started with. The argument can
///                      be [`null_mut`].
///  * `res` - A [`GAsyncResult`].
///  * `data` - User data passed to the callback. The argument can be [`null_mut`].
pub type GAsyncReadyCallback =
    extern "C" fn(source_object: *mut GObject, res: *mut GAsyncResult, data: gpointer);
