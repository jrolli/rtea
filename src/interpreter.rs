use std::ffi::CString;
use std::ffi::c_double;
use std::ffi::c_int;
use std::ffi::c_longlong;
use std::ffi::c_uint;
use std::ffi::c_ulonglong;
use std::ffi::c_void;
use std::os::raw::c_char;

use crate::Object;
use crate::ObjectType;
use crate::RawObject;
use crate::TclObjectType;
use crate::tcl::*;

/// A wrapper around a [Tcl](https://www.tcl.tk) interpreter object.
///
/// This is a wrapper around the Tcl interpreter object that leverages the
/// Stubs interface.  It makes assumptions about the interface (such as
/// stability of function pointers) that are made by the underlying stubs
/// implementation.  However, as a deliberate nod to the importance of the
/// borrow checker and other rust conventions, it obtains the stubs table on
/// each `command` rather than caching the version passed during the call to
/// an initialization routine.  While this sacrifices runtime performance
/// (extra indirection), it fits more with Rust paradigms and should reduce
/// the risk of trying to use the API without an associated interpeter.
#[repr(C)]
#[derive(Debug)]
pub struct Interpreter {
    _legacy_result: *const c_void,
    _legace_free_proc: *const c_void,
    _error_line: isize,
    stubs: *const Stubs,
}

type CmdProc = fn(interp: &Interpreter, args: Vec<&str>) -> Result<TclStatus, String>;
type ObjCmdProc = fn(interp: &Interpreter, args: Vec<Object>) -> Result<TclStatus, Object>;

const TCL_STUB_MAGIC: u32 = 0xFCA3BACB + size_of::<*const c_void>() as u32;

/// A wrapper for Tcl return status codes.
///
/// This is a simple wrapper around the expected return codes for Tcl
/// commands.  `Ok` and `Error` are the most common ones, but the others have
/// specific meanings under certain conditions (e.g., binding handlers in
/// Tk).  See the appropriate documentation for specific behavior.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum TclStatus {
    Ok = 0,
    Error = 1,
    Return = 2,
    Break = 3,
    Continue = 4,
}

impl From<i32> for TclStatus {
    fn from(val: i32) -> Self {
        match val {
            0 => TclStatus::Ok,
            2 => TclStatus::Return,
            3 => TclStatus::Break,
            4 => TclStatus::Continue,
            _ => TclStatus::Error,
        }
    }
}

/// A wrapper for values passed to Tcl's [unload](https://www.tcl.tk/man/tcl/TclCmd/unload.html) function.
#[repr(isize)]
pub enum TclUnloadFlag {
    /// Inidicates the interpreter is exiting but that the module's code is
    /// not being unmapped.
    DetachFromInterpreter = 1 << 0,
    /// Inidicates the last interpreter is detaching and the module is about
    /// to be unmapped from the process.
    DetachFromProcesss = 1 << 1,
}

