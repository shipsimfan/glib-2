use crate::{gobject::GObject, raw::gobject::g_object_set, util::Property};
use std::{ffi::CStr, ptr::null_mut};

impl GObject {
    /// Sets properties on an object.
    pub unsafe fn set<T: Property + ?Sized>(&self, property: &CStr, value: &T) {
        unsafe {
            g_object_set(
                self.handle,
                property.as_ptr(),
                value.as_ptr(),
                null_mut::<()>(),
            );
        }
    }
}
