use crate::{gio::GAsyncResult, gobject::GObject, raw, util::NewRaw};

impl GAsyncResult {
    /// Create a new [`GAsyncResult`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::gio::GAsyncResult, owned: bool) -> Self {
        let object = unsafe { GObject::new_raw(handle, owned) };
        GAsyncResult { object }
    }
}

impl NewRaw for GAsyncResult {
    unsafe fn new_raw(handle: *mut raw::gobject::GObject, owned: bool) -> Self {
        unsafe { GAsyncResult::new_raw(handle, owned) }
    }
}
