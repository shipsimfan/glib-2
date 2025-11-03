use crate::{glib::GMainContext, raw};

impl GMainContext {
    /// Get the underlying handle to the main context
    pub const unsafe fn handle(&self) -> *mut raw::glib::GMainContext {
        self.handle
    }
}
