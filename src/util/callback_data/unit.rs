use crate::util::CallbackData;
use std::{ffi::c_void, ptr::null_mut};

impl CallbackData for () {
    fn into_ptr(self) -> *mut c_void {
        null_mut()
    }

    unsafe fn from_ptr(_: *mut c_void) -> Self {
        ()
    }
}
