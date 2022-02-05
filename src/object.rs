use std::ffi::CStr;

use crate::tcl::*;
use crate::Interpreter;

/// A wrapper for [Tcl objects](https://www.tcl.tk/man/tcl/TclLib/Object.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub(crate) struct RawObject {
    pub(crate) ref_count: i32,
    pub(crate) bytes: *mut u8,
    pub(crate) length: i32,
    pub(crate) obj_type: *const ObjectType,
    _internal1: usize,
    _internal2: usize,
}

impl RawObject {
    pub(crate) fn wrap(obj: *mut RawObject) -> Object {
        unsafe { INCR_REF_COUNT(obj) };
        Object { obj: obj }
    }
}

#[derive(Debug)]
pub struct Object {
    obj: *mut RawObject,
}

impl Object {
    pub fn new() -> Object {
        unsafe { RawObject::wrap(NEW_OBJ.expect("module must have been initialized")()) }
    }

    /// Gets the string associated with the Tcl object.
    pub fn get_string(&self) -> &str {
        unsafe {
            CStr::from_ptr(GET_STRING.expect("module must have been initialized")(
                self.obj,
            ))
            .to_str()
            .expect("TCL guarantees strings are valid UTF-8")
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { DECR_REF_COUNT(self.obj) }
    }
}

/// A wrapper for [Tcl object types](https://www.tcl.tk/man/tcl/TclLib/ObjectType.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub struct ObjectType {
    _name: *const u8,
    pub(crate) free_internal_rep_proc: Option<extern "C" fn(*mut RawObject)>,
    _dup_internal_rep_proc: extern "C" fn(*const RawObject, *mut RawObject),
    _update_string_proc: Option<extern "C" fn(*mut RawObject)>,
    _set_from_any_proc: Option<extern "C" fn(*const Interpreter, *mut RawObject)>,
}
