use crate::glib::GPtrArray;
use std::ops::Index;

impl<'a, T> Index<usize> for GPtrArray<'a, T> {
    type Output = *mut T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slice[index]
    }
}
