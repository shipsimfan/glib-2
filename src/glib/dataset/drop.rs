use crate::{glib::GDataset, raw::glib::g_dataset_destroy};

impl Drop for GDataset {
    fn drop(&mut self) {
        if self.owned {
            unsafe { g_dataset_destroy(self.handle) };
        }
    }
}
