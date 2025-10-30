use std::ffi::c_void;

mod attach;
mod destroy;
mod set_callback;
mod unref;

pub use attach::g_source_attach;
pub use destroy::g_source_destroy;
pub use set_callback::g_source_set_callback;
pub use unref::g_source_unref;

/// The [`GSource`] struct is an opaque data type representing an event source.
pub type GSource = c_void;
