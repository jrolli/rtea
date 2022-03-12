//! rtea-proc provides macros to ergonomically wrap the initialization and
//! unload functions expected by TEA.
//!
//! The library provides the simple macros to conveniently wrap Rust
//! initialization and unload functions without having to deal with
//! `extern "C"` or raw pointers.  
//!
//! # Example
//!
//! ```rust
//! use rtea::{Interpreter, TclStatus, TclUnloadFlag}; // Implicit dependency of macro when invoked.
//!
//! #[module_init(Example, "1.0.0")]
//! fn init(interp: &Interpreter) -> Result<TclStatus, String> {
//!     safe_init(interp, args)?;
//!     // Add additional commands that may not be safe for untrusted code...
//!     Ok(TclStatus::Ok)
//! }
//!
//! #[module_safe_init(Example, "1.0.0")]
//! fn safe_init(_interp: &Interpreter) -> Result<TclStatus, String> {
//!     // Add commands that are safe even for untrusted code...
//!     Ok(TclStatus::Ok)
//! }
//!
//! #[module_unload(Example)]
//! fn unload(interp: &Interpreter) -> Result<TclStatus, String> {
//!     safe_unload(interp, args)?;
//!     // Remove the additional commands that were not considered "safe"...
//!     Ok(TclStatus::Ok)
//! }
//!
//! #[module_safe_unload(Example)]
//! fn safe_unload(_interp: &Interpreter) -> Result<TclStatus, String> {
//!     // Remove the "safe" set of commands
//!     Ok(TclStatus::Ok)
//! }
//! ```
//!
//! # Note
//!
//! This code assumes that it extends Tcl and treats any violations of Tcl's
//! API (unexpected null-pointers, non-UTF8 strings, etc.) as irrecovable
//! errors that should panic.

use proc_macro::TokenStream;
use proc_macro::TokenTree::Punct;
use std::str::FromStr;

fn module_init_common(prefix: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut mod_name = None;
    let mut version = None;
    for a in attr {
        if let Punct(_) = a {
            continue;
        }
        if mod_name == None {
            mod_name = Some(a.to_string());
        } else if version == None {
            version = Some(a.to_string());
        } else {
            panic!("Unexpected additional attributes to 'module_init': {}", a)
        }
    }
    let mod_name = mod_name.expect("no module name found");
    let version = version.unwrap_or("".to_string());

    let mut out_stream = TokenStream::new();

    let mut next_item = false;
    let mut fn_name = None;
    for i in item {
        if next_item {
            fn_name = Some(i.to_string());
            next_item = false;
        } else if fn_name.is_none() && i.to_string() == "fn" {
            next_item = true;
        }
        out_stream.extend([i]);
    }
    let fn_name = fn_name.expect("'module_init' macro not used on a function");

    out_stream.extend(
        TokenStream::from_str(&format!(
            r#"
                #[no_mangle]
                pub extern "C" fn {module_symbol}_{prefix}Init(interp: *const Interpreter) -> TclStatus {{
                    Interpreter::from_raw(interp)
                        .map(|interp| {{
                            interp.init_global_functions();
                            {init_fn}(interp)
                                .and(interp.provide_package("{module_tcl}", {version}))
                                .unwrap_or_else(|s| {{interp.set_result(&s); TclStatus::Error}})
                        }})
                        .unwrap_or(TclStatus::Error)
                }}
            "#,
            prefix = prefix,
            module_symbol = mod_name,
            init_fn = fn_name,
            module_tcl = mod_name.to_lowercase(),
            version = version
        ))
        .unwrap(),
    );

    out_stream
}

/// Helper for creating the initialization function for Tcl extensions.
///
/// This macro will automatically create the appropriate wrapper to validate
/// the interpreter and "provide" the package to the interpreter.  The
/// prototype of the wrapped function should be
///
/// ```rust
/// type init_fn = fn(interp: &rtea::Interpreter) -> Result<rtea::TclStatus, String>;
/// ```
///
/// and one or two attributes should be passed to the macro.  The first must
/// be the module's name with a capital first letter and all others lowercase
/// (this is a Tcl requirement).  The second, optional attribute, is the
/// version which by Tcl convention should be in accordance with semver.
///
/// # Example
///
/// ```rust
/// #[module_init(Example, "1.0.0")]
/// fn init(interp: &Interpreter) -> Result<TclStatus, String> {
///     interp.eval("Initializing module...")
/// }
/// ```
///
/// The above example will create a function named `Example_Init` (with the
/// `no_mangle` attribute) which Tcl will use as the initialization routine.
/// This assumes that your files final library name matches the expectation
/// of `-lexample` for the C linker (which is the case if used in a "cdylib"
/// crate named "example").
#[proc_macro_attribute]
pub fn module_init(attr: TokenStream, item: TokenStream) -> TokenStream {
    module_init_common("", attr, item)
}

