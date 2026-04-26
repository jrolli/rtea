# rtea-proc

Procedural macros for writing Tcl extensions with [rtea](../README.md). These
macros generate the `extern "C"` entry points that Tcl expects when loading or
unloading an extension, handling interpreter validation and pointer casting so
you do not have to.

This crate is re-exported by `rtea` and is not normally added as a direct
dependency.

## Macros

### `#[module_init(Name, "version")]`

Generates `Name_Init`, the entry point Tcl calls when the extension is loaded
with `load` or `package require`.

The macro wraps your function, validates the interpreter pointer, initialises
the global stubs table, calls your function, and then registers the package
version with Tcl. If your function returns `Err`, the message is set as the
interpreter's error result and `TclStatus::Error` is returned to Tcl.

```rust
#[module_init(Example, "1.0.0")]
fn init(interp: &Interpreter) -> Result<TclStatus, String> {
    interp.create_command("hello", hello_cmd)
}
```

The `version` argument is optional; omit it to register the package without a
version string.

### `#[module_safe_init(Name, "version")]`

Generates `Name_SafeInit`, called by Tcl when loading the extension into a safe
interpreter. Use this to register only the subset of commands that are safe for
untrusted code. The function signature is identical to `module_init`.

```rust
#[module_safe_init(Example, "1.0.0")]
fn safe_init(interp: &Interpreter) -> Result<TclStatus, String> {
    interp.create_command("hello", hello_cmd)
}
```

### `#[module_unload(Name)]`

Generates `Name_Unload`, called by Tcl when the extension is unloaded. The
`flags` argument indicates whether only this interpreter is detaching
(`TclUnloadFlag::DetachFromInterpreter`) or whether the library is being
unmapped from the process entirely (`TclUnloadFlag::DetachFromProcess`).

```rust
#[module_unload(Example)]
fn unload(_interp: &Interpreter, _flags: TclUnloadFlag) -> Result<TclStatus, String> {
    Ok(TclStatus::Ok)
}
```

### `#[module_safe_unload(Name)]`

Generates `Name_SafeUnload`, the counterpart to `module_safe_init`. Called when
a safe interpreter that loaded the extension is torn down.

```rust
#[module_safe_unload(Example)]
fn safe_unload(_interp: &Interpreter, _flags: TclUnloadFlag) -> Result<TclStatus, String> {
    Ok(TclStatus::Ok)
}
```

### `#[derive(TclObjectType)]`

Implements `TclObjectType` for a struct, registering it as a native Tcl object
type. The struct must also implement `Clone`, `Display`, and `Debug`.

The generated implementation provides:
- A static `ObjectType` descriptor recognised by Tcl's type system.
- `from_object(&Object) -> Option<&Self>` — borrows the inner value from a Tcl
  object, converting from its string representation if needed.
- `into_object(self) -> Object` — wraps the value in a new Tcl object.
- A `From<Self> for Object` impl for convenience.

```rust
#[derive(Clone, Debug, TclObjectType)]
struct Point { x: f64, y: f64 }

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
```

Register the type with the interpreter before use:

```rust
interp.register_obj_type::<Point>();
```

## Naming Conventions

Tcl requires extension entry points to follow the pattern `Name_Init` where
`Name` has an uppercase first letter and all remaining letters lowercase. The
macros enforce this by using the name exactly as written — ensure it matches the
final library name (e.g. `Example` for `libexample.so`).