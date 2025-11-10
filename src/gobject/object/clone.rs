use crate::{gobject::GObject, raw::gobject::g_object_ref};

impl Clone for GObject {
    fn clone(&self) -> Self {
        let handle = unsafe { g_object_ref(self.handle) };
        unsafe { GObject::new_raw(handle, true) }
    }
}
