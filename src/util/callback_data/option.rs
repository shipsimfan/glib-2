use crate::util::CallbackData;
use std::{ffi::c_void, ptr::null_mut};

impl<T: CallbackData> CallbackData for Option<T> {
    fn into_ptr(self) -> *mut c_void {
        match self {
            Some(value) => value.into_ptr(),
            None => null_mut(),
        }
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        if ptr == null_mut() {
            None
        } else {
            Some(unsafe { T::from_ptr(ptr) })
        }
    }
}
