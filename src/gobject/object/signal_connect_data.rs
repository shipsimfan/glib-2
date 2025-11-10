use crate::{
    G_CALLBACK,
    gobject::{GConnectFlags, GDataSignalCallback, GObject},
    raw::{glib::gulong, gobject::g_signal_connect_data},
    util::CallbackData,
};
use std::ffi::CStr;

impl GObject {
    /// Connects a [`GDataSignalCallback`] function to a signal for a particular object
    pub fn signal_connect_data<Callback: GDataSignalCallback>(
        &self,
        detailed_signal: &CStr,
        data: Callback::UserData,
        connect_flags: GConnectFlags,
    ) -> gulong {
        unsafe {
            g_signal_connect_data(
                self.handle,
                detailed_signal.as_ptr(),
                G_CALLBACK!(Callback::trampoline),
                data.into_ptr(),
                Some(Callback::drop),
                connect_flags,
            )
        }
    }
}