const _TCL_STATIC: *const c_void = 0 as *const c_void;
const _TCL_VOLATILE: *const c_void = 1 as *const c_void;
const _TCL_DYNAMIC: *const c_void = 3 as *const c_void;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
struct Stubs {
    magic: u32,
    hooks: *const c_void,
    Tcl_PkgProvideEx:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, *const c_void) -> c_int, // 0
    Tcl_PkgRequireEx: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_int,
        *mut c_void,
    ) -> *const c_char, // 1
    Tcl_Panic: extern "C" fn(*const c_char, *const c_char), // 2
    Tcl_Alloc: extern "C" fn(usize) -> *mut c_void,         // 3
    Tcl_Free: extern "C" fn(*mut c_void),                   // 4
    Tcl_Realloc: extern "C" fn(*mut c_void, usize) -> *mut c_void, // 5
    Tcl_DbCkalloc: extern "C" fn(usize, *const c_char, c_int) -> *mut c_void, // 6
    Tcl_DbCkfree: extern "C" fn(*mut c_void, *const c_char, c_int), // 7
    Tcl_DbCkrealloc: extern "C" fn(*mut c_void, usize, *const c_char, c_int) -> *mut c_void, // 8
    Tcl_CreateFileHandler: extern "C" fn(c_int, c_int, *mut c_void, *mut c_void), // 9
    Tcl_DeleteFileHandler: extern "C" fn(c_int),            // 10
    Tcl_SetTimer: extern "C" fn(*const c_void),             // 11
    Tcl_Sleep: extern "C" fn(c_int),                        // 12
    Tcl_WaitForEvent: extern "C" fn(*const c_void) -> c_int, // 13
    Tcl_AppendAllObjTypes: extern "C" fn(*const Interpreter, *mut RawObject) -> c_int, // 14
    Tcl_AppendStringsToObj: extern "C" fn(*mut RawObject, *mut RawObject), // 15
    Tcl_AppendToObj: extern "C" fn(*mut RawObject, *const c_char, usize), // 16
    Tcl_ConcatObj: extern "C" fn(usize, *mut c_void) -> *mut RawObject, // 17
    Tcl_ConvertToType:
        extern "C" fn(*const Interpreter, *mut RawObject, *const ObjectType) -> c_int, // 18
    Tcl_DbDecrRefCount: extern "C" fn(*mut RawObject, *const c_char, c_int), // 19
    Tcl_DbIncrRefCount: extern "C" fn(*mut RawObject, *const c_char, c_int), // 20
    Tcl_DbIsShared: extern "C" fn(*mut RawObject, *const c_char, c_int) -> c_int, // 21
    _deprecated_22: *const c_void,                          // 22
    Tcl_DbNewByteArrayObj:
        extern "C" fn(*const c_void, usize, *const c_char, c_int) -> *mut RawObject, // 23
    Tcl_DbNewDoubleObj: extern "C" fn(c_double, *const c_char, c_int) -> *mut RawObject, // 24
    Tcl_DbNewListObj: extern "C" fn(usize, *mut c_void, *const c_char, c_int) -> *mut RawObject, // 25
    _deprecated_26: *const c_void, // 26
    Tcl_DbNewObj: extern "C" fn(*const c_char, c_int) -> *mut RawObject, // 27
    Tcl_DbNewStringObj: extern "C" fn(*const c_char, usize, *const c_char, c_int) -> *mut RawObject, // 28
    Tcl_DuplicateObj: extern "C" fn(*mut RawObject) -> *mut RawObject, // 29
    TclFreeObj: extern "C" fn(*mut RawObject),                         // 30
    Tcl_GetBoolean: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 31
    Tcl_GetBooleanFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 32
    Tcl_GetByteArrayFromObj: extern "C" fn(*mut RawObject, *mut c_void) -> *mut c_void, // 33
    Tcl_GetDouble: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 34
    Tcl_GetDoubleFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 35
    _deprecated_36: *const c_void, // 36
    Tcl_GetInt: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 37
    Tcl_GetIntFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 38
    Tcl_GetLongFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 39
    Tcl_GetObjType: extern "C" fn(*const c_char) -> *const ObjectType, // 40
    TclGetStringFromObj: extern "C" fn(*mut RawObject, *mut c_void) -> *mut c_void, // 41
    Tcl_InvalidateStringRep: extern "C" fn(*mut RawObject),            // 42
    Tcl_ListObjAppendList:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject) -> c_int, // 43
    Tcl_ListObjAppendElement:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject) -> c_int, // 44
    TclListObjGetElements:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void, *mut c_void) -> c_int, // 45
    Tcl_ListObjIndex:
        extern "C" fn(*const Interpreter, *mut RawObject, usize, *mut c_void) -> c_int, // 46
    TclListObjLength: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 47
    Tcl_ListObjReplace: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        usize,
        usize,
        usize,
        *mut c_void,
    ) -> c_int, // 48
    _deprecated_49: *const c_void,                                     // 49
    Tcl_NewByteArrayObj: extern "C" fn(*const c_void, usize) -> *mut RawObject, // 50
    Tcl_NewDoubleObj: extern "C" fn(c_double) -> *mut RawObject,       // 51
    _deprecated_52: *const c_void,                                     // 52
    Tcl_NewListObj: extern "C" fn(usize, *mut c_void) -> *mut RawObject, // 53
    _deprecated_54: *const c_void,                                     // 54
    Tcl_NewObj: extern "C" fn() -> *mut RawObject,                     // 55
    Tcl_NewStringObj: extern "C" fn(*const c_char, usize) -> *mut RawObject, // 56
    _deprecated_57: *const c_void,                                     // 57
    Tcl_SetByteArrayLength: extern "C" fn(*mut RawObject, usize) -> *mut c_void, // 58
    Tcl_SetByteArrayObj: extern "C" fn(*mut RawObject, *const c_void, usize), // 59
    Tcl_SetDoubleObj: extern "C" fn(*mut RawObject, c_double),         // 60
    _deprecated_61: *const c_void,                                     // 61
    Tcl_SetListObj: extern "C" fn(*mut RawObject, usize, *mut c_void), // 62
    _deprecated_63: *const c_void,                                     // 63
    Tcl_SetObjLength: extern "C" fn(*mut RawObject, usize),            // 64
    Tcl_SetStringObj: extern "C" fn(*mut RawObject, *const c_char, usize), // 65
    _deprecated_66: *const c_void,                                     // 66
    _deprecated_67: *const c_void,                                     // 67
    Tcl_AllowExceptions: extern "C" fn(*const Interpreter),            // 68
    Tcl_AppendElement: extern "C" fn(*const Interpreter, *const c_char), // 69
    Tcl_AppendResult: extern "C" fn(*const Interpreter, *const Interpreter), // 70
    Tcl_AsyncCreate: extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void, // 71
    Tcl_AsyncDelete: extern "C" fn(*mut c_void),                       // 72
    Tcl_AsyncInvoke: extern "C" fn(*const Interpreter, c_int) -> c_int, // 73
    Tcl_AsyncMark: extern "C" fn(*mut c_void),                         // 74
    Tcl_AsyncReady: extern "C" fn() -> c_int,                          // 75
    _deprecated_76: *const c_void,                                     // 76
    _deprecated_77: *const c_void,                                     // 77
    Tcl_BadChannelOption: extern "C" fn(*const Interpreter, *const c_char, *const c_char) -> c_int, // 78
    Tcl_CallWhenDeleted: extern "C" fn(*const Interpreter, *mut c_void, *mut c_void), // 79
    Tcl_CancelIdleCall: extern "C" fn(*mut c_void, *mut c_void),                      // 80
    Tcl_Close: extern "C" fn(*const Interpreter, *mut c_void) -> c_int,               // 81
    Tcl_CommandComplete: extern "C" fn(*const c_char) -> c_int,                       // 82
    Tcl_Concat: extern "C" fn(usize, *const c_void) -> *mut c_void,                   // 83
    Tcl_ConvertElement: extern "C" fn(*const c_char, *mut c_void, c_int) -> usize,    // 84
    Tcl_ConvertCountedElement: extern "C" fn(*const c_char, usize, *mut c_void, c_int) -> usize, // 85
    Tcl_CreateAlias: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const Interpreter,
        *const c_char,
        usize,
        *const c_void,
    ) -> c_int, // 86
    Tcl_CreateAliasObj: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const Interpreter,
        *const c_char,
        usize,
        *mut c_void,
    ) -> c_int, // 87
    Tcl_CreateChannel:
        extern "C" fn(*const c_void, *const c_char, *mut c_void, c_int) -> *mut c_void, // 88
    Tcl_CreateChannelHandler: extern "C" fn(*mut c_void, c_int, *mut c_void, *mut c_void), // 89
    Tcl_CreateCloseHandler: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),          // 90
    Tcl_CreateCommand: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 91
    Tcl_CreateEventSource: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),           // 92
    Tcl_CreateExitHandler: extern "C" fn(*mut c_void, *mut c_void),                        // 93
    Tcl_CreateInterp: extern "C" fn() -> *const Interpreter,                               // 94
    _deprecated_95: *const c_void,                                                         // 95
    Tcl_CreateObjCommand: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 96
    Tcl_CreateChild: extern "C" fn(*const Interpreter, *const c_char, c_int) -> *const Interpreter, // 97
    Tcl_CreateTimerHandler: extern "C" fn(c_int, *mut c_void, *mut c_void) -> *mut c_void, // 98
    Tcl_CreateTrace:
        extern "C" fn(*const Interpreter, usize, *mut c_void, *mut c_void) -> *mut c_void, // 99
    Tcl_DeleteAssocData: extern "C" fn(*const Interpreter, *const c_char),                 // 100
    Tcl_DeleteChannelHandler: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),        // 101
    Tcl_DeleteCloseHandler: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),          // 102
    Tcl_DeleteCommand: extern "C" fn(*const Interpreter, *const c_char) -> c_int,          // 103
    Tcl_DeleteCommandFromToken: extern "C" fn(*const Interpreter, *mut c_void) -> c_int,   // 104
    Tcl_DeleteEvents: extern "C" fn(*mut c_void, *mut c_void),                             // 105
    Tcl_DeleteEventSource: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),           // 106
    Tcl_DeleteExitHandler: extern "C" fn(*mut c_void, *mut c_void),                        // 107
    Tcl_DeleteHashEntry: extern "C" fn(*mut c_void),                                       // 108
    Tcl_DeleteHashTable: extern "C" fn(*mut c_void),                                       // 109
    Tcl_DeleteInterp: extern "C" fn(*const Interpreter),                                   // 110
    Tcl_DetachPids: extern "C" fn(usize, *mut c_void),                                     // 111
    Tcl_DeleteTimerHandler: extern "C" fn(*mut c_void),                                    // 112
    Tcl_DeleteTrace: extern "C" fn(*const Interpreter, *mut c_void),                       // 113
    Tcl_DontCallWhenDeleted: extern "C" fn(*const Interpreter, *mut c_void, *mut c_void),  // 114
    Tcl_DoOneEvent: extern "C" fn(c_int) -> c_int,                                         // 115
    Tcl_DoWhenIdle: extern "C" fn(*mut c_void, *mut c_void),                               // 116
    Tcl_DStringAppend: extern "C" fn(*mut c_void, *const c_char, usize) -> *mut c_void,    // 117
    Tcl_DStringAppendElement: extern "C" fn(*mut c_void, *const c_char) -> *mut c_void,    // 118
    Tcl_DStringEndSublist: extern "C" fn(*mut c_void),                                     // 119
    Tcl_DStringFree: extern "C" fn(*mut c_void),                                           // 120
    Tcl_DStringGetResult: extern "C" fn(*const Interpreter, *mut c_void),                  // 121
    Tcl_DStringInit: extern "C" fn(*mut c_void),                                           // 122
    Tcl_DStringResult: extern "C" fn(*const Interpreter, *mut c_void),                     // 123
    Tcl_DStringSetLength: extern "C" fn(*mut c_void, usize),                               // 124
    Tcl_DStringStartSublist: extern "C" fn(*mut c_void),                                   // 125
    Tcl_Eof: extern "C" fn(*mut c_void) -> c_int,                                          // 126
    Tcl_ErrnoId: extern "C" fn() -> *const c_char,                                         // 127
    Tcl_ErrnoMsg: extern "C" fn(c_int) -> *const c_char,                                   // 128
    _deprecated_129: *const c_void,                                                        // 129
    Tcl_EvalFile: extern "C" fn(*const Interpreter, *const c_char) -> c_int,               // 130
    _deprecated_131: *const c_void,                                                        // 131
    Tcl_EventuallyFree: extern "C" fn(*mut c_void, *mut c_void),                           // 132
    Tcl_Exit: extern "C" fn(c_int),                                                        // 133
    Tcl_ExposeCommand: extern "C" fn(*const Interpreter, *const c_char, *const c_char) -> c_int, // 134
    Tcl_ExprBoolean: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 135
    Tcl_ExprBooleanObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 136
    Tcl_ExprDouble: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 137
    Tcl_ExprDoubleObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 138
    Tcl_ExprLong: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 139
    Tcl_ExprLongObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 140
    Tcl_ExprObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 141
    Tcl_ExprString: extern "C" fn(*const Interpreter, *const c_char) -> c_int,            // 142
    Tcl_Finalize: extern "C" fn(),                                                        // 143
    _deprecated_144: *const c_void,                                                       // 144
    Tcl_FirstHashEntry: extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,           // 145
    Tcl_Flush: extern "C" fn(*mut c_void) -> c_int,                                       // 146
    _deprecated_147: *const c_void,                                                       // 147
    _deprecated_148: *const c_void,                                                       // 148
    TclGetAliasObj: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *const c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 149
    Tcl_GetAssocData: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> *mut c_void, // 150
    Tcl_GetChannel: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> *mut c_void, // 151
    Tcl_GetChannelBufferSize: extern "C" fn(*mut c_void) -> usize, // 152
    Tcl_GetChannelHandle: extern "C" fn(*mut c_void, c_int, *mut c_void) -> c_int, // 153
    Tcl_GetChannelInstanceData: extern "C" fn(*mut c_void) -> *mut c_void, // 154
    Tcl_GetChannelMode: extern "C" fn(*mut c_void) -> c_int,       // 155
    Tcl_GetChannelName: extern "C" fn(*mut c_void) -> *const c_char, // 156
    Tcl_GetChannelOption:
        extern "C" fn(*const Interpreter, *mut c_void, *const c_char, *mut c_void) -> c_int, // 157
    Tcl_GetChannelType: extern "C" fn(*mut c_void) -> *const c_void, // 158
    Tcl_GetCommandInfo: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 159
    Tcl_GetCommandName: extern "C" fn(*const Interpreter, *mut c_void) -> *const c_char, // 160
    Tcl_GetErrno: extern "C" fn() -> c_int,                                              // 161
    Tcl_GetHostName: extern "C" fn() -> *const c_char,                                   // 162
    Tcl_GetInterpPath: extern "C" fn(*const Interpreter, *const Interpreter) -> c_int,   // 163
    Tcl_GetParent: extern "C" fn(*const Interpreter) -> *const Interpreter,              // 164
    Tcl_GetNameOfExecutable: extern "C" fn() -> *const c_char,                           // 165
    Tcl_GetObjResult: extern "C" fn(*const Interpreter) -> *mut RawObject,               // 166
    Tcl_GetOpenFile:
        extern "C" fn(*const Interpreter, *const c_char, c_int, c_int, *mut c_void) -> c_int, // 167
    Tcl_GetPathType: extern "C" fn(*const c_char) -> *mut c_void,                        // 168
    Tcl_Gets: extern "C" fn(*mut c_void, *mut c_void) -> usize,                          // 169
    Tcl_GetsObj: extern "C" fn(*mut c_void, *mut RawObject) -> usize,                    // 170
    Tcl_GetServiceMode: extern "C" fn() -> c_int,                                        // 171
    Tcl_GetChild: extern "C" fn(*const Interpreter, *const c_char) -> *const Interpreter, // 172
    Tcl_GetStdChannel: extern "C" fn(c_int) -> *mut c_void,                              // 173
    _deprecated_174: *const c_void,                                                      // 174
    _deprecated_175: *const c_void,                                                      // 175
    Tcl_GetVar2:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, c_int) -> *const c_char, // 176
    _deprecated_177: *const c_void, // 177
    _deprecated_178: *const c_void, // 178
    Tcl_HideCommand: extern "C" fn(*const Interpreter, *const c_char, *const c_char) -> c_int, // 179
    Tcl_Init: extern "C" fn(*const Interpreter) -> c_int, // 180
    Tcl_InitHashTable: extern "C" fn(*mut c_void, c_int), // 181
    Tcl_InputBlocked: extern "C" fn(*mut c_void) -> c_int, // 182
    Tcl_InputBuffered: extern "C" fn(*mut c_void) -> c_int, // 183
    Tcl_InterpDeleted: extern "C" fn(*const Interpreter) -> c_int, // 184
    Tcl_IsSafe: extern "C" fn(*const Interpreter) -> c_int, // 185
    Tcl_JoinPath: extern "C" fn(usize, *const c_void, *mut c_void) -> *mut c_void, // 186
    Tcl_LinkVar: extern "C" fn(*const Interpreter, *const c_char, *mut c_void, c_int) -> c_int, // 187
    _deprecated_188: *const c_void, // 188
    Tcl_MakeFileChannel: extern "C" fn(*mut c_void, c_int) -> *mut c_void, // 189
    _deprecated_190: *const c_void, // 190
    Tcl_MakeTcpClientChannel: extern "C" fn(*mut c_void) -> *mut c_void, // 191
    Tcl_Merge: extern "C" fn(usize, *const c_void) -> *mut c_void, // 192
    Tcl_NextHashEntry: extern "C" fn(*mut c_void) -> *mut c_void, // 193
    Tcl_NotifyChannel: extern "C" fn(*mut c_void, c_int), // 194
    Tcl_ObjGetVar2:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject, c_int) -> *mut RawObject, // 195
    Tcl_ObjSetVar2: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *mut RawObject,
        *mut RawObject,
        c_int,
    ) -> *mut RawObject, // 196
    Tcl_OpenCommandChannel:
        extern "C" fn(*const Interpreter, usize, *const c_void, c_int) -> *mut c_void, // 197
    Tcl_OpenFileChannel:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, c_int) -> *mut c_void, // 198
    Tcl_OpenTcpClient: extern "C" fn(
        *const Interpreter,
        c_int,
        *const c_char,
        *const c_char,
        c_int,
        c_int,
    ) -> *mut c_void, // 199
    Tcl_OpenTcpServer: extern "C" fn(
        *const Interpreter,
        c_int,
        *const c_char,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 200
    Tcl_Preserve: extern "C" fn(*mut c_void), // 201
    Tcl_PrintDouble: extern "C" fn(*const Interpreter, c_double, *mut c_void), // 202
    Tcl_PutEnv: extern "C" fn(*const c_char) -> c_int, // 203
    Tcl_PosixError: extern "C" fn(*const Interpreter) -> *const c_char, // 204
    Tcl_QueueEvent: extern "C" fn(*mut c_void, c_int), // 205
    Tcl_Read: extern "C" fn(*mut c_void, *mut c_void, usize) -> usize, // 206
    Tcl_ReapDetachedProcs: extern "C" fn(),   // 207
    Tcl_RecordAndEval: extern "C" fn(*const Interpreter, *const c_char, c_int) -> c_int, // 208
    Tcl_RecordAndEvalObj: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> c_int, // 209
    Tcl_RegisterChannel: extern "C" fn(*const Interpreter, *mut c_void), // 210
    Tcl_RegisterObjType: extern "C" fn(*const ObjectType), // 211
    Tcl_RegExpCompile: extern "C" fn(*const Interpreter, *const c_char) -> *mut c_void, // 212
    Tcl_RegExpExec:
        extern "C" fn(*const Interpreter, *mut c_void, *const c_char, *const c_char) -> c_int, // 213
    Tcl_RegExpMatch: extern "C" fn(*const Interpreter, *const c_char, *const c_char) -> c_int, // 214
    Tcl_RegExpRange: extern "C" fn(*mut c_void, usize, *const c_void, *const c_void), // 215
    Tcl_Release: extern "C" fn(*mut c_void),                                          // 216
    Tcl_ResetResult: extern "C" fn(*const Interpreter),                               // 217
    Tcl_ScanElement: extern "C" fn(*const c_char, *mut c_void) -> usize,              // 218
    Tcl_ScanCountedElement: extern "C" fn(*const c_char, usize, *mut c_void) -> usize, // 219
    _deprecated_220: *const c_void,                                                   // 220
    Tcl_ServiceAll: extern "C" fn() -> c_int,                                         // 221
    Tcl_ServiceEvent: extern "C" fn(c_int) -> c_int,                                  // 222
    Tcl_SetAssocData: extern "C" fn(*const Interpreter, *const c_char, *mut c_void, *mut c_void), // 223
    Tcl_SetChannelBufferSize: extern "C" fn(*mut c_void, usize), // 224
    Tcl_SetChannelOption:
        extern "C" fn(*const Interpreter, *mut c_void, *const c_char, *const c_char) -> c_int, // 225
    Tcl_SetCommandInfo: extern "C" fn(*const Interpreter, *const c_char, *const c_void) -> c_int, // 226
    Tcl_SetErrno: extern "C" fn(c_int), // 227
    Tcl_SetErrorCode: extern "C" fn(*const Interpreter, *const Interpreter), // 228
    Tcl_SetMaxBlockTime: extern "C" fn(*const c_void), // 229
    _deprecated_230: *const c_void,     // 230
    Tcl_SetRecursionLimit: extern "C" fn(*const Interpreter, usize) -> usize, // 231
    _deprecated_232: *const c_void,     // 232
    Tcl_SetServiceMode: extern "C" fn(c_int) -> c_int, // 233
    Tcl_SetObjErrorCode: extern "C" fn(*const Interpreter, *mut RawObject), // 234
    Tcl_SetObjResult: extern "C" fn(*const Interpreter, *mut RawObject), // 235
    Tcl_SetStdChannel: extern "C" fn(*mut c_void, c_int), // 236
    _deprecated_237: *const c_void,     // 237
    Tcl_SetVar2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        *const c_char,
        c_int,
    ) -> *const c_char, // 238
    Tcl_SignalId: extern "C" fn(c_int) -> *const c_char, // 239
    Tcl_SignalMsg: extern "C" fn(c_int) -> *const c_char, // 240
    Tcl_SourceRCFile: extern "C" fn(*const Interpreter), // 241
    TclSplitList:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, *const c_void) -> c_int, // 242
    TclSplitPath: extern "C" fn(*const c_char, *mut c_void, *const c_void), // 243
    _deprecated_244: *const c_void,                                         // 244
    _deprecated_245: *const c_void,                                         // 245
    _deprecated_246: *const c_void,                                         // 246
    _deprecated_247: *const c_void,                                         // 247
    Tcl_TraceVar2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 248
    Tcl_TranslateFileName:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> *mut c_void, // 249
    Tcl_Ungets: extern "C" fn(*mut c_void, *const c_char, usize, c_int) -> usize, // 250
    Tcl_UnlinkVar: extern "C" fn(*const Interpreter, *const c_char),        // 251
    Tcl_UnregisterChannel: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 252
    _deprecated_253: *const c_void,                                         // 253
    Tcl_UnsetVar2: extern "C" fn(*const Interpreter, *const c_char, *const c_char, c_int) -> c_int, // 254
    _deprecated_255: *const c_void, // 255
    Tcl_UntraceVar2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_int,
        *mut c_void,
        *mut c_void,
    ), // 256
    Tcl_UpdateLinkedVar: extern "C" fn(*const Interpreter, *const c_char), // 257
    _deprecated_258: *const c_void, // 258
    Tcl_UpVar2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        *const c_char,
        *const c_char,
        c_int,
    ) -> c_int, // 259
    Tcl_VarEval: extern "C" fn(*const Interpreter, *const Interpreter) -> c_int, // 260
    _deprecated_261: *const c_void, // 261
    Tcl_VarTraceInfo2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 262
    Tcl_Write: extern "C" fn(*mut c_void, *const c_char, usize) -> usize, // 263
    Tcl_WrongNumArgs: extern "C" fn(*const Interpreter, usize, *mut c_void, *const c_char), // 264
    Tcl_DumpActiveMemory: extern "C" fn(*const c_char) -> c_int, // 265
    Tcl_ValidateAllMemory: extern "C" fn(*const c_char, c_int), // 266
    _deprecated_267: *const c_void, // 267
    _deprecated_268: *const c_void, // 268
    Tcl_HashStats: extern "C" fn(*mut c_void) -> *mut c_void, // 269
    Tcl_ParseVar: extern "C" fn(*const Interpreter, *const c_char, *const c_void) -> *const c_char, // 270
    _deprecated_271: *const c_void, // 271
    Tcl_PkgPresentEx: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_int,
        *mut c_void,
    ) -> *const c_char, // 272
    _deprecated_273: *const c_void, // 273
    _deprecated_274: *const c_void, // 274
    _deprecated_275: *const c_void, // 275
    _deprecated_276: *const c_void, // 276
    Tcl_WaitPid: extern "C" fn(*mut c_void, *mut c_void, c_int) -> *mut c_void, // 277
    _deprecated_278: *const c_void, // 278
    Tcl_GetVersion: extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void), // 279
    Tcl_InitMemory: extern "C" fn(*const Interpreter), // 280
    Tcl_StackChannel: extern "C" fn(
        *const Interpreter,
        *const c_void,
        *mut c_void,
        c_int,
        *mut c_void,
    ) -> *mut c_void, // 281
    Tcl_UnstackChannel: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 282
    Tcl_GetStackedChannel: extern "C" fn(*mut c_void) -> *mut c_void, // 283
    Tcl_SetMainLoop: extern "C" fn(*mut c_void), // 284
    Tcl_GetAliasObj: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *const c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 285
    Tcl_AppendObjToObj: extern "C" fn(*mut RawObject, *mut RawObject), // 286
    Tcl_CreateEncoding: extern "C" fn(*const c_void) -> *mut c_void, // 287
    Tcl_CreateThreadExitHandler: extern "C" fn(*mut c_void, *mut c_void), // 288
    Tcl_DeleteThreadExitHandler: extern "C" fn(*mut c_void, *mut c_void), // 289
    _deprecated_290: *const c_void, // 290
    Tcl_EvalEx: extern "C" fn(*const Interpreter, *const c_char, usize, c_int) -> c_int, // 291
    Tcl_EvalObjv: extern "C" fn(*const Interpreter, usize, *mut c_void, c_int) -> c_int, // 292
    Tcl_EvalObjEx: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> c_int, // 293
    Tcl_ExitThread: extern "C" fn(c_int), // 294
    Tcl_ExternalToUtf: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *const c_char,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
        usize,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 295
    Tcl_ExternalToUtfDString:
        extern "C" fn(*mut c_void, *const c_char, usize, *mut c_void) -> *mut c_void, // 296
    Tcl_FinalizeThread: extern "C" fn(), // 297
    Tcl_FinalizeNotifier: extern "C" fn(*mut c_void), // 298
    Tcl_FreeEncoding: extern "C" fn(*mut c_void), // 299
    Tcl_GetCurrentThread: extern "C" fn() -> *mut c_void, // 300
    Tcl_GetEncoding: extern "C" fn(*const Interpreter, *const c_char) -> *mut c_void, // 301
    Tcl_GetEncodingName: extern "C" fn(*mut c_void) -> *const c_char, // 302
    Tcl_GetEncodingNames: extern "C" fn(*const Interpreter), // 303
    Tcl_GetIndexFromObjStruct: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *const c_void,
        usize,
        *const c_char,
        c_int,
        *mut c_void,
    ) -> c_int, // 304
    Tcl_GetThreadData: extern "C" fn(*mut c_void, usize) -> *mut c_void, // 305
    Tcl_GetVar2Ex:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, c_int) -> *mut RawObject, // 306
    Tcl_InitNotifier: extern "C" fn() -> *mut c_void, // 307
    Tcl_MutexLock: extern "C" fn(*mut c_void),        // 308
    Tcl_MutexUnlock: extern "C" fn(*mut c_void),      // 309
    Tcl_ConditionNotify: extern "C" fn(*mut c_void),  // 310
    Tcl_ConditionWait: extern "C" fn(*mut c_void, *mut c_void, *const c_void), // 311
    TclNumUtfChars: extern "C" fn(*const c_char, usize) -> usize, // 312
    Tcl_ReadChars: extern "C" fn(*mut c_void, *mut RawObject, usize, c_int) -> usize, // 313
    _deprecated_314: *const c_void,                   // 314
    _deprecated_315: *const c_void,                   // 315
    Tcl_SetSystemEncoding: extern "C" fn(*const Interpreter, *const c_char) -> c_int, // 316
    Tcl_SetVar2Ex: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        *mut RawObject,
        c_int,
    ) -> *mut RawObject, // 317
    Tcl_ThreadAlert: extern "C" fn(*mut c_void),      // 318
    Tcl_ThreadQueueEvent: extern "C" fn(*mut c_void, *mut c_void, c_int), // 319
    Tcl_UniCharAtIndex: extern "C" fn(*const c_char, usize) -> c_int, // 320
    Tcl_UniCharToLower: extern "C" fn(c_int) -> c_int, // 321
    Tcl_UniCharToTitle: extern "C" fn(c_int) -> c_int, // 322
    Tcl_UniCharToUpper: extern "C" fn(c_int) -> c_int, // 323
    Tcl_UniCharToUtf: extern "C" fn(c_int, *mut c_void) -> usize, // 324
    TclUtfAtIndex: extern "C" fn(*const c_char, usize) -> *const c_char, // 325
    TclUtfCharComplete: extern "C" fn(*const c_char, usize) -> c_int, // 326
    Tcl_UtfBackslash: extern "C" fn(*const c_char, *mut c_void, *mut c_void) -> usize, // 327
    Tcl_UtfFindFirst: extern "C" fn(*const c_char, c_int) -> *const c_char, // 328
    Tcl_UtfFindLast: extern "C" fn(*const c_char, c_int) -> *const c_char, // 329
    TclUtfNext: extern "C" fn(*const c_char) -> *const c_char, // 330
    TclUtfPrev: extern "C" fn(*const c_char, *const c_char) -> *const c_char, // 331
    Tcl_UtfToExternal: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *const c_char,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
        usize,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 332
    Tcl_UtfToExternalDString:
        extern "C" fn(*mut c_void, *const c_char, usize, *mut c_void) -> *mut c_void, // 333
    Tcl_UtfToLower: extern "C" fn(*mut c_void) -> usize, // 334
    Tcl_UtfToTitle: extern "C" fn(*mut c_void) -> usize, // 335
    Tcl_UtfToChar16: extern "C" fn(*const c_char, *mut c_void) -> usize, // 336
    Tcl_UtfToUpper: extern "C" fn(*mut c_void) -> usize, // 337
    Tcl_WriteChars: extern "C" fn(*mut c_void, *const c_char, usize) -> usize, // 338
    Tcl_WriteObj: extern "C" fn(*mut c_void, *mut RawObject) -> usize, // 339
    Tcl_GetString: extern "C" fn(*mut RawObject) -> *mut c_char, // 340
    _deprecated_341: *const c_void,                   // 341
    _deprecated_342: *const c_void,                   // 342
    Tcl_AlertNotifier: extern "C" fn(*mut c_void),    // 343
    Tcl_ServiceModeHook: extern "C" fn(c_int),        // 344
    Tcl_UniCharIsAlnum: extern "C" fn(c_int) -> c_int, // 345
    Tcl_UniCharIsAlpha: extern "C" fn(c_int) -> c_int, // 346
    Tcl_UniCharIsDigit: extern "C" fn(c_int) -> c_int, // 347
    Tcl_UniCharIsLower: extern "C" fn(c_int) -> c_int, // 348
    Tcl_UniCharIsSpace: extern "C" fn(c_int) -> c_int, // 349
    Tcl_UniCharIsUpper: extern "C" fn(c_int) -> c_int, // 350
    Tcl_UniCharIsWordChar: extern "C" fn(c_int) -> c_int, // 351
    Tcl_Char16Len: extern "C" fn(*const c_void) -> usize, // 352
    _deprecated_353: *const c_void,                   // 353
    Tcl_Char16ToUtfDString: extern "C" fn(*const c_void, usize, *mut c_void) -> *mut c_void, // 354
    Tcl_UtfToChar16DString: extern "C" fn(*const c_char, usize, *mut c_void) -> *mut c_void, // 355
    Tcl_GetRegExpFromObj: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> *mut c_void, // 356
    _deprecated_357: *const c_void,            // 357
    Tcl_FreeParse: extern "C" fn(*mut c_void), // 358
    Tcl_LogCommandInfo: extern "C" fn(*const Interpreter, *const c_char, *const c_char, usize), // 359
    Tcl_ParseBraces: extern "C" fn(
        *const Interpreter,
        *const c_char,
        usize,
        *mut c_void,
        c_int,
        *const c_void,
    ) -> c_int, // 360
    Tcl_ParseCommand:
        extern "C" fn(*const Interpreter, *const c_char, usize, c_int, *mut c_void) -> c_int, // 361
    Tcl_ParseExpr: extern "C" fn(*const Interpreter, *const c_char, usize, *mut c_void) -> c_int, // 362
    Tcl_ParseQuotedString: extern "C" fn(
        *const Interpreter,
        *const c_char,
        usize,
        *mut c_void,
        c_int,
        *const c_void,
    ) -> c_int, // 363
    Tcl_ParseVarName:
        extern "C" fn(*const Interpreter, *const c_char, usize, *mut c_void, c_int) -> c_int, // 364
    Tcl_GetCwd: extern "C" fn(*const Interpreter, *mut c_void) -> *mut c_void, // 365
    Tcl_Chdir: extern "C" fn(*const c_char) -> c_int,                          // 366
    Tcl_Access: extern "C" fn(*const c_char, c_int) -> c_int,                  // 367
    Tcl_Stat: extern "C" fn(*const c_char, *mut c_void) -> c_int,              // 368
    TclUtfNcmp: extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, // 369
    TclUtfNcasecmp: extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, // 370
    Tcl_StringCaseMatch: extern "C" fn(*const c_char, *const c_char, c_int) -> c_int, // 371
    Tcl_UniCharIsControl: extern "C" fn(c_int) -> c_int,                       // 372
    Tcl_UniCharIsGraph: extern "C" fn(c_int) -> c_int,                         // 373
    Tcl_UniCharIsPrint: extern "C" fn(c_int) -> c_int,                         // 374
    Tcl_UniCharIsPunct: extern "C" fn(c_int) -> c_int,                         // 375
    Tcl_RegExpExecObj: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *mut RawObject,
        usize,
        usize,
        c_int,
    ) -> c_int, // 376
    Tcl_RegExpGetInfo: extern "C" fn(*mut c_void, *mut c_void),                // 377
    Tcl_NewUnicodeObj: extern "C" fn(*const c_void, usize) -> *mut RawObject,  // 378
    Tcl_SetUnicodeObj: extern "C" fn(*mut RawObject, *const c_void, usize),    // 379
    TclGetCharLength: extern "C" fn(*mut RawObject) -> usize,                  // 380
    TclGetUniChar: extern "C" fn(*mut RawObject, usize) -> c_int,              // 381
    _deprecated_382: *const c_void,                                            // 382
    TclGetRange: extern "C" fn(*mut RawObject, usize, usize) -> *mut RawObject, // 383
    Tcl_AppendUnicodeToObj: extern "C" fn(*mut RawObject, *const c_void, usize), // 384
    Tcl_RegExpMatchObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject) -> c_int, // 385
    Tcl_SetNotifier: extern "C" fn(*const c_void), // 386
    Tcl_GetAllocMutex: extern "C" fn() -> *mut c_void, // 387
    Tcl_GetChannelNames: extern "C" fn(*const Interpreter) -> c_int, // 388
    Tcl_GetChannelNamesEx: extern "C" fn(*const Interpreter, *const c_char) -> c_int, // 389
    Tcl_ProcObjCmd: extern "C" fn(*mut c_void, *const Interpreter, usize, *mut c_void) -> c_int, // 390
    Tcl_ConditionFinalize: extern "C" fn(*mut c_void), // 391
    Tcl_MutexFinalize: extern "C" fn(*mut c_void),     // 392
    Tcl_CreateThread: extern "C" fn(*mut c_void, *mut c_void, *mut c_void, usize, c_int) -> c_int, // 393
    Tcl_ReadRaw: extern "C" fn(*mut c_void, *mut c_void, usize) -> usize, // 394
    Tcl_WriteRaw: extern "C" fn(*mut c_void, *const c_char, usize) -> usize, // 395
    Tcl_GetTopChannel: extern "C" fn(*mut c_void) -> *mut c_void,         // 396
    Tcl_ChannelBuffered: extern "C" fn(*mut c_void) -> c_int,             // 397
    Tcl_ChannelName: extern "C" fn(*const c_void) -> *const c_char,       // 398
    Tcl_ChannelVersion: extern "C" fn(*const c_void) -> *mut c_void,      // 399
    Tcl_ChannelBlockModeProc: extern "C" fn(*const c_void) -> *mut c_void, // 400
    _deprecated_401: *const c_void,                                       // 401
    Tcl_ChannelClose2Proc: extern "C" fn(*const c_void) -> *mut c_void,   // 402
    Tcl_ChannelInputProc: extern "C" fn(*const c_void) -> *mut c_void,    // 403
    Tcl_ChannelOutputProc: extern "C" fn(*const c_void) -> *mut c_void,   // 404
    _deprecated_405: *const c_void,                                       // 405
    Tcl_ChannelSetOptionProc: extern "C" fn(*const c_void) -> *mut c_void, // 406
    Tcl_ChannelGetOptionProc: extern "C" fn(*const c_void) -> *mut c_void, // 407
    Tcl_ChannelWatchProc: extern "C" fn(*const c_void) -> *mut c_void,    // 408
    Tcl_ChannelGetHandleProc: extern "C" fn(*const c_void) -> *mut c_void, // 409
    Tcl_ChannelFlushProc: extern "C" fn(*const c_void) -> *mut c_void,    // 410
    Tcl_ChannelHandlerProc: extern "C" fn(*const c_void) -> *mut c_void,  // 411
    Tcl_JoinThread: extern "C" fn(*mut c_void, *mut c_void) -> c_int,     // 412
    Tcl_IsChannelShared: extern "C" fn(*mut c_void) -> c_int,             // 413
    Tcl_IsChannelRegistered: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 414
    Tcl_CutChannel: extern "C" fn(*mut c_void),                           // 415
    Tcl_SpliceChannel: extern "C" fn(*mut c_void),                        // 416
    Tcl_ClearChannelHandlers: extern "C" fn(*mut c_void),                 // 417
    Tcl_IsChannelExisting: extern "C" fn(*const c_char) -> c_int,         // 418
    _deprecated_419: *const c_void,                                       // 419
    _deprecated_420: *const c_void,                                       // 420
    _deprecated_421: *const c_void,                                       // 421
    _deprecated_422: *const c_void,                                       // 422
    Tcl_InitCustomHashTable: extern "C" fn(*mut c_void, c_int, *const c_void), // 423
    Tcl_InitObjHashTable: extern "C" fn(*mut c_void),                     // 424
    Tcl_CommandTraceInfo: extern "C" fn(
        *const Interpreter,
        *const c_char,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 425
    Tcl_TraceCommand:
        extern "C" fn(*const Interpreter, *const c_char, c_int, *mut c_void, *mut c_void) -> c_int, // 426
    Tcl_UntraceCommand:
        extern "C" fn(*const Interpreter, *const c_char, c_int, *mut c_void, *mut c_void), // 427
    Tcl_AttemptAlloc: extern "C" fn(usize) -> *mut c_void, // 428
    Tcl_AttemptDbCkalloc: extern "C" fn(usize, *const c_char, c_int) -> *mut c_void, // 429
    Tcl_AttemptRealloc: extern "C" fn(*mut c_void, usize) -> *mut c_void, // 430
    Tcl_AttemptDbCkrealloc: extern "C" fn(*mut c_void, usize, *const c_char, c_int) -> *mut c_void, // 431
    Tcl_AttemptSetObjLength: extern "C" fn(*mut RawObject, usize) -> c_int, // 432
    Tcl_GetChannelThread: extern "C" fn(*mut c_void) -> *mut c_void,        // 433
    TclGetUnicodeFromObj: extern "C" fn(*mut RawObject, *mut c_void) -> *mut c_void, // 434
    _deprecated_435: *const c_void,                                         // 435
    _deprecated_436: *const c_void,                                         // 436
    Tcl_SubstObj: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> *mut RawObject, // 437
    Tcl_DetachChannel: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 438
    Tcl_IsStandardChannel: extern "C" fn(*mut c_void) -> c_int,             // 439
    Tcl_FSCopyFile: extern "C" fn(*mut RawObject, *mut RawObject) -> c_int, // 440
    Tcl_FSCopyDirectory: extern "C" fn(*mut RawObject, *mut RawObject, *mut c_void) -> c_int, // 441
    Tcl_FSCreateDirectory: extern "C" fn(*mut RawObject) -> c_int,          // 442
    Tcl_FSDeleteFile: extern "C" fn(*mut RawObject) -> c_int,               // 443
    Tcl_FSLoadFile: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *const c_char,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 444
    Tcl_FSMatchInDirectory: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *mut RawObject,
        *const c_char,
        *mut c_void,
    ) -> c_int, // 445
    Tcl_FSLink: extern "C" fn(*mut RawObject, *mut RawObject, c_int) -> *mut RawObject, // 446
    Tcl_FSRemoveDirectory: extern "C" fn(*mut RawObject, c_int, *mut c_void) -> c_int, // 447
    Tcl_FSRenameFile: extern "C" fn(*mut RawObject, *mut RawObject) -> c_int, // 448
    Tcl_FSLstat: extern "C" fn(*mut RawObject, *mut c_void) -> c_int,       // 449
    Tcl_FSUtime: extern "C" fn(*mut RawObject, *mut c_void) -> c_int,       // 450
    Tcl_FSFileAttrsGet:
        extern "C" fn(*const Interpreter, c_int, *mut RawObject, *mut c_void) -> c_int, // 451
    Tcl_FSFileAttrsSet:
        extern "C" fn(*const Interpreter, c_int, *mut RawObject, *mut RawObject) -> c_int, // 452
    Tcl_FSFileAttrStrings: extern "C" fn(*mut RawObject, *mut c_void) -> *const c_void, // 453
    Tcl_FSStat: extern "C" fn(*mut RawObject, *mut c_void) -> c_int,        // 454
    Tcl_FSAccess: extern "C" fn(*mut RawObject, c_int) -> c_int,            // 455
    Tcl_FSOpenFileChannel:
        extern "C" fn(*const Interpreter, *mut RawObject, *const c_char, c_int) -> *mut c_void, // 456
    Tcl_FSGetCwd: extern "C" fn(*const Interpreter) -> *mut RawObject, // 457
    Tcl_FSChdir: extern "C" fn(*mut RawObject) -> c_int,               // 458
    Tcl_FSConvertToPathType: extern "C" fn(*const Interpreter, *mut RawObject) -> c_int, // 459
    Tcl_FSJoinPath: extern "C" fn(*mut RawObject, usize) -> *mut RawObject, // 460
    TclFSSplitPath: extern "C" fn(*mut RawObject, *mut c_void) -> *mut RawObject, // 461
    Tcl_FSEqualPaths: extern "C" fn(*mut RawObject, *mut RawObject) -> c_int, // 462
    Tcl_FSGetNormalizedPath: extern "C" fn(*const Interpreter, *mut RawObject) -> *mut RawObject, // 463
    Tcl_FSJoinToPath: extern "C" fn(*mut RawObject, usize, *mut c_void) -> *mut RawObject, // 464
    Tcl_FSGetInternalRep: extern "C" fn(*mut RawObject, *const c_void) -> *mut c_void,     // 465
    Tcl_FSGetTranslatedPath: extern "C" fn(*const Interpreter, *mut RawObject) -> *mut RawObject, // 466
    Tcl_FSEvalFile: extern "C" fn(*const Interpreter, *mut RawObject) -> c_int, // 467
    Tcl_FSNewNativePath: extern "C" fn(*const c_void, *mut c_void) -> *mut RawObject, // 468
    Tcl_FSGetNativePath: extern "C" fn(*mut RawObject) -> *const c_void,        // 469
    Tcl_FSFileSystemInfo: extern "C" fn(*mut RawObject) -> *mut RawObject,      // 470
    Tcl_FSPathSeparator: extern "C" fn(*mut RawObject) -> *mut RawObject,       // 471
    Tcl_FSListVolumes: extern "C" fn() -> *mut RawObject,                       // 472
    Tcl_FSRegister: extern "C" fn(*mut c_void, *const c_void) -> c_int,         // 473
    Tcl_FSUnregister: extern "C" fn(*const c_void) -> c_int,                    // 474
    Tcl_FSData: extern "C" fn(*const c_void) -> *mut c_void,                    // 475
    Tcl_FSGetTranslatedStringPath:
        extern "C" fn(*const Interpreter, *mut RawObject) -> *const c_char, // 476
    Tcl_FSGetFileSystemForPath: extern "C" fn(*mut RawObject) -> *const c_void, // 477
    Tcl_FSGetPathType: extern "C" fn(*mut RawObject) -> *mut c_void,            // 478
    Tcl_OutputBuffered: extern "C" fn(*mut c_void) -> c_int,                    // 479
    Tcl_FSMountsChanged: extern "C" fn(*const c_void),                          // 480
    Tcl_EvalTokensStandard: extern "C" fn(*const Interpreter, *mut c_void, usize) -> c_int, // 481
    Tcl_GetTime: extern "C" fn(*mut c_void),                                    // 482
    Tcl_CreateObjTrace: extern "C" fn(
        *const Interpreter,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 483
    Tcl_GetCommandInfoFromToken: extern "C" fn(*mut c_void, *mut c_void) -> c_int, // 484
    Tcl_SetCommandInfoFromToken: extern "C" fn(*mut c_void, *const c_void) -> c_int, // 485
    Tcl_DbNewWideIntObj: extern "C" fn(*mut c_void, *const c_char, c_int) -> *mut RawObject, // 486
    Tcl_GetWideIntFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 487
    Tcl_NewWideIntObj: extern "C" fn(*mut c_void) -> *mut RawObject, // 488
    Tcl_SetWideIntObj: extern "C" fn(*mut RawObject, *mut c_void),   // 489
    Tcl_AllocStatBuf: extern "C" fn() -> *mut c_void,                // 490
    Tcl_Seek: extern "C" fn(*mut c_void, c_longlong, c_int) -> c_longlong, // 491
    Tcl_Tell: extern "C" fn(*mut c_void) -> c_longlong,              // 492
    Tcl_ChannelWideSeekProc: extern "C" fn(*const c_void) -> *mut c_void, // 493
    Tcl_DictObjPut:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject, *mut RawObject) -> c_int, // 494
    Tcl_DictObjGet:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject, *mut c_void) -> c_int, // 495
    Tcl_DictObjRemove: extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject) -> c_int, // 496
    TclDictObjSize: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 497
    Tcl_DictObjFirst: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 498
    Tcl_DictObjNext: extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void),      // 499
    Tcl_DictObjDone: extern "C" fn(*mut c_void),                                             // 500
    Tcl_DictObjPutKeyList: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        usize,
        *mut c_void,
        *mut RawObject,
    ) -> c_int, // 501
    Tcl_DictObjRemoveKeyList:
        extern "C" fn(*const Interpreter, *mut RawObject, usize, *mut c_void) -> c_int, // 502
    Tcl_NewDictObj: extern "C" fn() -> *mut RawObject,                                       // 503
    Tcl_DbNewDictObj: extern "C" fn(*const c_char, c_int) -> *mut RawObject,                 // 504
    Tcl_RegisterConfig:
        extern "C" fn(*const Interpreter, *const c_char, *const c_void, *const c_char), // 505
    Tcl_CreateNamespace:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, *mut c_void) -> *mut c_void, // 506
    Tcl_DeleteNamespace: extern "C" fn(*mut c_void), // 507
    Tcl_AppendExportList: extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 508
    Tcl_Export: extern "C" fn(*const Interpreter, *mut c_void, *const c_char, c_int) -> c_int, // 509
    Tcl_Import: extern "C" fn(*const Interpreter, *mut c_void, *const c_char, c_int) -> c_int, // 510
    Tcl_ForgetImport: extern "C" fn(*const Interpreter, *mut c_void, *const c_char) -> c_int, // 511
    Tcl_GetCurrentNamespace: extern "C" fn(*const Interpreter) -> *mut c_void,                // 512
    Tcl_GetGlobalNamespace: extern "C" fn(*const Interpreter) -> *mut c_void,                 // 513
    Tcl_FindNamespace:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, c_int) -> *mut c_void, // 514
    Tcl_FindCommand:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, c_int) -> *mut c_void, // 515
    Tcl_GetCommandFromObj: extern "C" fn(*const Interpreter, *mut RawObject) -> *mut c_void,  // 516
    Tcl_GetCommandFullName: extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject),   // 517
    Tcl_FSEvalFileEx: extern "C" fn(*const Interpreter, *mut RawObject, *const c_char) -> c_int, // 518
    _deprecated_519: *const c_void, // 519
    Tcl_LimitAddHandler:
        extern "C" fn(*const Interpreter, c_int, *mut c_void, *mut c_void, *mut c_void), // 520
    Tcl_LimitRemoveHandler: extern "C" fn(*const Interpreter, c_int, *mut c_void, *mut c_void), // 521
    Tcl_LimitReady: extern "C" fn(*const Interpreter) -> c_int, // 522
    Tcl_LimitCheck: extern "C" fn(*const Interpreter) -> c_int, // 523
    Tcl_LimitExceeded: extern "C" fn(*const Interpreter) -> c_int, // 524
    Tcl_LimitSetCommands: extern "C" fn(*const Interpreter, usize), // 525
    Tcl_LimitSetTime: extern "C" fn(*const Interpreter, *mut c_void), // 526
    Tcl_LimitSetGranularity: extern "C" fn(*const Interpreter, c_int, c_int), // 527
    Tcl_LimitTypeEnabled: extern "C" fn(*const Interpreter, c_int) -> c_int, // 528
    Tcl_LimitTypeExceeded: extern "C" fn(*const Interpreter, c_int) -> c_int, // 529
    Tcl_LimitTypeSet: extern "C" fn(*const Interpreter, c_int), // 530
    Tcl_LimitTypeReset: extern "C" fn(*const Interpreter, c_int), // 531
    Tcl_LimitGetCommands: extern "C" fn(*const Interpreter) -> c_int, // 532
    Tcl_LimitGetTime: extern "C" fn(*const Interpreter, *mut c_void), // 533
    Tcl_LimitGetGranularity: extern "C" fn(*const Interpreter, c_int) -> c_int, // 534
    Tcl_SaveInterpState: extern "C" fn(*const Interpreter, c_int) -> *mut c_void, // 535
    Tcl_RestoreInterpState: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 536
    Tcl_DiscardInterpState: extern "C" fn(*mut c_void),         // 537
    Tcl_SetReturnOptions: extern "C" fn(*const Interpreter, *mut RawObject) -> c_int, // 538
    Tcl_GetReturnOptions: extern "C" fn(*const Interpreter, c_int) -> *mut RawObject, // 539
    Tcl_IsEnsemble: extern "C" fn(*mut c_void) -> c_int,        // 540
    Tcl_CreateEnsemble:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, c_int) -> *mut c_void, // 541
    Tcl_FindEnsemble: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> *mut c_void, // 542
    Tcl_SetEnsembleSubcommandList:
        extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 543
    Tcl_SetEnsembleMappingDict:
        extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 544
    Tcl_SetEnsembleUnknownHandler:
        extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 545
    Tcl_SetEnsembleFlags: extern "C" fn(*const Interpreter, *mut c_void, c_int) -> c_int, // 546
    Tcl_GetEnsembleSubcommandList:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 547
    Tcl_GetEnsembleMappingDict:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 548
    Tcl_GetEnsembleUnknownHandler:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 549
    Tcl_GetEnsembleFlags: extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 550
    Tcl_GetEnsembleNamespace: extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 551
    Tcl_SetTimeProc: extern "C" fn(*mut c_void, *mut c_void, *mut c_void), // 552
    Tcl_QueryTimeProc: extern "C" fn(*mut c_void, *mut c_void, *mut c_void), // 553
    Tcl_ChannelThreadActionProc: extern "C" fn(*const c_void) -> *mut c_void, // 554
    Tcl_NewBignumObj: extern "C" fn(*mut c_void) -> *mut RawObject,        // 555
    Tcl_DbNewBignumObj: extern "C" fn(*mut c_void, *const c_char, c_int) -> *mut RawObject, // 556
    Tcl_SetBignumObj: extern "C" fn(*mut RawObject, *mut c_void),          // 557
    Tcl_GetBignumFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 558
    Tcl_TakeBignumFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 559
    Tcl_TruncateChannel: extern "C" fn(*mut c_void, c_longlong) -> c_int, // 560
    Tcl_ChannelTruncateProc: extern "C" fn(*const c_void) -> *mut c_void, // 561
    Tcl_SetChannelErrorInterp: extern "C" fn(*const Interpreter, *mut RawObject), // 562
    Tcl_GetChannelErrorInterp: extern "C" fn(*const Interpreter, *mut c_void), // 563
    Tcl_SetChannelError: extern "C" fn(*mut c_void, *mut RawObject),      // 564
    Tcl_GetChannelError: extern "C" fn(*mut c_void, *mut c_void),         // 565
    Tcl_InitBignumFromDouble: extern "C" fn(*const Interpreter, c_double, *mut c_void) -> c_int, // 566
    Tcl_GetNamespaceUnknownHandler:
        extern "C" fn(*const Interpreter, *mut c_void) -> *mut RawObject, // 567
    Tcl_SetNamespaceUnknownHandler:
        extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 568
    Tcl_GetEncodingFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 569
    Tcl_GetEncodingSearchPath: extern "C" fn() -> *mut RawObject, // 570
    Tcl_SetEncodingSearchPath: extern "C" fn(*mut RawObject) -> c_int, // 571
    Tcl_GetEncodingNameFromEnvironment: extern "C" fn(*mut c_void) -> *const c_char, // 572
    Tcl_PkgRequireProc:
        extern "C" fn(*const Interpreter, *const c_char, usize, *mut c_void, *mut c_void) -> c_int, // 573
    Tcl_AppendObjToErrorInfo: extern "C" fn(*const Interpreter, *mut RawObject), // 574
    Tcl_AppendLimitedToObj:
        extern "C" fn(*mut RawObject, *const c_char, usize, usize, *const c_char), // 575
    Tcl_Format:
        extern "C" fn(*const Interpreter, *const c_char, usize, *mut c_void) -> *mut RawObject, // 576
    Tcl_AppendFormatToObj: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *const c_char,
        usize,
        *mut c_void,
    ) -> c_int, // 577
    Tcl_ObjPrintf: extern "C" fn(*const c_char, *const c_char) -> *mut RawObject, // 578
    Tcl_AppendPrintfToObj: extern "C" fn(*mut RawObject, *const c_char, *const c_char), // 579
    Tcl_CancelEval: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void, c_int) -> c_int, // 580
    Tcl_Canceled: extern "C" fn(*const Interpreter, c_int) -> c_int, // 581
    Tcl_CreatePipe: extern "C" fn(*mut c_void, *mut c_void, *mut c_void, c_int) -> c_int, // 582
    Tcl_NRCreateCommand: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 583
    Tcl_NREvalObj: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> c_int, // 584
    Tcl_NREvalObjv: extern "C" fn(*const Interpreter, usize, *mut c_void, c_int) -> c_int, // 585
    Tcl_NRCmdSwap:
        extern "C" fn(*const Interpreter, *mut c_void, usize, *mut c_void, c_int) -> c_int, // 586
    Tcl_NRAddCallback: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ), // 587
    Tcl_NRCallObjProc:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void, usize, *mut c_void) -> c_int, // 588
    Tcl_GetFSDeviceFromStat: extern "C" fn(*const c_void) -> *mut c_void, // 589
    Tcl_GetFSInodeFromStat: extern "C" fn(*const c_void) -> *mut c_void,  // 590
    Tcl_GetModeFromStat: extern "C" fn(*const c_void) -> *mut c_void,     // 591
    Tcl_GetLinkCountFromStat: extern "C" fn(*const c_void) -> c_int,      // 592
    Tcl_GetUserIdFromStat: extern "C" fn(*const c_void) -> c_int,         // 593
    Tcl_GetGroupIdFromStat: extern "C" fn(*const c_void) -> c_int,        // 594
    Tcl_GetDeviceTypeFromStat: extern "C" fn(*const c_void) -> c_int,     // 595
    Tcl_GetAccessTimeFromStat: extern "C" fn(*const c_void) -> c_longlong, // 596
    Tcl_GetModificationTimeFromStat: extern "C" fn(*const c_void) -> c_longlong, // 597
    Tcl_GetChangeTimeFromStat: extern "C" fn(*const c_void) -> c_longlong, // 598
    Tcl_GetSizeFromStat: extern "C" fn(*const c_void) -> c_ulonglong,     // 599
    Tcl_GetBlocksFromStat: extern "C" fn(*const c_void) -> c_ulonglong,   // 600
    Tcl_GetBlockSizeFromStat: extern "C" fn(*const c_void) -> *mut c_void, // 601
    Tcl_SetEnsembleParameterList:
        extern "C" fn(*const Interpreter, *mut c_void, *mut RawObject) -> c_int, // 602
    Tcl_GetEnsembleParameterList:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void) -> c_int, // 603
    TclParseArgsObjv: extern "C" fn(
        *const Interpreter,
        *const c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 604
    Tcl_GetErrorLine: extern "C" fn(*const Interpreter) -> c_int,         // 605
    Tcl_SetErrorLine: extern "C" fn(*const Interpreter, c_int),           // 606
    Tcl_TransferResult: extern "C" fn(*const Interpreter, c_int, *const Interpreter), // 607
    Tcl_InterpActive: extern "C" fn(*const Interpreter) -> c_int,         // 608
    Tcl_BackgroundException: extern "C" fn(*const Interpreter, c_int),    // 609
    Tcl_ZlibDeflate:
        extern "C" fn(*const Interpreter, c_int, *mut RawObject, c_int, *mut RawObject) -> c_int, // 610
    Tcl_ZlibInflate:
        extern "C" fn(*const Interpreter, c_int, *mut RawObject, usize, *mut RawObject) -> c_int, // 611
    Tcl_ZlibCRC32: extern "C" fn(c_uint, *const c_void, usize) -> c_uint, // 612
    Tcl_ZlibAdler32: extern "C" fn(c_uint, *const c_void, usize) -> c_uint, // 613
    Tcl_ZlibStreamInit: extern "C" fn(
        *const Interpreter,
        c_int,
        c_int,
        c_int,
        *mut RawObject,
        *mut c_void,
    ) -> c_int, // 614
    Tcl_ZlibStreamGetCommandName: extern "C" fn(*mut c_void) -> *mut RawObject, // 615
    Tcl_ZlibStreamEof: extern "C" fn(*mut c_void) -> c_int,               // 616
    Tcl_ZlibStreamChecksum: extern "C" fn(*mut c_void) -> c_int,          // 617
    Tcl_ZlibStreamPut: extern "C" fn(*mut c_void, *mut RawObject, c_int) -> c_int, // 618
    Tcl_ZlibStreamGet: extern "C" fn(*mut c_void, *mut RawObject, usize) -> c_int, // 619
    Tcl_ZlibStreamClose: extern "C" fn(*mut c_void) -> c_int,             // 620
    Tcl_ZlibStreamReset: extern "C" fn(*mut c_void) -> c_int,             // 621
    Tcl_SetStartupScript: extern "C" fn(*mut RawObject, *const c_char),   // 622
    Tcl_GetStartupScript: extern "C" fn(*const c_void) -> *mut RawObject, // 623
    Tcl_CloseEx: extern "C" fn(*const Interpreter, *mut c_void, c_int) -> c_int, // 624
    Tcl_NRExprObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut RawObject) -> c_int, // 625
    Tcl_NRSubstObj: extern "C" fn(*const Interpreter, *mut RawObject, c_int) -> c_int, // 626
    Tcl_LoadFile: extern "C" fn(
        *const Interpreter,
        *mut RawObject,
        *mut c_void,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 627
    Tcl_FindSymbol: extern "C" fn(*const Interpreter, *mut c_void, *const c_char) -> *mut c_void, // 628
    Tcl_FSUnloadFile: extern "C" fn(*const Interpreter, *mut c_void) -> c_int, // 629
    Tcl_ZlibStreamSetCompressionDictionary: extern "C" fn(*mut c_void, *mut RawObject), // 630
    Tcl_OpenTcpServerEx: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_char,
        c_uint,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 631
    TclZipfs_Mount:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, *const c_char) -> c_int, // 632
    TclZipfs_Unmount: extern "C" fn(*const Interpreter, *const c_char) -> c_int, // 633
    TclZipfs_TclLibrary: extern "C" fn() -> *mut RawObject,                      // 634
    TclZipfs_MountBuffer: extern "C" fn(
        *const Interpreter,
        *const c_void,
        *mut c_void,
        *const c_char,
        c_int,
    ) -> c_int, // 635
    Tcl_FreeInternalRep: extern "C" fn(*mut RawObject),                          // 636
    Tcl_InitStringRep: extern "C" fn(*mut RawObject, *const c_char, usize) -> *mut c_void, // 637
    Tcl_FetchInternalRep: extern "C" fn(*mut RawObject, *const ObjectType) -> *mut c_void, // 638
    Tcl_StoreInternalRep: extern "C" fn(*mut RawObject, *const ObjectType, *const c_void), // 639
    Tcl_HasStringRep: extern "C" fn(*mut RawObject) -> c_int,                    // 640
    Tcl_IncrRefCount: extern "C" fn(*mut RawObject),                             // 641
    Tcl_DecrRefCount: extern "C" fn(*mut RawObject),                             // 642
    Tcl_IsShared: extern "C" fn(*mut RawObject) -> c_int,                        // 643
    Tcl_LinkArray:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, c_int, usize) -> c_int, // 644
    Tcl_GetIntForIndex:
        extern "C" fn(*const Interpreter, *mut RawObject, usize, *mut c_void) -> c_int, // 645
    Tcl_UtfToUniChar: extern "C" fn(*const c_char, *mut c_void) -> usize,        // 646
    Tcl_UniCharToUtfDString: extern "C" fn(*const c_void, usize, *mut c_void) -> *mut c_void, // 647
    Tcl_UtfToUniCharDString: extern "C" fn(*const c_char, usize, *mut c_void) -> *mut c_void, // 648
    TclGetBytesFromObj:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> *mut c_void, // 649
    Tcl_GetBytesFromObj:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> *mut c_void, // 650
    Tcl_GetStringFromObj: extern "C" fn(*mut RawObject, *mut c_void) -> *mut c_void, // 651
    Tcl_GetUnicodeFromObj: extern "C" fn(*mut RawObject, *mut c_void) -> *mut c_void, // 652
    Tcl_GetSizeIntFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 653
    Tcl_UtfCharComplete: extern "C" fn(*const c_char, usize) -> c_int, // 654
    Tcl_UtfNext: extern "C" fn(*const c_char) -> *const c_char,        // 655
    Tcl_UtfPrev: extern "C" fn(*const c_char, *const c_char) -> *const c_char, // 656
    Tcl_FSTildeExpand: extern "C" fn(*const Interpreter, *const c_char, *mut c_void) -> c_int, // 657
    Tcl_ExternalToUtfDStringEx: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *const c_char,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 658
    Tcl_UtfToExternalDStringEx: extern "C" fn(
        *const Interpreter,
        *mut c_void,
        *const c_char,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 659
    Tcl_AsyncMarkFromSignal: extern "C" fn(*mut c_void, c_int) -> c_int, // 660
    Tcl_ListObjGetElements:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void, *mut c_void) -> c_int, // 661
    Tcl_ListObjLength: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 662
    Tcl_DictObjSize: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 663
    Tcl_SplitList:
        extern "C" fn(*const Interpreter, *const c_char, *mut c_void, *const c_void) -> c_int, // 664
    Tcl_SplitPath: extern "C" fn(*const c_char, *mut c_void, *const c_void), // 665
    Tcl_FSSplitPath: extern "C" fn(*mut RawObject, *mut c_void) -> *mut RawObject, // 666
    Tcl_ParseArgsObjv: extern "C" fn(
        *const Interpreter,
        *const c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> c_int, // 667
    Tcl_UniCharLen: extern "C" fn(*const c_void) -> usize,                   // 668
    Tcl_NumUtfChars: extern "C" fn(*const c_char, usize) -> usize,           // 669
    Tcl_GetCharLength: extern "C" fn(*mut RawObject) -> usize,               // 670
    Tcl_UtfAtIndex: extern "C" fn(*const c_char, usize) -> *const c_char,    // 671
    Tcl_GetRange: extern "C" fn(*mut RawObject, usize, usize) -> *mut RawObject, // 672
    Tcl_GetUniChar: extern "C" fn(*mut RawObject, usize) -> c_int,           // 673
    Tcl_GetBool: extern "C" fn(*const Interpreter, *const c_char, c_int, *mut c_void) -> c_int, // 674
    Tcl_GetBoolFromObj:
        extern "C" fn(*const Interpreter, *mut RawObject, c_int, *mut c_void) -> c_int, // 675
    Tcl_CreateObjCommand2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 676
    Tcl_CreateObjTrace2: extern "C" fn(
        *const Interpreter,
        usize,
        c_int,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 677
    Tcl_NRCreateCommand2: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *mut c_void,
    ) -> *mut c_void, // 678
    Tcl_NRCallObjProc2:
        extern "C" fn(*const Interpreter, *mut c_void, *mut c_void, usize, *mut c_void) -> c_int, // 679
    Tcl_GetNumberFromObj:
        extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void, *mut c_void) -> c_int, // 680
    Tcl_GetNumber:
        extern "C" fn(*const Interpreter, *const c_char, usize, *mut c_void, *mut c_void) -> c_int, // 681
    Tcl_RemoveChannelMode: extern "C" fn(*const Interpreter, *mut c_void, c_int) -> c_int, // 682
    Tcl_GetEncodingNulLength: extern "C" fn(*mut c_void) -> usize,                         // 683
    Tcl_GetWideUIntFromObj: extern "C" fn(*const Interpreter, *mut RawObject, *mut c_void) -> c_int, // 684
    Tcl_DStringToObj: extern "C" fn(*mut c_void) -> *mut RawObject, // 685
    Tcl_UtfNcmp: extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, // 686
    Tcl_UtfNcasecmp: extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, // 687
    Tcl_NewWideUIntObj: extern "C" fn(*mut c_void) -> *mut RawObject, // 688
    Tcl_SetWideUIntObj: extern "C" fn(*mut RawObject, *mut c_void), // 689
}

