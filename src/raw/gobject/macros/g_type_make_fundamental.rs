// rustdoc imports
#[allow(unused_imports)]
use crate::raw::gobject::GType;

/// Get the type ID for the fundamental type number `x`.
///
/// Use [`g_type_fundamental_next`] instead of this macro to create new fundamental types.
///
/// # Parameters
///  * `x` - the fundamental type number.
///
/// # Returns
/// the [`GType`]
#[macro_export]
macro_rules! G_TYPE_MAKE_FUNDAMENTAL {
    ($x: expr) => {
        ($x as $crate::raw::gobject::GType) << $crate::raw::gobject::G_TYPE_FUNDAMENTAL_SHIFT
    };
}
