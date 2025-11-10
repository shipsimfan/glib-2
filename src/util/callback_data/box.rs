use crate::util::CallbackData;
use std::ffi::c_void;

impl<T> CallbackData for Box<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a;

    fn into_ptr(self) -> *mut c_void {
        Box::into_raw(self).cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Box::from_raw(ptr.cast()) }
    }

    unsafe fn from_ptr_ref<'a>(ptr: *mut c_void) -> Self::Ref<'a> {
        unsafe { &*(ptr.cast()) }
    }
}
