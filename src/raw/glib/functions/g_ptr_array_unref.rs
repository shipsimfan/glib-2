use crate::raw::glib::GPtrArray;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Atomically decrements the reference count of array by one. If the reference count drops to
    /// 0, the effect is the same as calling [`g_ptr_array_free`] with `free_segment` set to
    /// `true`. This function is thread-safe and may be called from any thread.
    ///
    /// # Parameters
    ///  * `array` - A pointer array.
    pub fn g_ptr_array_unref(array: *mut GPtrArray);
}
