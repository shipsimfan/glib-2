use crate::{
    glib::GBytes,
    raw::{self, glib::g_bytes_new},
};
use std::{marker::PhantomData, ptr::null_mut};

impl<'a> GBytes<'a> {
    /// Creates a new GBytes from `data`
    pub fn new(data: &[u8]) -> Self {
        let handle = unsafe { g_bytes_new(data.as_ptr().cast(), data.len()) };
        unsafe { GBytes::new_raw(handle, true) }
    }

    /// Create a new [`GBytes`] from a raw `handle`
    pub unsafe fn new_raw(handle: *mut raw::glib::GBytes, owned: bool) -> Self {
        assert_ne!(handle, null_mut());
        GBytes {
            handle,
            owned: if owned { Some(PhantomData) } else { None },
        }
    }
}
