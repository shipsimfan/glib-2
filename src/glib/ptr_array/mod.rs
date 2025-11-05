use crate::raw;

mod deref;
mod drop;
mod get;
mod index;
mod into_iterator;
mod new;

/// An array of pointers
pub struct GPtrArray<'a, T> {
    /// The raw pointer to the underlying [`raw::glib::GPtrArray`]
    handle: *mut raw::glib::GPtrArray,

    /// The Rust view of the array
    slice: &'a [*mut T],

    /// Is this array owned?
    owned: bool,
}
