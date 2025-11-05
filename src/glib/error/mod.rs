use crate::raw;

mod display;
mod drop;
mod get;
mod new;
mod to_string_lossy;

/// The [`GError`] structure contains information about an error that has occurred.
pub struct GError {
    /// The handle to the underlying error
    handle: *mut raw::glib::GError,

    /// Is this error owned?
    owned: bool,
}