/// Helper for creating the "safe" initialization function for Tcl extensions.
///
/// This macro will automatically create the appropriate wrapper to validate
/// the interpreter and "provide" the package to the interpreter.  The
/// prototype of the wrapped function should be
///
/// ```rust
/// type init_fn = fn(interp: &rtea::Interpreter) -> Result<rtea::TclStatus, String>;
/// ```
///
/// and one or two attributes should be passed to the macro.  The first must
/// be the module's name with a capital first letter and all others lowercase
/// (this is a Tcl requirement).  The second, optional attribute, is the
/// version which by Tcl convention should be in accordance with semver.
///
/// # Example
///
/// ```rust
/// #[module_safe_init(Example, "1.0.0")]
/// fn init(interp: &Interpreter) -> Result<TclStatus, String> {
///     interp.eval("Initializing module...")
/// }
/// ```
///
/// The above example will create a function named `Example_SafeInit` (with the
/// `no_mangle` attribute) which Tcl will use as the initialization routine.
/// This assumes that your files final library name matches the expectation
/// of `-lexample` for the C linker (which is the case if used in a "cdylib"
/// crate named "example").
///
/// # Warning
///
/// This initialization routine is intended to be safe to use
/// from **untrusted** code.  Users must take care that the functionality
/// they expose to Tcl scripts from here is truly "safe" (in the destroy a
/// system sense, not Rust's crash a program sense).  It is highly
/// recommended you read about [Safe Tcl](https://www.tcl.tk/man/tcl/TclCmd/safe.html)
/// before using this macro.
#[proc_macro_attribute]
pub fn module_safe_init(attr: TokenStream, item: TokenStream) -> TokenStream {
    module_init_common("Safe", attr, item)
}

fn module_unload_common(prefix: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut mod_name = None;
    for a in attr {
        if mod_name == None {
            mod_name = Some(a.to_string());
        } else {
            panic!("Unexpected additional attributes to 'module_init': {}", a)
        }
    }
    let mod_name = mod_name.expect("no module name found");

    let mut out_stream = TokenStream::new();

    let mut next_item = false;
    let mut fn_name = None;
    for i in item {
        if next_item {
            fn_name = Some(i.to_string());
            next_item = false;
        } else if fn_name.is_none() && i.to_string() == "fn" {
            next_item = true;
        }
        out_stream.extend([i]);
    }
    let fn_name = fn_name.expect("'module_unload' macro not used on a function");

    out_stream.extend(
        TokenStream::from_str(&format!(
            r#"
                #[no_mangle]
                pub extern "C" fn {module_symbol}_{prefix}Unload(interp: *const Interpreter, flags: TclUnloadFlag) -> TclStatus {{
                    Interpreter::from_raw(interp)
                        .map(|interp| {unload_fn}(interp, flags)
                            .unwrap_or_else(|s| {{interp.set_result(&s); TclStatus::Error}}))
                        .unwrap_or(TclStatus::Error)
                }}
            "#,
            prefix = prefix,
            module_symbol = mod_name,
            unload_fn = fn_name,
        ))
        .unwrap(),
    );

    out_stream
}

/// Helper for unloading a Tcl extension.
///
/// This macro will automatically create the appropriate wrapper to validate
/// the interpreter and pass it to the given unload routine.  The prototype
/// of the wrapped function should be
///
/// ```rust
/// type unload_fn = fn(interp: &rtea::Interpreter, flags: TclUnloadFlag) -> Result<rtea::TclStatus, String>;
/// ```
///
/// and the module's name (as given to [module_init]) should be given as the
/// sole attribute to the macro.
#[proc_macro_attribute]
pub fn module_unload(attr: TokenStream, item: TokenStream) -> TokenStream {
    module_unload_common("", attr, item)
}

/// Helper for unloading a "safe" Tcl extensions
///
/// This macro will automatically create the appropriate wrapper to validate
/// the interpreter and pass it to the given unload routine.  The prototype
/// of the wrapped function should be
///
/// ```rust
/// type unload_fn = fn(interp: &rtea::Interpreter, flags: TclUnloadFlag) -> Result<rtea::TclStatus, String>;
/// ```
///
/// and the module's name (as given to [module_init]) should be given as the
/// sole attribute to the macro.
#[proc_macro_attribute]
pub fn module_safe_unload(attr: TokenStream, item: TokenStream) -> TokenStream {
    module_unload_common("Safe", attr, item)
}

