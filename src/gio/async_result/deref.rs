use crate::{gio::GAsyncResult, gobject::GObject};
use std::ops::Deref;

impl Deref for GAsyncResult {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl AsRef<GObject> for GAsyncResult {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
