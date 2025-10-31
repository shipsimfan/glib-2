use crate::{glib::GPtrArray, raw::glib::g_ptr_array_unref};

impl<'a, T> Drop for GPtrArray<'a, T> {
    fn drop(&mut self) {
        unsafe { g_ptr_array_unref(self.handle) };
    }
}
