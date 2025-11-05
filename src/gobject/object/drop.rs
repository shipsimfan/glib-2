use crate::{gobject::GObject, raw::gobject::g_object_unref};

impl Drop for GObject {
    fn drop(&mut self) {
        if self.owned {
            unsafe { g_object_unref(self.handle) };
        }
    }
}
