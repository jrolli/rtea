# rtea

rtea (Rust Tcl Extension Architecture) is a library for writing Tcl extensions
in Rust. It wraps the Tcl stubs interface so that extensions can be written
without `unsafe` code or direct knowledge of Tcl's C API.

## Setup

Your crate must be a `cdylib`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
rtea = "0.3"
```

## Writing an Extension

### Initialization

Use the `#[module_init]` macro to define the entry point Tcl calls when loading
your extension. The macro generates the correctly named and typed `extern "C"`
symbol, validates the interpreter, and registers the package version.

```rust
use rtea::*;

#[module_init(Example, "1.0.0")]
fn init(interp: &Interpreter) -> Result<TclStatus, String> {
    interp.create_command("hello", hello_cmd)
}

fn hello_cmd(interp: &Interpreter, _args: Vec<&str>) -> Result<TclStatus, String> {
    interp.set_result("Hello from Rust!");
    Ok(TclStatus::Ok)
}
```

The module name passed to the macro must match Tcl's naming conventions: first
letter capitalised, all others lowercase (e.g. `Example` for a library named
`libexample`). The generated symbol is `Example_Init`.

If the extension should also be loadable into a safe interpreter, provide a
second entry point with `#[module_safe_init]`, which generates `Example_SafeInit`.

### Unloading

```rust
#[module_unload(Example)]
fn unload(_interp: &Interpreter, _flags: TclUnloadFlag) -> Result<TclStatus, String> {
    Ok(TclStatus::Ok)
}
```

`#[module_safe_unload]` generates the corresponding `Example_SafeUnload` symbol.

## Commands

### Stateless commands

Commands that do not need to carry state take the form:

```rust
fn my_cmd(interp: &Interpreter, args: Vec<&str>) -> Result<TclStatus, String> {
    // args[0] is the command name, args[1..] are the arguments.
    interp.set_result("ok");
    Ok(TclStatus::Ok)
}

interp.create_command("mycmd", my_cmd)?;
```

Return `Err(message)` to signal a Tcl error; the message becomes the
interpreter's error result automatically.

### Stateful commands

`StatefulCommand` associates owned data with a command. The data is dropped
automatically when the command is deleted from the interpreter.

```rust
use std::cell::RefCell;

fn counter_cmd(
    interp: &Interpreter,
    count: &RefCell<usize>,
    _args: Vec<&str>,
) -> Result<TclStatus, String> {
    let mut n = count.borrow_mut();
    interp.set_result(&n.to_string());
    *n += 1;
    Ok(TclStatus::Ok)
}

StatefulCommand::new(counter_cmd, RefCell::new(0usize))
    .attach_command(interp, "counter")?;
```

### Object commands

Commands that work with typed Tcl objects use `create_obj_command`:

```rust
fn add_cmd(interp: &Interpreter, args: Vec<Object>) -> Result<TclStatus, Object> {
    let a: i64 = args[1].get_string().parse().map_err(|_| Object::new_string("bad int"))?;
    let b: i64 = args[2].get_string().parse().map_err(|_| Object::new_string("bad int"))?;
    interp.set_result(&(a + b).to_string());
    Ok(TclStatus::Ok)
}

interp.create_obj_command("add", add_cmd)?;
```

## Custom Object Types

The `TclObjectType` derive macro registers a Rust struct as a native Tcl object
type, allowing values to be passed between Tcl and Rust without repeated string
parsing.

```rust
use rtea::*;

#[derive(Clone, Debug, TclObjectType)]
struct Point { x: f64, y: f64 }

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
```

After calling `interp.register_obj_type::<Point>()`, the type is available to
Tcl. Use `Point::from_object(&obj)` to borrow the inner value and
`point.into_object()` (or the `From` impl) to wrap one in an `Object`.

## Evaluating Tcl

```rust
let result = interp.eval("expr {1 + 1}")?;
println!("{}", result.get_string()); // "2"
```

`eval` returns `Ok(Object)` on success and `Err(Object)` on error, where the
object holds the interpreter's result or error message respectively.

## Maintenance

`stubParser.tcl` regenerates the Tcl stubs vtable in `src/interpreter.rs` from
a Tcl source tree. Run it whenever updating the Tcl version:

```
tclsh stubParser.tcl /path/to/tcl/generic/tcl.decls
```
