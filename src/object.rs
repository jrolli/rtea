use std::ffi::c_void;
use std::ffi::CStr;
use std::fmt::Display;
use std::os::raw::c_char;

use crate::tcl::*;
use crate::Interpreter;
use crate::TclStatus;

/// A wrapper for [Tcl objects](https://www.tcl.tk/man/tcl/TclLib/Object.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub struct RawObject {
    pub(crate) ref_count: i32,
    pub bytes: *mut c_char,
    pub length: i32,
    pub obj_type: *const ObjectType,
    pub ptr1: *mut c_void,
    pub ptr2: *mut c_void,
}

impl RawObject {
    pub fn wrap(obj: *mut RawObject) -> Object {
        unsafe { INCR_REF_COUNT(obj) };
        Object { obj: obj }
    }
}

#[derive(Debug)]
pub struct Object {
    pub obj: *mut RawObject,
}

impl Object {
    pub fn new() -> Object {
        unsafe { RawObject::wrap(NEW_OBJ.expect("module must have been initialized")()) }
    }

    pub fn new_string(s: &str) -> Object {
        unsafe {
            RawObject::wrap(NEW_STRING_OBJ.expect("module must have been initialized")(
                s.as_ptr() as *const i8,
                s.len() as i32,
            ))
        }
    }

    pub fn set_string(&self, s: &str) {
        unsafe {
            SET_STRING_OBJ.expect("module must have been initialized")(
                self.obj,
                s.as_ptr() as *const i8,
                s.len() as i32,
            );
        }
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

    /// Gets the Tcl ObjType Name
    pub fn get_type_name(&self) -> &str {
        let raw_obj = unsafe { &*self.obj };
        if raw_obj.obj_type.is_null() {
            "string"
        } else {
            unsafe {
                let obj_type = &*raw_obj.obj_type;
                CStr::from_ptr(obj_type.name as *const i8)
                    .to_str()
                    .expect("TCL guarantees string are valid UTF-8")
            }
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { DECR_REF_COUNT(self.obj) }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            let o_type = (*self.obj).obj_type;
            if (*self.obj).bytes.is_null()
                && !o_type.is_null()
                && o_type.as_ref().unwrap().update_string_proc.is_some()
            {
                let update_fn = o_type.as_ref().unwrap().update_string_proc.unwrap();
                update_fn(self.obj);
            } else if (*self.obj).bytes.is_null() {
                panic!("invalid string representation and no update function");
            }

            write!(f, "{}", CStr::from_ptr((*self.obj).bytes).to_str().unwrap())
        }
    }
}

/// A wrapper for [Tcl object types](https://www.tcl.tk/man/tcl/TclLib/ObjectType.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub struct ObjectType {
    pub name: *const u8,
    pub free_internal_rep_proc: Option<extern "C" fn(*mut RawObject)>,
    pub dup_internal_rep_proc: extern "C" fn(*const RawObject, *mut RawObject),
    pub update_string_proc: Option<extern "C" fn(*mut RawObject)>,
    pub set_from_any_proc: Option<extern "C" fn(*const Interpreter, *mut RawObject) -> TclStatus>,
}

unsafe impl Sync for ObjectType {}

pub trait TclObjectType: Clone + Display {
    fn from_object(obj: &Object) -> Option<&Self>;

    fn into_object(self) -> Object;

    fn type_name() -> &'static str;

    fn tcl_type() -> &'static ObjectType;

    fn as_string(&self) -> String {
        format!("{}", self)
    }

    fn convert(obj: Object) -> Result<Object, Object> {
        Err(obj)
    }
}
