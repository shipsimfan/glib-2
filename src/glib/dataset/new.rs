use crate::{glib::GDataset, raw::glib::gconstpointer, util::AsHandle};
use std::ptr::null_mut;

impl GDataset {
    /// Creates a new owned [`GDataset`] associated with `location`
    pub fn new<T: AsHandle>(location: &T) -> Self {
        let handle = unsafe { location.as_handle() };
        unsafe { GDataset::new_raw(handle, true) }
    }

    /// Creates a new borrowed [`GDataset`] associated with `location`
    pub fn new_at<T: AsHandle>(location: &T) -> Self {
        let handle = unsafe { location.as_handle() };
        unsafe { GDataset::new_raw(handle, false) }
    }

    /// Creates a new [`GDataset`] from a raw `handle`
    pub unsafe fn new_raw(handle: gconstpointer, owned: bool) -> Self {
        assert_ne!(handle, null_mut());
        GDataset { handle, owned }
    }
}
