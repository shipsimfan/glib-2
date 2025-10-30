use std::ffi::c_void;

mod get_context;
mod is_running;
mod new;
mod quit;
mod r#ref;
mod run;
mod unref;

pub use get_context::g_main_loop_get_context;
pub use is_running::g_main_loop_is_running;
pub use new::g_main_loop_new;
pub use quit::g_main_loop_quit;
pub use r#ref::g_main_loop_ref;
pub use run::g_main_loop_run;
pub use unref::g_main_loop_unref;

/// The [`GMainLoop`] struct is an opaque data type representing the main event loop of a GLib or
/// GTK application.
pub type GMainLoop = c_void;
