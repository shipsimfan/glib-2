use crate::{glib::GQuark, raw};

impl GQuark {
    /// Get the underlying handle to the quark
    pub unsafe fn handle(&self) -> raw::glib::GQuark {
        self.handle
    }
}
