use crate::raw::glib::{GBytes, gconstpointer, gsize};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Get the byte data in the [`GBytes`].
    ///
    /// This data should not be modified.
    ///
    /// This function will always return the same pointer for a given [`GBytes`].
    ///
    /// [`null_mut`] may be returned if `size` is 0. This is not guaranteed, as the [`GBytes`] may
    /// represent an empty string with data not [`null_mut`] and `size` as 0. [`null_mut`] will not
    /// be returned if size is non-zero.
    ///
    /// # Parameters
    ///  * `size` - Location to return size of byte data. The argument will be set by the function.
    ///             The argument can be [`null_mut`].
    ///
    /// # Return Value
    /// a pointer to the byte data. The length of the array is in the `size` argument.
    pub fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
}
