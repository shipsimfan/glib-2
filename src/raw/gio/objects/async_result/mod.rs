use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{gio::GAsyncReadyCallback, glib::g_main_context_push_thread_default};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// [`GAsyncResult`] provides a base class for implementing asynchronous function results.
///
/// Asynchronous operations are broken up into two separate operations which are chained together
/// by a [`GAsyncReadyCallback`]. To begin an asynchronous operation, provide a
/// [`GAsyncReadyCallback`] to the asynchronous function. This callback will be triggered when the
/// operation has completed, and must be run in a later iteration of the thread-default main
/// context (see [`g_main_context_push_thread_default`]) from where the operation was initiated. It
/// will be passed a [`GAsyncResult`] instance filled with the details of the operation’s success
/// or failure, the object the asynchronous function was started for and any error codes returned.
/// The asynchronous callback function is then expected to call the corresponding `_finish`
/// function, passing the object the function was called for, the [`GAsyncResult`] instance, and
/// (optionally) an error to grab any error conditions that may have occurred.
///
/// The `_finish` function for an operation takes the generic result (of type [`GAsyncResult`]) and
/// returns the specific result that the operation in question yields (e.g. a [`GFileEnumerator`]
/// for a “enumerate children” operation). If the result or error status of the operation is not
/// needed, there is no need to call the `_finish` function; GIO will take care of cleaning up the
/// result and error information after the [`GAsyncReadyCallback`] returns. You can pass
/// [`null_mut`] for the [`GAsyncReadyCallback`] if you don’t need to take any action at all after
/// the operation completes. Applications may also take a reference to the [`GAsyncResult`] and
/// call `_finish` later; however, the `_finish` function may be called at most once.
pub type GAsyncResult = c_void;
