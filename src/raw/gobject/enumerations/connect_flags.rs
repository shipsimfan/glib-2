/// The connection flags are used to specify the behaviour of a signal’s connection.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum GConnectFlags {
    /// Default behaviour (no special flags).
    Default = 0,

    /// If set, the handler should be called after the default handler of the signal. Normally, the
    /// handler is called before the default handler.
    After = 1,

    /// If set, the instance and data should be swapped when calling the handler; see
    /// [`g_signal_connect_swapped`] for an example.
    Swapped = 2,
}
