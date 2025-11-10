use crate::{
    gobject::GObject,
    raw::{glib::gulong, gobject::g_signal_handler_disconnect},
};

impl GObject {
    /// Disconnects a handler from an instance so it will not be called during any future or
    /// currently ongoing emissions of the signal it has been connected to
    pub fn signal_handler_disconnect(&self, handler_id: gulong) {
        unsafe { g_signal_handler_disconnect(self.handle, handler_id) };
    }
}
