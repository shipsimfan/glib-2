use crate::util::CallbackData;
use std::{ffi::c_void, ptr::null_mut};

impl<T: CallbackData> CallbackData for Option<T> {
    type Ref<'a>
        = Option<T::Ref<'a>>
    where
        Self: 'a;

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

    unsafe fn from_ptr_ref<'a>(ptr: *mut c_void) -> Self::Ref<'a> {
        if ptr == null_mut() {
            None
        } else {
            Some(unsafe { T::from_ptr_ref(ptr) })
        }
    }
}
