use crate::util::Property;
use std::ffi::CStr;

impl Property for CStr {
    fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.as_ptr().cast_mut().cast()
    }

    unsafe fn from_ptr<'a>(ptr: *mut std::ffi::c_void) -> &'a Self {
        unsafe { CStr::from_ptr(ptr.cast()) }
    }
}
