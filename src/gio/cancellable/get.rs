use crate::{gio::GCancellable, raw};

impl GCancellable {
    /// Get the underlying handle to the object
    pub unsafe fn handle(&self) -> *mut raw::gio::GCancellable {
        self.handle
    }
}
