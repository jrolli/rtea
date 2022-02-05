//! rtea tries to be the Rust [TEA](https://www.tcl.tk/doc/tea/TEAOverview.html)
//! ([Tcl](https://www.tcl.tk) Extension Architecture).
//!
//! The library provides the necessary wrappers around the Tcl stubs
//! implentation to write a "rusty" extension to Tcl without having to use
//! unsafe code.  
//!
//! # Example
//!
//! The following example assumes that you are in the root directory of a project
//! with a `cdylib` target and the two files given below.  If you execute:
//!
//! ```bash
//! cargo build
//! tclsh example.tcl
//! ```
//!
//! Then you should see the following output:
//!
//! ```text
//! Hello, world!
//! Hello from Rust!
//! ```
//!
//! **src/lib.rs**
//! ```rust
//! use rtea::*;
//!
//! #[module_init(Example, "1.0.0")]
//! fn init(interp: &Interpreter) -> Result<TclStatus, String> {
//!     interp.create_command("example", example)
//! }
//!
//! fn example(interp: &Interpreter, args: Vec<&str>) -> Result<TclStatus, String> {
//!     interp.eval("puts {Hello, world!}").map_err(|e| e.get_string().to_string())?;
//!     interp.set_result("Hello from Rust!");
//!     Ok(TclStatus::Ok)
//! }
//! ```
//!
//! **example.tcl**
//! ```tcl
//! load ./target/debug/libexample.so
//! puts [example]
//! ```
//!
//! The Tcl code above uses `load` just to show that the module can properly
//! interact with Tcl.  Production uses should wrap the shared object as a
//! Tcl package and load it using `package require example`.
//! The `module_init` macro already handles registering the "example"
//! package.
//!
//! # Note
//!
//! This code assumes that it extends Tcl and treats any violations of Tcl's
//! API (unexpected null-pointers, non-UTF8 strings, etc.) as irrecovable
//! errors that should panic.

mod interpreter;
mod object;
mod tcl;

pub use interpreter::*;
pub use object::*;
pub use rtea_proc::module_init;
