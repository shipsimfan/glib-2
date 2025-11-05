use crate::{gio::GCancellable, gobject::GObject, raw, util::NewRaw};

impl GCancellable {
    /// Create a new [`GCancellable`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::gio::GCancellable, owned: bool) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        GCancellable { object }
    }
}

impl NewRaw for GCancellable {
    unsafe fn new_raw(handle: *mut raw::gobject::GObject, owned: bool) -> Self {
        unsafe { GCancellable::new_raw(handle, owned) }
    }
}
