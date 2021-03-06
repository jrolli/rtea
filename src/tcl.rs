//! This module wraps all of the TCL functions that need to static references
//! for object lifecycle management task to work.  Specifically, these are
//! functions which may need to be called in contexts where an interpreter
//! pointer is either not known (objects & object types) or immediately
//! available (methods on objects).

use std::os::raw::c_char;

use crate::Interpreter;
use crate::ObjectType;
use crate::RawObject;
use crate::TclStatus;

pub(crate) static mut ALLOC: Option<extern "C" fn(usize) -> *mut u8> = None;
pub(crate) static mut REALLOC: Option<extern "C" fn(*mut u8, usize) -> *mut u8> = None;
pub(crate) static mut FREE: Option<extern "C" fn(*mut u8)> = None;

pub(crate) static mut NEW_OBJ: Option<extern "C" fn() -> *mut RawObject> = None;
pub(crate) static mut FREE_OBJ: Option<extern "C" fn(*mut RawObject)> = None;
pub(crate) static mut DUPLICATE_OBJ: Option<extern "C" fn(*const RawObject) -> *mut RawObject> =
    None;
pub(crate) static mut INCR_REF_COUNT: extern "C" fn(*mut RawObject) = incr_ref_count;
pub(crate) static mut DECR_REF_COUNT: extern "C" fn(*mut RawObject) = decr_ref_count;
pub(crate) static mut IS_SHARED: extern "C" fn(*const RawObject) -> isize = is_shared;
pub(crate) static mut INVALIDATE_STRING_REP: Option<extern "C" fn(*mut RawObject)> = None;
pub(crate) static mut GET_STRING: Option<extern "C" fn(*const RawObject) -> *const c_char> = None;

pub(crate) static mut GET_OBJ_TYPE: Option<extern "C" fn(*const u8) -> *const ObjectType> = None;
pub(crate) static mut CONVERT_TO_TYPE: Option<
    extern "C" fn(*const Interpreter, *mut RawObject, *const ObjectType) -> TclStatus,
> = None;

pub(crate) static mut NEW_STRING_OBJ: Option<extern "C" fn(*const c_char, i32) -> *mut RawObject> =
    None;

pub(crate) static mut SET_STRING_OBJ: Option<extern "C" fn(*mut RawObject, *const c_char, i32)> =
    None;

pub fn tcl_string(rust_str: &str) -> (*mut c_char, usize) {
    let tcl_alloc_len = rust_str.len() + 1;
    unsafe {
        {
            let tcl_buf = ALLOC.expect("module not initialized")(tcl_alloc_len);
            let tcl_str = std::slice::from_raw_parts_mut(tcl_buf, tcl_alloc_len);
            tcl_str[..rust_str.len()].copy_from_slice(rust_str.as_bytes());
            if let Some(terminator) = tcl_str.last_mut() {
                *terminator = 0;
            }
            (tcl_str.as_ptr() as *mut c_char, rust_str.len())
        }
    }
}

extern "C" fn incr_ref_count(obj: *mut RawObject) {
    unsafe {
        obj.as_mut().expect("invalid invocation").ref_count += 1;
    }
}

extern "C" fn decr_ref_count(obj: *mut RawObject) {
    unsafe {
        let obj = obj.as_mut().expect("invalid invocation");
        assert!(obj.ref_count > 0);
        obj.ref_count -= 1;
        if obj.ref_count == 0 {
            if !obj.obj_type.is_null()
                && obj
                    .obj_type
                    .as_ref()
                    .expect("checked null alread")
                    .free_internal_rep_proc
                    .is_some()
            {
                obj.obj_type
                    .as_ref()
                    .expect("checked null alread")
                    .free_internal_rep_proc
                    .expect("checked for value")(obj);
            }
            FREE_OBJ.expect("module must be initialized first")(obj);
        }
    }
}

extern "C" fn is_shared(obj: *const RawObject) -> isize {
    unsafe {
        if obj.as_ref().expect("invalid invocation").ref_count > 1 {
            0
        } else {
            1
        }
    }
}
