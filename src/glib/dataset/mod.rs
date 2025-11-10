use crate::raw::glib::gconstpointer;

mod drop;
mod get;
mod new;
mod set;

/// A set of data associated with a paticular location
pub struct GDataset {
    /// The underlying handle to the dataset
    handle: gconstpointer,

    /// Is this copy of the dataset owned?
    owned: bool,
}
