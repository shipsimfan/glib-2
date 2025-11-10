use crate::raw::{
    glib::{gpointer, guint},
    gobject::GValue,
};

/// A [`GClosure`] represents a callback supplied by the programmer.
///
/// It will generally comprise a function of some kind and a marshaller used to call it. It is the
/// responsibility of the marshaller to convert the arguments for the invocation from [`GValues`]
/// into a suitable form, perform the callback on the converted arguments, and transform the return
/// value back into a [`GValue`].
///
/// In the case of C programs, a closure usually just holds a pointer to a function and maybe a
/// data argument, and the marshaller converts between [`GValue`] and native C types. The GObject
/// library provides the GCClosure type for this purpose. Bindings for other languages need
/// marshallers which convert between [`GValue`]s and suitable representations in the runtime of
/// the language in order to use functions written in that language as callbacks. Use
/// [`g_closure_set_marshal`] to set the marshaller on such a custom closure implementation.
///
/// Within GObject, closures play an important role in the implementation of signals. When a signal
/// is registered, the `c_marshaller` argument to [`g_signal_new`] specifies the default C
/// marshaller for any closure which is connected to this signal. GObject provides a number of C
/// marshallers for this purpose, see the `g_cclosure_marshal_*` functions. Additional C
/// marshallers can be generated with the `glib-genmarshal` utility. Closures can be explicitly
/// connected to signals with [`g_signal_connect_closure`], but it usually more convenient to let
/// GObject create a closure automatically by using one of the `g_signal_connect_*` functions which
/// take a callback function/user data pair.
///
/// Using closures has a number of important advantages over a simple callback function/data
/// pointer combination:
///  - Closures allow the callee to get the types of the callback parameters, which means that
///    language bindings don’t have to write individual glue for each callback type.
///  - The reference counting of [`GClosure`] makes it easy to handle reentrancy right; if a
///    callback is removed while it is being invoked, the closure and its parameters won’t be freed
///    until the invocation finishes.
///  - [`g_closure_invalidate`] and invalidation notifiers allow callbacks to be automatically
///    removed when the objects they point to go away.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GClosure {
    /// Bit 0: Indicates whether the closure is currently being invoked with [`g_closure_invoke`].
    ///
    /// Bit 1: Indicates whether the closure has been invalidated by [`g_closure_invalidate`].
    pub flags: guint,

    /// No description available.
    pub marshal: Option<
        extern "C" fn(
            closure: *mut GClosure,
            return_value: *mut GValue,
            n_param_values: guint,
            param_values: *const GValue,
            invocation_hint: gpointer,
            marshal_data: gpointer,
        ),
    >,
}

impl Default for GClosure {
    fn default() -> Self {
        GClosure {
            flags: 0,
            marshal: None,
        }
    }
}
