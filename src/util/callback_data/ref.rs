use crate::util::CallbackData;
use std::ffi::c_void;

impl<T> CallbackData for &'static T {
    type Ref<'a>
        = &'static T
    where
        Self: 'a;

    fn into_ptr(self) -> *mut c_void {
        self as *const _ as _
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { &*(ptr.cast()) }
    }

    unsafe fn from_ptr_ref<'a>(ptr: *mut c_void) -> Self::Ref<'a> {
        unsafe { &*(ptr.cast()) }
    }
}
