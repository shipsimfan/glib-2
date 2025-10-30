use crate::raw::glib::gpointer;

mod foreach;

pub use foreach::g_list_foreach;

/// The [`GList`] struct is used for each element in a doubly-linked list.
#[repr(C)]
pub struct GList {
    /// Holds the element’s data, which can be a pointer to any kind of data, or any integer value
    pub data: gpointer,

    /// Contains the link to the next element in the list.
    pub next: gpointer,

    /// Contains the link to the previous element in the list.
    pub prev: gpointer,
}
