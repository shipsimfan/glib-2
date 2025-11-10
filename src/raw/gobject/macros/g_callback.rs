// rustdoc imports
#[allow(unused_imports)]
use crate::raw::gobject::GCallback;

/// Casts `fn` to a [`GCallback`]
#[macro_export]
macro_rules! G_CALLBACK {
    ($fn: expr) => {
        $fn as *const std::ffi::c_void
    };
}
