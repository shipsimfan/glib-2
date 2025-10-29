use std::ffi::c_void;

mod default;
mod get_thread_default;
mod new;
mod pop_thread_default;
mod push_thread_default;
mod r#ref;
mod ref_thread_default;
mod unref;

pub use default::g_main_context_default;
pub use get_thread_default::g_main_context_get_thread_default;
pub use new::g_main_context_new;
pub use pop_thread_default::g_main_context_pop_thread_default;
pub use push_thread_default::g_main_context_push_thread_default;
pub use r#ref::g_main_context_ref;
pub use ref_thread_default::g_main_context_ref_thread_default;
pub use unref::g_main_context_unref;

/// The [`GMainContext`] struct is an opaque data type representing a set of sources to be handled
/// in a main loop.
pub type GMainContext = c_void;
