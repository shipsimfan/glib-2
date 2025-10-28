use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    G_SOURCE_FUNC,
    glib::{
        FALSE, G_SOURCE_CONTINUE, G_SOURCE_REMOVE, TRUE, g_idle_add, g_idle_add_full,
        g_list_foreach, g_slist_foreach, g_source_set_callback, g_timeout_add, g_timeout_add_full,
    },
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// A standard boolean type. Variables of this type should only contain the value [`TRUE`] or
/// [`FALSE`].
///
/// There is no validation when assigning to a [`gboolean`] variable and so it could contain any
/// value represented by a [`gint`]. All non-zero values in C evaluate to ‘true’.
#[allow(non_camel_case_types)]
pub type gboolean = gint;

/// Equivalent to the standard C `char` type.
///
/// This type only exists for symmetry with [`guchar`]. The standard C `char` type should be
/// preferred in new code.
#[allow(non_camel_case_types)]
pub type gchar = c_char;

/// An untyped pointer to constant data, exactly equivalent to `const void*`.
///
/// The data pointed to should not be changed.
///
/// This is typically used in function prototypes to indicate that the data pointed to will not be
/// altered by the function.
///
/// The standard C `const void*` type should usually be preferred in new code, but
/// [`gconstpointer`] can be used in contexts where a type name must be a single word.
#[allow(non_camel_case_types)]
pub type gconstpointer = *const c_void;

/// Specifies the type of function which is called when a data element is destroyed. It is passed
/// the pointer to the data element and should free any memory and resources allocated for it.
///
/// # Parameters
///  * `data` - The data element. The argument can be [`null_mut`].
pub type GDestroyNotify = extern "C" fn(data: gpointer);

/// Equivalent to the standard C `double` type.
#[allow(non_camel_case_types)]
pub type gdouble = c_double;

/// Equivalent to the standard C `float` type.
#[allow(non_camel_case_types)]
pub type gfloat = c_float;

/// Specifies the type of functions passed to [`g_list_foreach`] and [`g_slist_foreach`].
///
/// # Parameters
///  * `data` - The element’s data. The argument can be [`null_mut`].
///  * `user_data` - User data passed to [`g_list_foreach`] or [`g_slist_foreach`]. The argument
///                  can be [`null_mut`].
pub type GFunc = extern "C" fn(data: gpointer, user_data: gpointer);

/// Equivalent to the standard C `int` type.
///
/// Values of this type can range from [`c_int::MIN`] to [`c_int::MAX`].
///
/// This type only exists for symmetry with guint. The standard C `int` type should be preferred in
/// new code.
#[allow(non_camel_case_types)]
pub type gint = c_int;

/// A signed integer guaranteed to be 8 bits on all platforms, similar to the standard C `int8_t`.
///
/// The `int8_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`gint8`] (see [`gsize`] for more details).
///
/// Values of this type can range from -128 to 127.
#[allow(non_camel_case_types)]
pub type gint8 = i8;

/// A signed integer guaranteed to be 16 bits on all platforms, similar to the standard C
/// `int16_t`.
///
/// The `int16_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`gint16`] (see [`gsize`] for more details).
///
/// Values of this type can range from -32,768 to 32,767.
#[allow(non_camel_case_types)]
pub type gint16 = i16;

/// A signed integer guaranteed to be 32 bits on all platforms.
///
/// The `int32_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`gint16`] (see [`gsize`] for more details).
///
/// Values of this type can range from -2,147,483,648 to 2,147,483,647.
///
/// Note that on platforms with more than one 32-bit standard integer type, [`gint32`] and
/// `int32_t` are not necessarily implemented by the same 32-bit integer type. For example, on an
/// ILP32 platform where `int` and `long` are both 32-bit, it might be the case that one of these
/// types is `int` and the other is `long`. See [`gsize`] for more details of what this implies.
#[allow(non_camel_case_types)]
pub type gint32 = i32;

/// A signed integer guaranteed to be 64 bits on all platforms, similar to the standard C
/// `int64_t`.
///
/// The `int64_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`gint64`] (see [`gsize`] for more details).
///
/// Values of this type can range from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.
///
/// Note that on platforms with more than one 64-bit standard integer type, [`gint64`] and
/// `int64_t` are not necessarily implemented by the same 64-bit integer type. For example, on a
/// platform where both `long` and `long long` are 64-bit, it might be the case that one of those
/// types is used for [`gint64`] and the other is used for `int64_t`. See [`gsize`] for more
/// details of what this implies.
#[allow(non_camel_case_types)]
pub type gint64 = i64;