/// Error codes for unwrapping a Tcl interpreter.
///
/// These exist primarily for debugging and advanced use-cases.  Unless you
/// are calling [from_raw](Interpreter::from_raw), you should not need to worry about these.
#[derive(Debug)]
pub enum Error {
    NullInterpreter,
    NullStubs,
    InvalidStubs,
    TclError(String),
}

impl<'a> Interpreter {
    /// Converts a raw pointer to a Tcl interpreter into a Rust reference.
    ///
    /// Most users should not need to use this function because a reference is
    /// already passed to the appropriate functions.  This is public because
    /// of how the [module_init](rtea_proc::module_init) macro works.
    pub fn from_raw(interpreter: *const Interpreter) -> Result<&'a Interpreter, Error> {
        if let Some(interpreter) = unsafe { interpreter.as_ref() } {
            if let Some(stubs) = unsafe { interpreter.stubs.as_ref() } {
                if stubs.magic == TCL_STUB_MAGIC {
                    Ok(interpreter)
                } else {
                    Err(Error::InvalidStubs)
                }
            } else {
                Err(Error::NullStubs)
            }
        } else {
            Err(Error::NullInterpreter)
        }
    }

    /// DO NOT USE THIS FUNCTION.
    ///
    /// This function is only public so that it can be called by the
    /// [module_init](rtea_proc::module_init) macro for derivative RTEA
    /// projects..
    pub fn init_global_functions(&self) {
        unsafe {
            let stubs = self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check");

            ALLOC = Some(stubs.Tcl_AttemptAlloc);
            REALLOC = Some(stubs.Tcl_AttemptRealloc);
            FREE = Some(stubs.Tcl_Free);

            NEW_OBJ = Some(stubs.Tcl_NewObj);
            DUPLICATE_OBJ = Some(stubs.Tcl_DuplicateObj);
            INCR_REF_COUNT = Some(stubs.Tcl_IncrRefCount);
            DECR_REF_COUNT = Some(stubs.Tcl_DecrRefCount);
            IS_SHARED = Some(stubs.Tcl_IsShared);
            INVALIDATE_STRING_REP = Some(stubs.Tcl_InvalidateStringRep);
            GET_STRING = Some(stubs.Tcl_GetString);

            GET_OBJ_TYPE = Some(stubs.Tcl_GetObjType);
            CONVERT_TO_TYPE = Some(stubs.Tcl_ConvertToType);

            NEW_STRING_OBJ = Some(stubs.Tcl_NewStringObj);
            SET_STRING_OBJ = Some(stubs.Tcl_SetStringObj);
        }
    }

    /// Informs the Tcl interpreter that the given package and version is available.
    pub fn provide_package(&self, name: &str, version: &str) -> Result<TclStatus, String> {
        let name =
            CString::new(name).map_err(|_| "unexpected Nul in package version".to_string())?;
        let version =
            CString::new(version).map_err(|_| "unexpected Nul in package version".to_string())?;
        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_PkgProvideEx)(
                self as *const Interpreter,
                name.as_ptr(),
                version.as_ptr(),
                std::ptr::null(),
            )
        };
        Ok(TclStatus::Ok)
    }

    /// Registers the command given by `proc` as `name`.
    pub fn create_command(&self, name: &str, proc: CmdProc) -> Result<TclStatus, String> {
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        // type TclCmdProc = extern "C" fn(
        //     data: *const c_void,
        //     interp: *const Interpreter,
        //     argc: usize,
        //     argv: *const *const i8,
        // ) -> TclStatus;
        extern "C" fn wrapper_proc(
            f_ptr: *const c_void,
            i: *const Interpreter,
            argc: usize,
            argv: *const *const i8,
        ) -> TclStatus {
            let interp = Interpreter::from_raw(i).expect("Tcl passed bad interpreter");
            let raw_args = unsafe { std::slice::from_raw_parts(argv, argc) };
            let mut args = Vec::with_capacity(raw_args.len());
            for arg in raw_args {
                args.push(
                    unsafe { std::ffi::CStr::from_ptr(*arg) }
                        .to_str()
                        .expect("invalid args from Tcl"),
                );
            }

            let f: CmdProc = unsafe { std::mem::transmute(f_ptr) };

            f(interp, args).unwrap_or_else(|s| {
                interp.set_result(&s);
                TclStatus::Error
            })
        }

        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_CreateCommand)(
                self as *const Interpreter,
                name.as_ptr(),
                wrapper_proc as *mut c_void,
                proc as *mut c_void,
                std::ptr::null_mut::<c_void>(),
            )
        };

        Ok(TclStatus::Ok)
    }

    /// Registers the command given by `proc` as `name`.
    pub fn create_obj_command(&self, name: &str, proc: ObjCmdProc) -> Result<TclStatus, String> {
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        // type TclCmdProc = extern "C" fn(
        //     data: *const c_void,
        //     interp: *const Interpreter,
        //     argc: usize,
        //     argv: *const *mut RawObject,
        // ) -> TclStatus;
        extern "C" fn wrapper_proc(
            f_ptr: *const c_void,
            i: *const Interpreter,
            argc: usize,
            argv: *const *mut RawObject,
        ) -> TclStatus {
            let interp = Interpreter::from_raw(i).expect("Tcl passed bad interpreter");
            let raw_args = unsafe { std::slice::from_raw_parts(argv, argc) };
            let mut args = Vec::with_capacity(raw_args.len());
            for arg in raw_args {
                args.push(RawObject::wrap(*arg));
            }

            let f: ObjCmdProc = unsafe { std::mem::transmute(f_ptr) };

            f(interp, args).unwrap_or_else(|obj| {
                interp.set_obj_result(&obj);
                TclStatus::Error
            })
        }

        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_CreateObjCommand)(
                self as *const Interpreter,
                name.as_ptr(),
                wrapper_proc as *mut c_void,
                proc as *mut c_void,
                std::ptr::null_mut::<c_void>(),
            )
        };

        Ok(TclStatus::Ok)
    }

    /// Registers the object with TCL
    pub fn register_obj_type<T: TclObjectType>(&self) {
        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_RegisterObjType)(T::tcl_type() as *const ObjectType)
        }
    }

    /// Deletes the given command.
    ///
    /// This function attempts to delete the command `name` in the
    /// interpreter.  If it exists, `true` is returned, otherwise `false` is
    /// returned.  An error is only returned when the given `name` contains
    /// Nul characters and is therefore not a valid Tcl string.
    pub fn delete_command(&self, name: &str) -> Result<bool, String> {
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        let ret = unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_DeleteCommand)(self as *const Interpreter, name.as_ptr())
        };

        Ok(ret == 0)
    }

    /// Get the current result object for the interpreter.
    pub fn get_obj_result(&self) -> Object {
        unsafe {
            RawObject::wrap((self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_GetObjResult)(
                self as *const Interpreter
            ))
        }
    }

    /// Evaluate a Tcl script.
    ///
    /// Evaluates the given string as a Tcl script.  If the script return
    /// `TclStatus::Error`, then the associated error message is passed back
    /// as `Err`.  Otherwise the last commands return value is passed through
    /// as is.
    pub fn eval(&self, script: &str) -> Result<Object, Object> {
        if script.len() > 1 << 31 {
            return Err(
                Object::new(), // "Tcl versions prior to 9.0 do not support scripts greater than 2 GiB".to_string(),
            );
        }
        let status = unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_EvalEx)(
                self as *const Interpreter,
                script.as_ptr() as *const c_char,
                script.len(),
                0,
            )
        };
        let result = self.get_obj_result();
        if TclStatus::Error == status.into() {
            Err(result)
        } else {
            Ok(result)
        }
    }

    /// Set the interpreter's current result value.
    ///
    /// When inside command logic, this can be used to set the return value
    /// visible to the invoking Tcl script.
    pub fn set_result(&self, text: &str) {
        let tcl_str = self
            .alloc(text.len() + 1)
            .expect("propagating memory failure in Tcl");

        tcl_str[..text.len()].copy_from_slice(text.as_bytes());

        if let Some(terminator) = tcl_str.last_mut() {
            *terminator = 0;
        }

        let result = Object::new_string(text);
        self.set_obj_result(&result);
    }

    pub fn set_obj_result(&self, result: &Object) {
        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_SetObjResult)(
                self as *const Interpreter, result.obj as *mut RawObject
            )
        }
    }

    /// Allocates Tcl-managed memory.
    ///
    /// Allocates memory that is directly managed by Tcl.  This is required
    /// for certain interfaces (e.g., bytes representation of Tcl objects)
    /// and convenient for others (e.g., creating a Nul terminated string
    /// from a Rust `String`).
    pub fn alloc(&self, size: usize) -> Option<&mut [u8]> {
        if size >= 1 << 32 {
            return None;
        }
        let ptr = unsafe {
            // Trusting Tcl to handle this correctly (check above can be removed for Tcl 9.0)
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_AttemptAlloc)(size)
        };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                // We've checked that it is not null and therefore trust Tcl
                Some(std::slice::from_raw_parts_mut(ptr as *mut u8, size))
            }
        }
    }
}

