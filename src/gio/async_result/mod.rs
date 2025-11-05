use crate::gobject::GObject;

mod deref;
mod new;

/// [`GAsyncResult`] provides a base class for implementing asynchronous function results
pub struct GAsyncResult {
    /// The handle to the underlying object
    object: GObject,
}
