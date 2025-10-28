use crate::glib::GSource;

// rustdoc imports
#[allow(unused_imports)]
use crate::glib::{G_PRIORITY_DEFAULT, G_PRIORITY_DEFAULT_IDLE, GMainContext, g_source_attach};

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Creates a new idle source.
    ///
    /// The source will not initially be associated with any [`GMainContext`] and must be added to
    /// one with [`g_source_attach`] before it will be executed. Note that the default priority for
    /// idle sources is [`G_PRIORITY_DEFAULT_IDLE`], as compared to other sources which have a
    /// default priority of [`G_PRIORITY_DEFAULT`].
    ///
    /// # Return Value
    /// The newly-created idle source.
    pub fn g_idle_source_new() -> *mut GSource;
}
