use crate::raw;

mod clone;
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

    /// Is this loop owned?
    owned: bool,
}

unsafe impl Send for GMainLoop {}
unsafe impl Sync for GMainLoop {}
