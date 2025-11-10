use crate::raw::{
    glib::{gpointer, gsize},
    gobject::GClosure,
};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::G_CALLBACK;
#[allow(unused_imports)]
use std::ptr::null_mut;

/// The type used for callback functions in structure definitions and function signatures.
///
/// This doesn’t mean that all callback functions must take no parameters and return void. The
/// required signature of a callback function is determined by the context in which is used (e.g.
/// the signal to which it is connected).
///
/// Use [`G_CALLBACK`] to cast the callback function to a [`GCallback`].
pub type GCallback = *const c_void;

/// The type used for the various notification callbacks which can be registered on closures.
///
/// # Parameters
///  * `data` - Data specified when registering the notification callback. The argument can be
///             [`null_mut`].
///  * `closure` - The [`GClosure`] on which the notification is emitted.
pub type GClosureNotify = extern "C" fn(data: gpointer, closure: *mut GClosure);

/// The base object type.
///
/// [`GObject`] is the fundamental type providing the common attributes and methods for all object
/// types in GTK, Pango and other libraries based on [`GObject`]. The [`GObject`] class provides
/// methods for object construction and destruction, property access methods, and signal support.
///
/// Since GLib 2.72, all [`GObject`]s are guaranteed to be aligned to at least the alignment of the
/// largest basic GLib type (typically this is guint64 or gdouble). If you need larger alignment
/// for an element in a [`GObject`], you should allocate it on the heap (aligned), or arrange for
/// your [`GObject`] to be appropriately padded. This guarantee applies to the [`GObject`] (or
/// derived) struct, the `GObjectClass` (or derived) struct, and any private data allocated by
/// `G_ADD_PRIVATE`.
pub type GObject = c_void;

/// A numerical value which represents the unique identifier of a registered type.
pub type GType = gsize;

/// An opaque structure used to hold different types of values.
///
/// Before it can be used, a [`GValue`] has to be initialized to a specific type by calling
/// [`g_value_init`] on it.
///
/// Many types which are stored within a [`GValue`] need to allocate data on the heap, so
/// [`g_value_unset`] must always be called on a [`GValue`] to free any such data once you’re
/// finished with the [`GValue`], even if the [`GValue`] itself is stored on the stack.
///
/// The data within the structure has protected scope: it is accessible only to functions within a
/// [`GTypeValueTable`] structure, or implementations of the `g_value_*` API. That is, code which
/// implements new fundamental types.
///
/// [`GValue`] users cannot make any assumptions about how data is stored within the 2 element data
/// union, and the `g_type` member should only be accessed through the [`G_VALUE_TYPE`] macro and
/// related macros.
pub type GValue = c_void;
