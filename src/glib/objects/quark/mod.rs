use crate::glib::guint32;

mod from_static_string;
mod from_string;
mod to_string;
mod try_string;

pub use from_static_string::g_quark_from_static_string;
pub use from_string::g_quark_from_string;
pub use to_string::g_quark_to_string;
pub use try_string::g_quark_try_string;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{g_intern_static_string, g_intern_string};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// A [`GQuark`] is a non-zero integer which uniquely identifies a particular string.
///
/// A [`GQuark`] value of zero is associated to [`null_mut`].
///
/// Given either the string or the [`GQuark`] identifier it is possible to retrieve the other.
///
/// Quarks are used for both datasets and keyed data lists.
///
/// To create a new quark from a string, use [`g_quark_from_string`] or
/// [`g_quark_from_static_string`].
///
/// To find the string corresponding to a given [`GQuark`], use [`g_quark_to_string`].
///
/// To find the [`GQuark`] corresponding to a given string, use [`g_quark_try_string`].
///
/// Another use for the string pool maintained for the quark functions is string interning, using
/// [`g_intern_string`] or [`g_intern_static_string`]. An interned string is a canonical
/// representation for a string. One important advantage of interned strings is that they can be
/// compared for equality by a simple pointer comparison, rather than using `strcmp`.
pub type GQuark = guint32;
