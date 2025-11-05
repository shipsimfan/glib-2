use crate::{gobject::GObject, raw, util::NewRaw};
use std::ptr::null_mut;

impl GObject {
    /// Create a new [`GObject`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::gobject::GObject, owned: bool) -> Self {
        assert_ne!(handle, null_mut());
        GObject { handle, owned }
    }
}

impl NewRaw for GObject {
    unsafe fn new_raw(handle: *mut raw::gobject::GObject, owned: bool) -> Self {
        unsafe { GObject::new_raw(handle, owned) }
    }
}
