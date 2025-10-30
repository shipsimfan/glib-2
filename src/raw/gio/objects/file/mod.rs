use std::ffi::c_void;

mod supports_thread_contexts;

pub use supports_thread_contexts::g_file_supports_thread_contexts;

/// [`GFile`] is a high level abstraction for manipulating files on a virtual file system.
/// [`GFile`]s are lightweight, immutable objects that do no I/O upon creation. It is necessary to
/// understand that [`GFile`] objects do not represent files, merely an identifier for a file. All
/// file content I/O is implemented as streaming operations (see [`GInputStream`] and
/// [`GOutputStream`]).
pub type GFile = c_void;
