use crate::{glib::GBytes, raw};

impl<'a> GBytes<'a> {
    /// Get the underlying handle to the [`raw::glib::GBytes`]
    pub const unsafe fn handle(&self) -> *mut raw::glib::GBytes {
        self.handle
    }
}
