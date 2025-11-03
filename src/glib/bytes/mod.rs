use std::marker::PhantomData;

use crate::raw;

mod data;
mod drop;
mod get;
mod new;

/// A simple reference counted data type representing an immutable sequence of zero or more bytes
/// from an unspecified origin.
pub struct GBytes<'a> {
    /// The raw pointer to the underlying [`raw::glib::GBytes`]
    handle: *mut raw::glib::GBytes,

    /// Should this be freed when dropped?
    owned: Option<PhantomData<&'a ()>>,
}
