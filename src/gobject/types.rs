use std::ffi::c_void;

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
