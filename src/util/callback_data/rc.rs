use crate::util::CallbackData;
use std::{ffi::c_void, rc::Rc, sync::Arc};

impl<T> CallbackData for Rc<T> {
    fn into_ptr(self) -> *mut c_void {
        Rc::into_raw(self).cast_mut().cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Rc::from_raw(ptr.cast()) }
    }
}

impl<T> CallbackData for Arc<T> {
    fn into_ptr(self) -> *mut c_void {
        Arc::into_raw(self).cast_mut().cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Arc::from_raw(ptr.cast()) }
    }
}
