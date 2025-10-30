use crate::glib::GError;
use std::borrow::Cow;

impl GError {
    /// Get the message of this error as a string
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.message().to_string_lossy()
    }
}
