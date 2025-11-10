use crate::{
    glib::{GDataset, GQuark},
    raw::glib::{g_dataset_id_set_data_full, gpointer},
    util::CallbackData,
};

extern "C" fn drop_value<T: CallbackData>(handle: gpointer) {
    drop(unsafe { T::from_ptr(handle) })
}

impl GDataset {
    /// Set a `key` to `value` in the dataset
    pub fn set<T: CallbackData>(&self, key: GQuark, value: T) {
        unsafe {
            g_dataset_id_set_data_full(self.handle, key.handle(), value.into_ptr(), drop_value::<T>)
        };
    }
}
