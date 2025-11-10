use crate::raw::glib::gconstpointer;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Destroys the dataset, freeing all memory allocated, and calling any destroy functions set
    /// for data elements.
    ///
    /// # Parameters
    ///  * `dataset_location` - The location identifying the dataset.
    pub fn g_dataset_destroy(dataset_location: gconstpointer);
}
