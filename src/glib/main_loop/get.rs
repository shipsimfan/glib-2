use crate::{glib::GMainLoop, raw};

impl GMainLoop {
    /// Get the underlying handle to the main loop
    pub const unsafe fn handle(&self) -> *mut raw::glib::GMainLoop {
        self.handle
    }
}
