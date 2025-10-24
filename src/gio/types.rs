use std::ffi::c_void;

/// Thread-safe Operation Cancellation Stack
///
/// GCancellable is a thread-safe operation cancellation stack used throughout GIO to allow for
/// cancellation of synchronous and asynchronous operations.
pub type GCancellable = c_void;
