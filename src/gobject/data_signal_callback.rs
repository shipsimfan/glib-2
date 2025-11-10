use crate::{raw, util::CallbackData};

/// A callback to data signal
pub trait GDataSignalCallback {
    /// The type of data to be passed to the callback
    type UserData: CallbackData;

    /// Called when the signal emits data
    fn callback(data: &Self::UserData);

    /// Converts a [`raw::gobject::GCallback`] into a [`GDataSignalCallback`]
    extern "C" fn trampoline(data: raw::glib::gpointer) {
        let data = unsafe { Self::UserData::from_ptr(data) };

        Self::callback(&data);

        // Convert the data back into a pointer to prevent dropping
        data.into_ptr();
    }

    /// Implements a [`raw::gobject::GClosureNotify`] for dropping the user data
    extern "C" fn drop(data: raw::glib::gpointer, _: *mut raw::gobject::GClosure) {
        drop(unsafe { Self::UserData::from_ptr(data) });
    }
}
