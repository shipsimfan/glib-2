use crate::util::CallbackData;
use std::ffi::c_void;

impl<T> CallbackData for Box<T> {
    fn into_ptr(self) -> *mut c_void {
        Box::into_raw(self).cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Box::from_raw(ptr.cast()) }
    }
}
