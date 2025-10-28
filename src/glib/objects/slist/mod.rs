use crate::glib::gpointer;

mod foreach;

pub use foreach::g_slist_foreach;

/// The [`GSList`] struct is used for each element in the singly-linked list.
#[repr(C)]
pub struct GSList {
    /// Holds the element’s data, which can be a pointer to any kind of data, or any integer value
    pub data: gpointer,

    /// Contains the link to the next element in the list.
    pub next: gpointer,
}
