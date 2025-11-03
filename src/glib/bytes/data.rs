use crate::{glib::GBytes, raw::glib::g_bytes_get_data};

impl<'a> GBytes<'a> {
    /// Get the byte data in the [`GBytes`]
    pub fn data(&self) -> &'a [u8] {
        let mut size = 0;
        let ptr = unsafe { g_bytes_get_data(self.handle, &mut size) };
        if size == 0 {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(ptr.cast(), size) }
    }
}
