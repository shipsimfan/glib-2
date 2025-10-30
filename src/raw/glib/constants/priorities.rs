use crate::raw::glib::gint;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::glib::{g_idle_add, g_timeout_add};

/// Use this for high priority event sources.
///
/// It is not used within GLib or GTK.
pub const G_PRIORITY_HIGH: gint = -100;

/// Use this for default priority event sources.
///
/// In GLib this priority is used when adding timeout functions with [`g_timeout_add`]. In GDK this
/// priority is used for events from the X server.
pub const G_PRIORITY_DEFAULT: gint = 0;

/// Use this for high priority idle functions.
///
/// GTK uses `G_PRIORITY_HIGH_IDLE + 10` for resizing operations, and `G_PRIORITY_HIGH_IDLE + 20`
/// for redrawing operations. (This is done to ensure that any pending resizes are processed before
/// any pending redraws, so that widgets are not redrawn twice unnecessarily.).
pub const G_PRIORITY_HIGH_IDLE: gint = 100;

/// Use this for default priority idle functions.
///
/// In GLib this priority is used when adding idle functions with [`g_idle_add`].
pub const G_PRIORITY_DEFAULT_IDLE: gint = 200;
