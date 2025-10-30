use crate::{G_TYPE_MAKE_FUNDAMENTAL, raw::gobject::GType};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::gchar;

/// An invalid `#GType` used as error return value in some functions which return a `#GType`.
pub const G_TYPE_INVALID: GType = G_TYPE_MAKE_FUNDAMENTAL!(0);

/// A fundamental type which is used as a replacement for the C `void` return type.
pub const G_TYPE_NONE: GType = G_TYPE_MAKE_FUNDAMENTAL!(1);

/// The fundamental type from which all interfaces are derived.
pub const G_TYPE_INTERFACE: GType = G_TYPE_MAKE_FUNDAMENTAL!(2);

/// The fundamental type corresponding to `#gchar`.
///
/// The type designated by `%G_TYPE_CHAR` is unconditionally an 8-bit signed integer. This may or
/// may not be the same type a the C type [`gchar`].
pub const G_TYPE_CHAR: GType = G_TYPE_MAKE_FUNDAMENTAL!(3);

/// The fundamental type corresponding to `#guchar`.
pub const G_TYPE_UCHAR: GType = G_TYPE_MAKE_FUNDAMENTAL!(4);

/// The fundamental type corresponding to `#gboolean`.
pub const G_TYPE_BOOLEAN: GType = G_TYPE_MAKE_FUNDAMENTAL!(5);

/// The fundamental type corresponding to `#gint`.
pub const G_TYPE_INT: GType = G_TYPE_MAKE_FUNDAMENTAL!(6);

/// The fundamental type corresponding to `#guint`.
pub const G_TYPE_UINT: GType = G_TYPE_MAKE_FUNDAMENTAL!(7);

/// The fundamental type corresponding to `#glong`.
pub const G_TYPE_LONG: GType = G_TYPE_MAKE_FUNDAMENTAL!(8);

/// The fundamental type corresponding to `#gulong`.
pub const G_TYPE_ULONG: GType = G_TYPE_MAKE_FUNDAMENTAL!(9);

/// The fundamental type corresponding to `#gint64`.
pub const G_TYPE_INT64: GType = G_TYPE_MAKE_FUNDAMENTAL!(10);

/// The fundamental type corresponding to `#guint64`.
pub const G_TYPE_UINT64: GType = G_TYPE_MAKE_FUNDAMENTAL!(11);

/// The fundamental type from which all enumeration types are derived.
pub const G_TYPE_ENUM: GType = G_TYPE_MAKE_FUNDAMENTAL!(12);

/// The fundamental type from which all flags types are derived.
pub const G_TYPE_FLAGS: GType = G_TYPE_MAKE_FUNDAMENTAL!(13);

/// The fundamental type corresponding to `#gfloat`.
pub const G_TYPE_FLOAT: GType = G_TYPE_MAKE_FUNDAMENTAL!(14);

/// The fundamental type corresponding to `#gdouble`.
pub const G_TYPE_DOUBLE: GType = G_TYPE_MAKE_FUNDAMENTAL!(15);

/// The fundamental type corresponding to nul-terminated C strings.
pub const G_TYPE_STRING: GType = G_TYPE_MAKE_FUNDAMENTAL!(16);

/// The fundamental type corresponding to `#gpointer`.
pub const G_TYPE_POINTER: GType = G_TYPE_MAKE_FUNDAMENTAL!(17);

/// The fundamental type from which all boxed types are derived.
pub const G_TYPE_BOXED: GType = G_TYPE_MAKE_FUNDAMENTAL!(18);

/// The fundamental type from which all `#GParamSpec` types are derived.
pub const G_TYPE_PARAM: GType = G_TYPE_MAKE_FUNDAMENTAL!(19);

/// The fundamental type for `#GObject`.
pub const G_TYPE_OBJECT: GType = G_TYPE_MAKE_FUNDAMENTAL!(20);

/// The fundamental type corresponding to `#GVariant`.
///
/// All floating `#GVariant` instances passed through the `#GType` system are consumed.
///
/// Note that callbacks in closures, and signal handlers for signals of return type
/// `%G_TYPE_VARIANT`, must never return floating variants.
///
/// Note: GLib 2.24 did include a boxed type with this name. It was replaced with this fundamental
/// type in 2.26.
pub const G_TYPE_VARIANT: GType = G_TYPE_MAKE_FUNDAMENTAL!(21);
