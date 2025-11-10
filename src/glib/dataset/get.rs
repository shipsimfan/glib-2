use crate::{
    glib::{GDataset, GQuark},
    raw::glib::g_dataset_id_get_data,
    util::CallbackData,
};

impl GDataset {
    /// Get the value at `key` in this dataset
    pub unsafe fn get<'a, T: CallbackData>(&'a self, key: GQuark) -> T::Ref<'a> {
        let ptr = unsafe { g_dataset_id_get_data(self.handle, key.handle()) };
        unsafe { T::from_ptr_ref(ptr) }
    }
}