type CmdDataProc<T> =
    fn(interp: &Interpreter, data: &T, args: Vec<&str>) -> Result<TclStatus, String>;

/// A wrapper for creating stateful commands.
///
/// The `StatefulCommand` type enables the creation of Tcl commands that are
/// stateful.  In other words, they can be modified and carry-forward their
/// state to future invocations (in contrast to the `create_command` method
/// of [Interpreter] where the function must either be pure (from Tcl's
/// perspective) or use global state).
///
/// # Example
///
/// ```rust
/// use std::cell::RefCell;
///
/// use rtea::*;
///
/// fn create_stateful_command(interp: &Interpreter) {
///     fn cmd(
///         interp: &Interpreter,
///         counter: &RefCell<usize>,
///         _args: Vec<&str>,
///     ) -> Result<TclStatus, String> {
///         let mut val = counter.borrow_mut();
///         interp.set_result(&val.to_string());
///         *val += 1;
///
///         Ok(TclStatus::Ok)
///     }
///
///     let c = StatefulCommand::new(cmd, RefCell::<usize>::new(0));
///     c.attach_command(interp, "counter").unwrap();
///     
///     for i in 0..10 {
///         interp.eval("counter").unwrap();
///         assert_eq!(i.to_string(), interp.get_obj_result().get_string());
///     }
/// }
/// ```
pub struct StatefulCommand<T> {
    proc: CmdDataProc<T>,
    data: T,
}

