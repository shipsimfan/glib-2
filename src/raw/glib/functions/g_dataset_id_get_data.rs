use crate::raw::glib::{GQuark, gconstpointer, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Gets the data element corresponding to a [`GQuark`].
    ///
    /// # Parameters
    ///  * `dataset_location` - The location identifying the dataset.
    ///  * `key_id` - The [`GQuark`] id to identify the data element.
    ///
    /// # Return Value
    /// The data element corresponding to the [`GQuark`], or [`null_mut`] if it is not found.
    pub fn g_dataset_id_get_data(dataset_location: gconstpointer, key_id: GQuark) -> gpointer;
}
