use crate::gobject::GObject;

// rustdoc imports
#[allow(unused_imports)]
use crate::gobject::g_clear_object;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Decreases the reference count of object. When its reference count drops to 0, the object is
    /// finalized (i.e. its memory is freed).
    ///
    /// If the pointer to the [`GObject`] may be reused in future (for example, if it is an
    /// instance variable of another object), it is recommended to clear the pointer to
    /// [`null_mut`] rather than retain a dangling pointer to a potentially invalid [`GObject`]
    /// instance. Use [`g_clear_object`] for this.
    ///
    /// # Parameters
    ///  * `object` - a [`GObject`]
    pub fn g_object_unref(object: *mut GObject);
}
