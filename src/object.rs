use crate::Interpreter;

/// A wrapper for [TCL objects](https://www.tcl.tk/man/tcl/TclLib/Object.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub struct Object {
    _ref_count: isize,
    pub(crate) bytes: *mut u8,
    pub(crate) length: usize,
    _obj_type: *const ObjType,
    _internal: [u8; 8],
}

/// A wrapper for [TCL object types](https://www.tcl.tk/man/tcl/TclLib/ObjectType.html).
///
/// WIP
#[repr(C)]
#[derive(Debug)]
pub struct ObjType {
    _name: *const u8,
    _free_internal_rep_proc: Option<extern "C" fn(*mut Object)>,
    _dup_internal_rep_proc: extern "C" fn(*const Object, *mut Object),
    _update_string_proc: Option<extern "C" fn(*mut Object)>,
    _set_from_any_proc: Option<extern "C" fn(*const Interpreter, *mut Object)>,
}
