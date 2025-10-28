use crate::glib::{GFunc, GSList, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Calls a function for each element of a [`GSList`].
    ///
    /// It is safe for `func` to remove the element from `list`, but it must not modify any part of
    /// the list after that element.
    ///
    /// # Parameters
    ///  * `list` - A [`GSList`].
    ///  * `func` - The function to call with each element’s data.
    ///  * `user_data` - User data to pass to the function. The argument can be [`null_mut`].
    pub fn g_slist_foreach(list: *mut GSList, func: GFunc, user_data: gpointer);
}
