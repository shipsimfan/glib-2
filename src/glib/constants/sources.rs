use crate::glib::{FALSE, TRUE, gboolean};

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{GSource, GSourceFunc};

/// Use this macro as the return value of a [`GSourceFunc`] to leave the [`GSource`] in the main
/// loop.
pub const G_SOURCE_CONTINUE: gboolean = TRUE;

/// Use this macro as the return value of a [`GSourceFunc`] to remove the [`GSource`] from the main
/// loop.
pub const G_SOURCE_REMOVE: gboolean = FALSE;
