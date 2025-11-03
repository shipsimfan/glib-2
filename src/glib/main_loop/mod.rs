use crate::raw;

mod drop;
mod get;
mod new;
mod quit;
mod run;

/// The [`GMainLoop`] struct is an opaque data type representing the main event loop of a GLib or
/// GTK application.
pub struct GMainLoop {
    /// The handle to underlying main loop
    handle: *mut raw::glib::GMainLoop,
}
