use crate::{glib::GPtrArray, raw};
use std::ptr::null_mut;

impl<'a, T> GPtrArray<'a, T> {
    /// Create a new [`GPtrArray`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::glib::GPtrArray) -> Self {
        assert_ne!(handle, null_mut());
        let ptr_array = unsafe { &*handle };
        let slice =
            unsafe { std::slice::from_raw_parts(ptr_array.data.cast(), ptr_array.len as _) };
        GPtrArray { handle, slice }
    }
}
