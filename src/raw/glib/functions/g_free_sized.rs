use crate::raw::glib::gpointer;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::g_free;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Frees the memory pointed to by `mem`, assuming it is has the given `size`.
    ///
    /// If `mem` is [`null_mut`] this is a no-op (and `size` is ignored).
    ///
    /// It is an error if `size` doesn’t match the size passed when `mem` was allocated. `size` is
    /// passed to this function to allow optimizations in the allocator. If you don’t know the
    /// allocation size, use [`g_free`] instead.
    ///
    /// # Parameters
    ///  * `mem` - The memory to free. The argument can be [`null_mut`].
    ///  * `size` - Size of `mem`, in bytes.
    pub fn g_free_sized(mem: gpointer, size: usize);
}
