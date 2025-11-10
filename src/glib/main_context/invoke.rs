use crate::{
    glib::{GMainContext, GSourceFunc},
    raw::glib::g_main_context_invoke,
    util::CallbackData,
};

impl GMainContext {
    /// Invokes a function in such a way that `context` is owned during the invocation of
    /// `function`.
    pub fn invoke<Callback: GSourceFunc>(&self, data: Callback::UserData) {
        unsafe { g_main_context_invoke(self.handle, Callback::trampoline, data.into_ptr()) };
    }
}
