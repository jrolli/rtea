//! This module wraps all of the TCL functions that need to static references
//! for object lifecycle management task to work.  Specifically, these are
//! functions which may need to be called in contexts where an interpreter
//! pointer is either not known (objects & object types) or immediately
//! available (methods on objects).

use std::ffi::c_void;
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

pub fn tcl_string(rust_str: &str) -> (*mut c_char, usize) {
    let tcl_alloc_len = rust_str.len() + 1;
    unsafe {
        {
            let tcl_buf = ALLOC.expect("module not initialized")(tcl_alloc_len) as *mut u8;
            let tcl_str = std::slice::from_raw_parts_mut(tcl_buf, tcl_alloc_len);
            tcl_str[..rust_str.len()].copy_from_slice(rust_str.as_bytes());
            if let Some(terminator) = tcl_str.last_mut() {
                *terminator = 0;
            }
            (tcl_str.as_ptr() as *mut c_char, rust_str.len())
        }
    }
}
