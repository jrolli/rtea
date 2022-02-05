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
