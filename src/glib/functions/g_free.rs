use crate::glib::gpointer;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::g_free_sized;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Frees the memory pointed to by `mem`.
    ///
    /// If you know the allocated size of `mem`, calling [`g_free_sized`] may be faster, depending
    /// on the libc implementation in use.
    ///
    /// Starting from GLib 2.78, this may happen automatically in case a GCC compatible compiler is
    /// used with some optimization level and the allocated size is known at compile time (see
    /// documentation of `__builtin_object_size` to understand its caveats).
    ///
    /// If `mem` is [`null_mut`] it simply returns, so there is no need to check `mem` against
    /// [`null_mut`] before calling this function.
    ///
    /// # Parameters
    ///  * `mem` - The memory to free. The argument can be [`null_mut`].
    pub fn g_free(mem: gpointer);
}
