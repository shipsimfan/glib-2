use crate::util::CallbackData;
use std::ffi::c_void;

impl<T> CallbackData for &'static T {
    fn into_ptr(self) -> *mut c_void {
        self as *const _ as _
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { &*(ptr.cast()) }
    }
}

impl<T> CallbackData for &'static mut T {
    fn into_ptr(self) -> *mut c_void {
        self as *const _ as _
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { &mut *(ptr.cast()) }
    }
}
