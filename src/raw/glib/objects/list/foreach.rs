use crate::raw::glib::{GFunc, GList, gpointer};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Calls a function for each element of a [`GList`].
    ///
    /// It is safe for `func` to remove the element from `list`, but it must not modify any part of
    /// the list after that element.
    ///
    /// # Parameters
    ///  * `list` - A [`GList`], this must point to the top of the list.
    ///  * `func` - The function to call with each element’s data.
    ///  * `user_data` - User data to pass to the function. The argument can be [`null_mut`].
    pub fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer);
}
