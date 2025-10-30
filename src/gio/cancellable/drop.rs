use crate::{gio::GCancellable, raw::gobject::g_object_unref};

impl Drop for GCancellable {
    fn drop(&mut self) {
        unsafe { g_object_unref(self.handle) };
    }
}
