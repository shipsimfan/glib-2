use crate::glib::gint64;

#[link(name = "glib-2.0")]
unsafe extern "C" {
    /// Queries the system monotonic time.
    ///
    /// The monotonic clock will always increase and doesn’t suffer discontinuities when the user
    /// (or NTP) changes the system time. It may or may not continue to tick during times where the
    /// machine is suspended.
    ///
    /// We try to use the clock that corresponds as closely as possible to the passage of time as
    /// measured by system calls such as `poll` but it may not always be possible to do this.
    ///
    /// # Return Value
    /// The monotonic time, in microseconds.
    pub fn g_get_monotonic_time() -> gint64;
}
