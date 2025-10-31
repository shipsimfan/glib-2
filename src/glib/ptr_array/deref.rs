use crate::glib::GPtrArray;
use std::ops::Deref;

impl<'a, T> Deref for GPtrArray<'a, T> {
    type Target = [*mut T];

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}