/// Corresponds to the C99 type `intptr_t`, a signed integer type that can hold any pointer.
///
/// The standard `intptr_t` type should be preferred in new code, unless consistency with
/// pre-existing APIs requires [`gintptr`]. Note that `intptr_t` and [`gintptr`] might be
/// implemented by different standard integer types of the same size. See [`gsize`] for more
/// details.
///
/// [`gintptr`] is not guaranteed to be the same type or the same size as [`gssize`], even though
/// they are the same on many CPU architectures.
#[allow(non_camel_case_types)]
pub type gintptr = isize;

/// Equivalent to the standard C `long` type.
///
/// This type only exists for symmetry with [`gulong`]. The standard C `long` type should be
/// preferred in new code.
#[allow(non_camel_case_types)]
pub type glong = c_long;

/// A signed integer type that is used for file offsets, corresponding to the POSIX type `off_t` as
/// if compiling with `_FILE_OFFSET_BITS` set to 64. [`goffset`] is always 64 bits wide, even on
/// 32-bit architectures, and even if `off_t` is only 32 bits.
///
/// On platforms with more than one 64-bit standard integer type, even if `off_t` is also 64 bits
/// in size, [`goffset`] and `off_t` are not necessarily implemented by the same 64-bit integer
/// type. See [`gsize`] for more details of what this implies.
#[allow(non_camel_case_types)]
pub type goffset = gint64;

/// An untyped pointer, exactly equivalent to `void*`.
///
/// The standard C `void*` type should usually be preferred in new code, but [`gpointer`] can be
/// used in contexts where a type name must be a single word, such as in the [`GType`] name of
/// [`G_TYPE_POINTER`] or when generating a family of function names for multiple types using
/// macros.
#[allow(non_camel_case_types)]
pub type gpointer = *mut c_void;

/// Equivalent to the standard C `short` type.
///
/// This type only exists for symmetry with [`gushort`]. The standard C `short` type should be
/// preferred in new code.
#[allow(non_camel_case_types)]
pub type gshort = c_short;

/// An unsigned integer type of the result of the `sizeof` operator, corresponding to the `size_t`
/// type defined in C99.
///
/// The standard `size_t` type should be preferred in new code, unless consistency with
/// pre-existing APIs requires [`gsize`] (see below for more details).
///
/// [`gsize`] is usually 32 bit wide on a 32-bit platform and 64 bit wide on a 64-bit platform.
///
/// This type is wide enough to hold the size of the largest possible memory allocation, but is not
/// guaranteed to be wide enough to hold the numeric value of a pointer: on platforms that use
/// tagged pointers, such as CHERI, pointers can be numerically larger than the size of the address
/// space. If the numeric value of a pointer needs to be stored in an integer without information
/// loss, use the standard C types `intptr_t` or `uintptr_t`, or the similar GLib types [`gintptr`]
/// or [`guintptr`].
///
/// Note that on platforms where more than one standard integer type is the same size, `size_t` and
/// [`gsize`] are always the same size but are not necessarily implemented by the same standard
/// integer type. For example, on an ILP32 platform where int, long and pointers are all 32-bit,
/// `size_t` might be `unsigned long` while [`gsize`] might be `unsigned int`. This can result in
/// compiler warnings or unexpected C++ name-mangling if the two types are used inconsistently.
///
/// As a result, changing a type from [`gsize`] to `size_t` in existing APIs might be an
/// incompatible API or ABI change, especially if C++ is involved. The safe option is to leave
/// existing APIs using the same type that they have historically used, and only use the standard
/// C types in new APIs.
///
/// Similar considerations apply to all the fixed-size types ([`gint8`], [`guint8`], [`gint16`],
/// [`guint16`], [`gint32`], [`guint32`], [`gint64`], [`guint64`] and [`goffset`]), as well as
/// [`gintptr`] and [`guintptr`]. Types that are 32 bits or larger are particularly likely to be
/// affected by this.
#[allow(non_camel_case_types)]
pub type gsize = usize;

/// Specifies the type of function passed to [`g_timeout_add`], [`g_timeout_add_full`],
/// [`g_idle_add`], and [`g_idle_add_full`].
///
/// When calling [`g_source_set_callback`], you may need to cast a function of a different type to
/// this type. Use [`G_SOURCE_FUNC`] to avoid warnings about incompatible function types.
///
/// # Parameters
///  * `user_data` - Data passed to the function, set when the source was created with one of the
///                  above functions. The argument can be [`null_mut`].
///
/// # Return Value
/// [`FALSE`] if the source should be removed. [`G_SOURCE_CONTINUE`] and [`G_SOURCE_REMOVE`] are
/// more memorable names for the return value.
pub type GSourceFunc = extern "C" fn(user_data: gpointer) -> gboolean;

