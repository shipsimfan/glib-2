use crate::raw::{glib::gulong, gobject::GObject};

#[link(name = "gobject-2.0")]
unsafe extern "C" {
    /// Disconnects a handler from an instance so it will not be called during any future or
    /// currently ongoing emissions of the signal it has been connected to. The `handler_id`
    /// becomes invalid and may be reused.
    ///
    /// The `handler_id` has to be a valid signal handler id, connected to a signal of `instance`.
    ///
    /// # Parameters
    ///  * `instance` - The instance to remove the signal handler from.
    ///  * `handler_id` - Handler id of the handler to be disconnected.
    pub fn g_signal_handler_disconnect(instance: *mut GObject, handler_id: gulong);
}
