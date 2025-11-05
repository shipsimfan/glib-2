use crate::raw;

mod drop;
mod get;
mod new;
mod set;

/// The base object type.
pub struct GObject {
    /// The handle to underlying object
    handle: *mut raw::gobject::GObject,

    /// Is this object owned?
    owned: bool,
}