fn get_struct_name(item: TokenStream) -> String {
    let mut next_item = false;
    for i in item {
        if next_item {
            return i.to_string();
        } else if i.to_string() == "struct" {
            next_item = true;
        }
    }
    panic!("Not a struct")
}

#[proc_macro_derive(TclObjectType)]
pub fn generate_tcl_object(item: TokenStream) -> TokenStream {
    let obj_name = get_struct_name(item);
    let tcl_obj_name = format!("{}_TCL_OBJECT", obj_name.to_uppercase());
    let mut out_stream = TokenStream::new();

    out_stream.extend(
        TokenStream::from_str(&format!(
            r#"
                extern "C" fn {obj_name}_tcl_free(obj: *mut RawObject) {{
                    unsafe {{
                        Box::from_raw((*obj).ptr1 as *mut {obj_name});
                    }}
                }}

                extern "C" fn {obj_name}_tcl_dup(obj: *const RawObject, new_obj: *mut RawObject) {{
                    unsafe {{
                        let new_rep = Box::into_raw(Box::new(((*obj).ptr1 as *mut {obj_name}).as_ref().unwrap().clone())) as *mut std::ffi::c_void;
                        (*new_obj).ptr1 = new_rep;
                        (*new_obj).obj_type = (&{tcl_obj_name}) as *const ObjectType;
                    }}
                }}

                extern "C" fn {obj_name}_tcl_update(obj: *mut RawObject) {{
                    unsafe {{
                        let inner = ((*obj).ptr1 as *mut {obj_name}).as_ref().unwrap();
                        let (tcl_str, tcl_str_len) = rtea::tcl_string(&inner.as_string());
                        (*obj).bytes = tcl_str;
                        (*obj).length = tcl_str_len as i32;
                    }}
                }}

                extern "C" fn {obj_name}_tcl_from(interp: *const Interpreter, obj: *mut RawObject) -> TclStatus {{
                    let interp = unsafe {{ interp.as_ref() }};
                    let obj = RawObject::wrap(obj);

                    let (res, _obj) = match {obj_name}::convert(obj) {{
                        Ok(obj) => {{
                            (TclStatus::Ok, obj)
                        }}
                        Err(obj) => {{
                            (TclStatus::Error, obj)
                        }}
                    }};

                    if res == TclStatus::Error && interp.is_some() {{
                        let interp = interp.unwrap();
                        interp.set_result("could not convert to '{obj_name}' type")
                    }}

                    res
                }}

                static {tcl_obj_name}: ObjectType = ObjectType {{
                    name: "{obj_name}\0".as_ptr(),
                    free_internal_rep_proc: Some({obj_name}_tcl_free),
                    dup_internal_rep_proc: {obj_name}_tcl_dup,
                    update_string_proc: Some({obj_name}_tcl_update),
                    set_from_any_proc: Some({obj_name}_tcl_from),
                }};

                impl rtea::TclObjectType for {obj_name} {{
                    fn from_object(obj: &rtea::Object) -> Option<&{obj_name}> {{
                        let obj_type_ptr = (&{tcl_obj_name}) as *const ObjectType;
                        unsafe {{
                            if (*obj.obj).obj_type != obj_type_ptr {{
                                {obj_name}_tcl_from(std::ptr::null(), obj.obj);
                            }}

                            if (*obj.obj).obj_type == obj_type_ptr {{
                                Some(((*obj.obj).ptr1 as *const {obj_name}).as_ref().unwrap())
                            }} else {{
                                None
                            }}
                        }}
                    }}

                    fn into_object(self) ->rtea::Object {{
                        let ptr = Box::into_raw(Box::new(self)) as *mut std::ffi::c_void;
                        let obj = rtea::Object::new();
                        unsafe {{
                            (*obj.obj).ptr1 = ptr;
                            (*obj.obj).obj_type = (&{tcl_obj_name}) as *const rtea::ObjectType;
                            (*obj.obj).bytes = std::ptr::null_mut();
                        }}
                        obj
                    }}

                    fn type_name() -> &'static str {{ "{obj_name}" }}

                    fn tcl_type() -> &'static ObjectType {{ &{tcl_obj_name} }}
                }}

                impl From<{obj_name}> for rtea::Object {{
                    fn from(pt: {obj_name}) -> rtea::Object {{
                        pt.into_object()
                    }}
                }}
            "#,
            obj_name = obj_name,
            tcl_obj_name = tcl_obj_name,
        ))
        .unwrap(),
    );

    out_stream
}
