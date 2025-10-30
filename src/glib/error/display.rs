use crate::glib::GError;

impl std::fmt::Debug for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(GError))
            .field(stringify!(domain), &self.domain())
            .field(stringify!(code), &self.code())
            .field(stringify!(message), &self.message())
            .finish()
    }
}

impl std::fmt::Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}
