use crate::raw;

mod drop;
mod get;

/// GCancellable allows operations to be cancelled.
pub struct GCancellable {
    /// The handle to the underlying object
    handle: *mut raw::gio::GCancellable,
}
