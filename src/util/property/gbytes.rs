use crate::{glib::GBytes, util::Property};

impl<'a> Property for GBytes<'a> {
    fn as_ptr(&self) -> *mut std::ffi::c_void {
        unsafe { self.handle() }.cast()
    }

    unsafe fn from_ptr<'b>(_: *mut std::ffi::c_void) -> &'b Self {
        unimplemented!();
    }
}
