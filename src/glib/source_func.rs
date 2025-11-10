use crate::{raw, util::CallbackData};

/// A callback function for many items
pub trait GSourceFunc {
    /// The type of data to be passed to the callback
    type UserData: CallbackData;

    /// The callback with user data passed
    fn callback(data: Self::UserData);

    /// Converts a [`raw::glib::GSourceFunc`] into a [`GSourceFunc`]
    extern "C" fn trampoline(data: raw::glib::gpointer) -> raw::glib::gboolean {
        let data = unsafe { Self::UserData::from_ptr(data) };

        Self::callback(data);

        raw::glib::G_SOURCE_REMOVE
    }
}
