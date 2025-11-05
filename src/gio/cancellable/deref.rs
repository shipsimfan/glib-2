use crate::{gio::GCancellable, gobject::GObject};
use std::ops::Deref;

impl Deref for GCancellable {
    type Target = GObject;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl AsRef<GObject> for GCancellable {
    fn as_ref(&self) -> &GObject {
        &self.object
    }
}
