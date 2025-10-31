use crate::glib::GPtrArray;

impl<'a, 'b, T> IntoIterator for &'b GPtrArray<'a, T> {
    type Item = *mut T;
    type IntoIter = std::iter::Map<std::slice::Iter<'b, *mut T>, fn(&*mut T) -> *mut T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter().map(|ptr| *ptr)
    }
}
