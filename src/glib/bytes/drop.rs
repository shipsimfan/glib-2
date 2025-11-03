use crate::{glib::GBytes, raw::glib::g_bytes_unref};

impl<'a> Drop for GBytes<'a> {
    fn drop(&mut self) {
        if self.owned.is_some() {
            unsafe { g_bytes_unref(self.handle) };
        }
    }
}
