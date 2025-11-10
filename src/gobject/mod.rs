//! GNOME Object library

mod data_signal_callback;
mod object;

pub use data_signal_callback::GDataSignalCallback;
pub use object::GObject;

pub use crate::raw::gobject::{GConnectFlags, GType};
