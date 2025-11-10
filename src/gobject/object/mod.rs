use crate::raw;

mod clone;
mod drop;
mod get;
mod new;
mod set;
mod signal_connect_data;
mod signal_handler_disconnect;

/// The base object type.
pub struct GObject {
    /// The handle to underlying object
    handle: *mut raw::gobject::GObject,

    /// Is this object owned?
    owned: bool,
}
