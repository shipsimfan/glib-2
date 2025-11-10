use crate::raw;

mod clone;
mod default;
mod drop;
mod get;
mod invoke;
mod new;

/// The [`GMainContext`] struct is an opaque data type representing a set of sources to be handled
/// in a main loop.
pub struct GMainContext {
    /// The handle to underlying main context
    handle: *mut raw::glib::GMainContext,

    /// Is this context owned?
    owned: bool,
}

unsafe impl Send for GMainContext {}
unsafe impl Sync for GMainContext {}
