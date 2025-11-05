use crate::raw;

mod default;
mod drop;
mod get;
mod new;

/// The [`GMainContext`] struct is an opaque data type representing a set of sources to be handled
/// in a main loop.
pub struct GMainContext {
    /// The handle to underlying main context
    handle: *mut raw::glib::GMainContext,

    /// Is this context owned?
    owned: bool,
}
