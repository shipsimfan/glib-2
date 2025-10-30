use crate::raw::glib::{GQuark, gchar, gint};
use std::ptr::null_mut;

/// The [`GError`] structure contains information about an error that has occurred.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GError {
    /// Error domain, e.g. [`G_FILE_ERROR`].
    pub domain: GQuark,

    /// Error code, e.g. [`G_FILE_ERROR_NOENT`].
    pub code: gint,

    /// Human-readable informative error message.
    pub message: *mut gchar,
}

impl Default for GError {
    fn default() -> Self {
        GError {
            domain: 0,
            code: 0,
            message: null_mut(),
        }
    }
}
