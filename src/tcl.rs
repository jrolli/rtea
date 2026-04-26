//! This module wraps all of the TCL functions that need to static references
//! for object lifecycle management task to work.  Specifically, these are
//! functions which may need to be called in contexts where an interpreter
//! pointer is either not known (objects & object types) or immediately
//! available (methods on objects).

use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;

use crate::Interpreter;
use crate::ObjectType;
use crate::RawObject;

pub(crate) static mut ALLOC: Option<extern "C" fn(usize) -> *mut c_void> = None;
pub(crate) static mut REALLOC: Option<extern "C" fn(*mut c_void, usize) -> *mut c_void> = None;
pub(crate) static mut FREE: Option<extern "C" fn(*mut c_void)> = None;

pub(crate) static mut NEW_OBJ: Option<extern "C" fn() -> *mut RawObject> = None;
pub(crate) static mut DUPLICATE_OBJ: Option<extern "C" fn(*mut RawObject) -> *mut RawObject> = None;
pub(crate) static mut INCR_REF_COUNT: Option<extern "C" fn(*mut RawObject)> = None;
pub(crate) static mut DECR_REF_COUNT: Option<extern "C" fn(*mut RawObject)> = None;
pub(crate) static mut IS_SHARED: Option<extern "C" fn(*mut RawObject) -> i32> = None;
pub(crate) static mut INVALIDATE_STRING_REP: Option<extern "C" fn(*mut RawObject)> = None;
pub(crate) static mut GET_STRING: Option<extern "C" fn(*mut RawObject) -> *mut c_char> = None;

pub(crate) static mut GET_OBJ_TYPE: Option<extern "C" fn(*const c_char) -> *const ObjectType> =
    None;
pub(crate) static mut CONVERT_TO_TYPE: Option<
    extern "C" fn(*const Interpreter, *mut RawObject, *const ObjectType) -> i32,
> = None;

pub(crate) static mut NEW_STRING_OBJ: Option<
    extern "C" fn(*const c_char, usize) -> *mut RawObject,
> = None;

pub(crate) static mut SET_STRING_OBJ: Option<extern "C" fn(*mut RawObject, *const c_char, usize)> =
    None;

/// An owned buffer of Tcl-managed memory.
///
/// `TclBuf` is a RAII wrapper around a heap allocation made by Tcl's
/// allocator.  When dropped it returns the memory to Tcl via `Tcl_Free`.
///
/// When the buffer must be handed to Tcl directly (e.g. assigning
/// `RawObject::bytes`), call [`into_raw_parts`](TclBuf::into_raw_parts) to
/// relinquish ownership without freeing the allocation.
pub struct TclBuf {
    ptr: *mut u8,
    /// The usable length of the buffer, **excluding** any null terminator.
    len: usize,
}

impl TclBuf {
    /// Constructs a `TclBuf` from a raw pointer and length.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a live allocation made by Tcl's allocator of at
    /// least `len` bytes, and the caller must transfer ownership to this
    /// `TclBuf`.
    pub(crate) unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Returns the number of usable bytes in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer contains no usable bytes.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Relinquishes ownership of the allocation, returning the raw pointer
    /// and length so they can be passed to Tcl directly.
    ///
    /// The returned pointer is cast to `*mut c_char` as required by Tcl's
    /// `Tcl_Obj::bytes` field.  The caller is responsible for ensuring the
    /// memory is eventually freed (typically Tcl does this automatically when
    /// it invalidates a string representation).
    pub fn into_raw_parts(self) -> (*mut c_char, usize) {
        let md = ManuallyDrop::new(self);
        (md.ptr as *mut c_char, md.len)
    }
}

impl Drop for TclBuf {
    fn drop(&mut self) {
        unsafe { FREE.expect("module must have been initialized")(self.ptr as *mut c_void) }
    }
}

impl Deref for TclBuf {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl DerefMut for TclBuf {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

// Safety: the underlying allocation is not aliased and Tcl's allocator is
// thread-safe, so TclBuf can safely be moved across threads.
unsafe impl Send for TclBuf {}

/// Allocates a null-terminated copy of `rust_str` in Tcl-managed memory and
/// returns it as a [`TclBuf`].
///
/// The returned buffer's [`len`](TclBuf::len) reflects the string's byte
/// length, **not** including the null terminator.  Use
/// [`into_raw_parts`](TclBuf::into_raw_parts) when the raw pointer and length
/// are needed for Tcl's object API.
pub fn tcl_string(rust_str: &str) -> TclBuf {
    let alloc_len = rust_str.len() + 1; // +1 for null terminator
    unsafe {
        let ptr = ALLOC.expect("module not initialized")(alloc_len) as *mut u8;
        let slice = std::slice::from_raw_parts_mut(ptr, alloc_len);
        slice[..rust_str.len()].copy_from_slice(rust_str.as_bytes());
        *slice.last_mut().expect("alloc_len is always >= 1") = 0;
        TclBuf::from_raw_parts(ptr, rust_str.len())
    }
}
