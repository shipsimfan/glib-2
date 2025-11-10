use crate::raw::glib::{GDestroyNotify, GQuark, gconstpointer, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Sets the data element associated with the given [`GQuark`] id, and also the function to
    /// call when the data element is destroyed. Any previous data with the same key is removed,
    /// and its destroy function is called.
    ///
    /// # Parameters
    ///  * `dataset_location` - The location identifying the dataset.
    ///  * `key_id` - The [`GQuark`] id to identify the data element.
    ///  * `data` - The data element. The argument can be [`null_mut`].
    ///  * `destroy_func` - The function to call when the data element is removed. This function
    ///                     will be called with the data element and can be used to free any memory
    ///                     allocated for it.
    pub fn g_dataset_id_set_data_full(
        dataset_location: gconstpointer,
        key_id: GQuark,
        data: gpointer,
        destroy_func: GDestroyNotify,
    );
}
