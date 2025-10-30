use std::ffi::c_void;

mod get_data;

pub use get_data::g_bytes_get_data;

/// A simple reference counted data type representing an immutable sequence of zero or more bytes
/// from an unspecified origin.
///
/// The purpose of a [`GBytes`] is to keep the memory region that it holds alive for as long as
/// anyone holds a reference to the bytes. When the last reference count is dropped, the memory is
/// released. Multiple unrelated callers can use byte data in the [`GBytes`] without coordinating
/// their activities, resting assured that the byte data will not change or move while they hold a
/// reference.
///
/// A [`GBytes`] can come from many different origins that may have different procedures for
/// freeing the memory region. Examples are memory from [`g_malloc`], from memory slices, from a
/// [`GMappedFile`] or memory from other allocators.
///
/// [`GBytes`] work well as keys in [`GHashTable`]. Use [`g_bytes_equal`] and [`g_bytes_hash`] as
/// parameters to [`g_hash_table_new`] or [`g_hash_table_new_full`]. [`GBytes`] can also be used as
/// keys in a [`GTree`] by passing the [`g_bytes_compare`] function to [`g_tree_new`].
///
/// The data pointed to by this bytes must not be modified. For a mutable array of bytes see
/// [`GByteArray`]. Use [`g_bytes_unref_to_array`] to create a mutable array for a GBytes sequence.
/// To create an immutable [`GBytes`] from a mutable [`GByteArray`], use the
/// [`g_byte_array_free_to_bytes`] function.
pub type GBytes = c_void;
