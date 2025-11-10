use crate::{
    gio::GAsyncResult,
    raw,
    util::{CallbackData, NewRaw},
};
use std::ptr::null_mut;

/// Type definition for a function that will be called back when an asynchronous operation within
/// GIO has been completed.
pub trait GAsyncReadyCallback {
    /// The type of data to be passed to the callback
    type UserData: CallbackData;

    /// The object expected to be the `source_object`
    type Object: NewRaw;

    /// Type definition for a function that will be called back when an asynchronous operation within
    /// GIO has been completed. [`GAsyncReadyCallback`] callbacks from [`GTask`] are guaranteed to be
    /// invoked in a later iteration of the thread-default main context (see
    /// [`GMainContext::push_thread_default`]) where the [`GTask`] was created. All other users of
    /// [`GAsyncReadyCallback`] must likewise call it asynchronously in a later iteration of the main
    /// context.
    ///
    /// The asynchronous operation is guaranteed to have held a reference to `source_object` from the
    /// time when the `*_async` function was called, until after this callback returns.
    ///
    /// # Parameters
    ///  * `source_object` - The object the asynchronous operation was started with.
    ///  * `res` - A [`GAsyncResult`].
    ///  * `data` - User data passed to the callback.
    fn callback(source_object: Option<Self::Object>, res: &GAsyncResult, data: Self::UserData);

    /// Converts a [`raw::gio::GAsyncReadyCallback`] into a [`GAsyncReadyCallback`]
    extern "C" fn trampoline(
        source_object: *mut raw::gobject::GObject,
        res: *mut raw::gio::GAsyncResult,
        data: raw::glib::gpointer,
    ) {
        let source_object = if source_object == null_mut() {
            None
        } else {
            Some(unsafe { Self::Object::new_raw(source_object, false) })
        };
        let res = unsafe { GAsyncResult::new_raw(res, false) };
        let data = unsafe { Self::UserData::from_ptr(data) };

        Self::callback(source_object, &res, data);
    }
}
