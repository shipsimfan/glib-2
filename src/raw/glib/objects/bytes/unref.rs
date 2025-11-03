use crate::raw::glib::GBytes;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Releases a reference on bytes.
    ///
    /// This may result in the bytes being freed. If bytes is [`null_mut`], it will return
    /// immediately.
    ///
    /// # Parameters
    ///  * `bytes` - A [`GBytes`]. The argument can be [`null_mut`].
    pub fn g_bytes_unref(bytes: *mut GBytes);
}