/// A signed variant of [`gsize`], corresponding to the `ssize_t` defined in POSIX or the similar
/// `SSIZE_T` in Windows.
///
/// In new platform-specific code, consider using `ssize_t` or `SSIZE_T` directly.
///
/// Note that on platforms where `ssize_t` is implemented, `ssize_t` and [`gssize`] might be
/// implemented by different standard integer types of the same size. Similarly, on Windows,
/// `SSIZE_T` and [`gssize`] might be implemented by different standard integer types of the same
/// size. See [`gsize`] for more details.
///
/// This type is also not guaranteed to be the same as standard C `ptrdiff_t`, although they are
/// the same on many platforms.
#[allow(non_camel_case_types)]
pub type gssize = isize;

/// Equivalent to the standard C `unsigned char` type.
///
/// The standard C `unsigned char` type should usually be preferred in new code, but [`guchar`] can
/// be used in contexts where a type name must be a single word, such as in the [`GType`] name of
/// [`G_TYPE_UCHAR`] or when generating a family of function names for multiple types using macros.
#[allow(non_camel_case_types)]
pub type guchar = c_uchar;

/// Equivalent to the standard C `unsigned int` type.
///
/// Values of this type can range from 0 to [`c_uint::MAX`].
///
/// The standard C `unsigned int` type should usually be preferred in new code, but [`guint`] can
/// be used in contexts where a type name must be a single word, such as in the [`GType`] name of
/// [`G_TYPE_UINT`] or when generating a family of function names for multiple types using macros.
#[allow(non_camel_case_types)]
pub type guint = c_uint;

/// An unsigned integer guaranteed to be 8 bits on all platforms, similar to the standard C
/// `uint8_t`.
///
/// The `uint8_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`guint8`] (see [`gsize`] for more details).
///
/// Values of this type can range from 0 to 255.
#[allow(non_camel_case_types)]
pub type guint8 = u8;

/// An unsigned integer guaranteed to be 16 bits on all platforms, similar to the standard C
/// `uint16_t`.
///
/// The `uint16_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`guint16`] (see [`gsize`] for more details).
///
/// Values of this type can range from 0 to 65,535.
#[allow(non_camel_case_types)]
pub type guint16 = u16;

/// An unsigned integer guaranteed to be 32 bits on all platforms, similar to the standard C
/// `uint32_t`.
///
/// The `uint32_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`guint32`] (see [`gsize`] for more details).
///
/// Values of this type can range from 0 to 4,294,967,295.
///
/// Note that on platforms with more than one 32-bit standard integer type, [`guint32`] and
/// `uint32_t` are not necessarily implemented by the same 32-bit integer type. See [`gsize`] for
/// more details of what this implies.
#[allow(non_camel_case_types)]
pub type guint32 = u32;

/// An unsigned integer guaranteed to be 64-bits on all platforms, similar to the standard C
/// `uint64_t` type.
///
/// The `uint64_t` type should be preferred in new code, unless consistency with pre-existing APIs
/// requires use of [`guint64`] (see [`gsize`] for more details).
///
/// Values of this type can range from 0 to 18,446,744,073,709,551,615.
///
/// Note that on platforms with more than one 64-bit standard integer type, [`guint64`] and
/// `uint64_t` are not necessarily implemented by the same 64-bit integer type. See [`gsize`] for
/// more details of what this implies.
#[allow(non_camel_case_types)]
pub type guint64 = u64;

/// Corresponds to the C99 type `uintptr_t`, an unsigned integer type that can hold any pointer.
///
/// The standard `uintptr_t` type should be preferred in new code, unless consistency with
/// pre-existing APIs requires [`guintptr`]. Note that `uintptr_t` and [`guintptr`] might be
/// implemented by different standard integer types of the same size. See [`gsize`] for more
/// details.
///
/// [`guintptr`] is not guaranteed to be the same type or the same size as [`gsize`], even though
/// they are the same on many CPU architectures.
#[allow(non_camel_case_types)]
pub type guintptr = usize;

/// Equivalent to the standard C `unsigned long` type.
///
/// The standard C `unsigned long` type should usually be preferred in new code, but [`gulong`] can
/// be used in contexts where a type name must be a single word, such as in the [`GType`] name of
/// [`G_TYPE_ULONG`] or when generating a family of function names for multiple types using macros.
#[allow(non_camel_case_types)]
pub type gulong = c_ulong;

/// Equivalent to the standard C `unsigned short` type.
///
/// The standard C `unsigned short` type should usually be preferred in new code, but [`gushort`]
/// can be used in contexts where a type name must be a single word, such as when generating a
/// family of function names for multiple types using macros.
#[allow(non_camel_case_types)]
pub type gushort = c_ushort;
