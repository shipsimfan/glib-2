use crate::{gobject::GObject, raw};

impl GObject {
    /// Get the underlying handle to the object
    pub unsafe fn handle(&self) -> *mut raw::gobject::GObject {
        self.handle
    }
}
