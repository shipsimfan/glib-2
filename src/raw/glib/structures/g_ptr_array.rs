use crate::raw::glib::{gpointer, guint};
use std::ptr::null_mut;

/// Contains the public fields of a [`GPtrArray`]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GPtrArray {
    /// A pointer to the array of pointers, which may be moved when the array grows.
    pub data: *mut gpointer,

    /// The number of pointers in the array.
    pub len: guint,
}

impl Default for GPtrArray {
    fn default() -> Self {
        GPtrArray {
            data: null_mut(),
            len: 0,
        }
    }
}
