use crate::gobject::GObject;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Clears a reference to a [`GObject`].
    ///
    /// `object_ptr` must not be [`null_mut`].
    ///
    /// If the reference is [`null_mut`] then this function does nothing. Otherwise, the reference
    /// count of the object is decreased and the pointer is set to [`null_mut`].
    ///
    /// A macro is also included that allows this function to be used without pointer casts.
    ///
    /// # Parameters
    ///  * `object_ptr` - A pointer to a [`GObject`] reference.
    pub fn g_clear_object(object_ptr: *mut *mut GObject);
}
