// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{GSourceFunc, g_source_set_callback};

/// Cast a function pointer to a [`GSourceFunc`], suppressing warnings from GCC 8 onwards with
/// `-Wextra` or `-Wcast-function-type` enabled about the function types being incompatible.
///
/// For example, the correct type of callback for a source created by [`g_child_watch_source_new`]
/// is [`GChildWatchFunc`], which accepts more arguments than [`GSourceFunc`]. Casting the function
/// with `(GSourceFunc)` to call [`g_source_set_callback`] will trigger a warning, even though it
/// will be cast back to the correct type before it is called by the source.
///
/// # Parameters
///  * `f` - A function pointer.
#[macro_export]
macro_rules! G_SOURCE_FUNC {
    ($f: expr) => {
        $f as $crate::glib::GSourceFunc
    };
}
