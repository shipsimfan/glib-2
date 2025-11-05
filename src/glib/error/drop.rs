use crate::{glib::GError, raw::glib::g_error_free};

impl Drop for GError {
    fn drop(&mut self) {
        if self.owned {
            unsafe { g_error_free(self.handle) };
        }
    }
}
