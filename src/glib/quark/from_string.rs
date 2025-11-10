use crate::{
    glib::GQuark,
    raw::glib::{g_quark_from_static_string, g_quark_from_string},
};
use std::ffi::{CStr, CString};

impl GQuark {
    /// Gets the [`GQuark`] identifying the given string.
    pub fn from_string(string: &str) -> Self {
        let string = CString::new(string).unwrap();
        GQuark::from_string_raw(&string)
    }

    /// Gets the [`GQuark`] identifying the given string.
    pub fn from_string_raw(string: &CStr) -> Self {
        let handle = unsafe { g_quark_from_string(string.as_ptr()) };
        unsafe { GQuark::new_raw(handle) }
    }

    /// Gets the [`GQuark`] identifying the given (static) string.
    pub fn from_static_string(string: &'static CStr) -> Self {
        let handle = unsafe { g_quark_from_static_string(string.as_ptr()) };
        unsafe { GQuark::new_raw(handle) }
    }
}
