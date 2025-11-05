use crate::raw;

/// An object which can be created from just a raw pointer
pub trait NewRaw {
    /// Create a new object from a raw `handle`
    unsafe fn new_raw(handle: *mut raw::gobject::GObject, owned: bool) -> Self;
}
