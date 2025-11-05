use std::ffi::c_void;

mod r#box;
mod option;
mod rc;
mod r#ref;
mod unit;

/// An item that can be used in an asynchronous callback as user data
pub trait CallbackData {
    /// Convert this item into a pointer
    fn into_ptr(self) -> *mut c_void;

    /// Convert `ptr` into this item
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
}
