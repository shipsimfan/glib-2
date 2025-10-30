use crate::{glib::GQuark, raw};

impl GQuark {
    /// Create a new [`GQuark`] from a raw `handle`
    pub unsafe fn new_raw(handle: raw::glib::GQuark) -> Self {
        GQuark { handle }
    }
}
