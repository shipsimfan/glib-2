use crate::raw;

mod from_string;
mod get;
mod new;

/// A [`GQuark`] is a non-zero integer which uniquely identifies a particular string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GQuark {
    /// The handle to the underlying [`raw::glib::GQuark`]
    handle: raw::glib::GQuark,
}
