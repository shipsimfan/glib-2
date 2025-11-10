use crate::util::CallbackData;
use std::{ffi::c_void, mem::ManuallyDrop, rc::Rc, sync::Arc};

impl<T> CallbackData for Rc<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a;

    fn into_ptr(self) -> *mut c_void {
        Rc::into_raw(self).cast_mut().cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Rc::from_raw(ptr.cast()) }
    }

    unsafe fn from_ptr_ref<'a>(ptr: *mut c_void) -> Self::Ref<'a> {
        let rc = ManuallyDrop::new(unsafe { Rc::from_raw(ptr.cast::<T>()) });
        unsafe { &*(rc.as_ref() as *const T) }
    }
}

impl<T> CallbackData for Arc<T> {
    type Ref<'a>
        = &'a T
    where
        Self: 'a;

    fn into_ptr(self) -> *mut c_void {
        Arc::into_raw(self).cast_mut().cast()
    }

    unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        unsafe { Arc::from_raw(ptr.cast()) }
    }

    unsafe fn from_ptr_ref<'a>(ptr: *mut c_void) -> Self::Ref<'a> {
        let rc = ManuallyDrop::new(unsafe { Arc::from_raw(ptr.cast::<T>()) });
        unsafe { &*(rc.as_ref() as *const T) }
    }
}
