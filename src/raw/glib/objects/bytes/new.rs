use crate::raw::glib::{GBytes, gconstpointer, gsize};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Creates a new [`GBytes`] from data.
    ///
    /// data is copied. If size is 0, data may be [`null`].
    ///
    /// As an optimization, [`g_bytes_new`] may avoid an extra allocation by copying the data
    /// within the resulting bytes structure if sufficiently small.
    ///
    /// # Parameters
    ///  * `data` - the data to be used for the bytes. The length of the array is specified in the
    ///             `size` argument.
    ///  * `size` - The size of data.
    ///
    /// # Return Value
    /// A new [`GBytes`].
    pub fn g_bytes_new(data: gconstpointer, size: gsize) -> *mut GBytes;
}
