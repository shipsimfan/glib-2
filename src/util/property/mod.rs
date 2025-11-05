use std::ffi::c_void;

mod cstr;
mod gbytes;

/// An item that can be used as a parameter in a [`GObject`]
pub trait Property {
    /// Convert this item into a pointer
    fn as_ptr(&self) -> *mut c_void;

    /// Convert `ptr` into this item
    unsafe fn from_ptr<'a>(ptr: *mut c_void) -> &'a Self;
}