impl<T> StatefulCommand<T> {
    /// Creates a new `StatefulCommand`.
    ///
    /// The creates a new `StatefulComand` with ownership of `data`.  The
    /// underlying implementation should be as thread-safe as the original
    /// implementation of `data`, but care needs to be taken to ensure that
    /// any concurrency from Tcl is safe on `data` (there are no concerns for
    /// `proc`).
    pub fn new(proc: CmdDataProc<T>, data: T) -> StatefulCommand<T> {
        StatefulCommand::<T> {
            proc: proc,
            data: data,
        }
    }

    /// Attaches the `StatefulCommand` to a Tcl interpreter.
    ///
    /// This exposes the instantiated `StatefulCommand` to the given
    /// interpreter.  This should allow exposing a command to multiple
    /// interpreters (or as aliases in the same interpreter) for advanced
    /// functionality. While the borrow checker should prevent some misuses
    /// (type is passed by ownership), this has not been heavily tested for
    /// every type `T`.
    pub fn attach_command(self, interp: &Interpreter, name: &str) -> Result<TclStatus, String> {
        let state = Box::new(self);
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        // Simple wrapper of the Rust function and data to work with Tcl's API.
        extern "C" fn wrapper_proc<T>(
            state: *const StatefulCommand<T>,
            i: *const Interpreter,
            argc: usize,
            argv: *const *const i8,
        ) -> TclStatus {
            let interp = Interpreter::from_raw(i).expect("Tcl passed bad interpreter");
            let raw_args = unsafe { std::slice::from_raw_parts(argv, argc) };
            let mut args = Vec::with_capacity(raw_args.len());
            for arg in raw_args {
                args.push(
                    unsafe { std::ffi::CStr::from_ptr(*arg) }
                        .to_str()
                        .expect("invalid args from Tcl"),
                );
            }

            let state = unsafe { state.as_ref() }.expect("data command corrupted!");

            (state.proc)(interp, &state.data, args).unwrap_or_else(|s| {
                interp.set_result(&s);
                TclStatus::Error
            })
        }

        // Simple function to restore the `StatefulCommand` to Rust's
        // understanding to allow Rust's RAII code to kick in.
        fn free_state<T>(state: *mut StatefulCommand<T>) {
            // This relies on Tcl to properly track the command state and
            // invoke this at the appropriate moment.  Retaking ownership
            // of the underlying pointer ensures the destructor gets called
            unsafe {
                let _ = Box::from_raw(state);
            };
        }

        unsafe {
            (interp
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .Tcl_CreateCommand)(
                interp as *const Interpreter,
                name.as_ptr(),
                wrapper_proc::<T> as *mut c_void,
                Box::<StatefulCommand<T>>::into_raw(state) as *mut c_void,
                free_state::<T> as *mut c_void,
            )
        };

        Ok(TclStatus::Ok)
    }
}
