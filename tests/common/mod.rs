use rtea::*;

#[link(name = "tcl")]
extern "C" {
    // fn Tcl_FindExecutable(name: *const c_char);
    fn Tcl_CreateInterp() -> *const Interpreter;
    fn Tcl_DeleteInterp(interp: *const Interpreter);
}

pub struct TestInterpreter {
    interp: *const Interpreter,
}

impl Drop for TestInterpreter {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.interp) }
    }
}

impl TestInterpreter {
    pub fn new() -> TestInterpreter {
        let raw_interp = unsafe { Tcl_CreateInterp() };
        assert_ne!(raw_interp, std::ptr::null());
        TestInterpreter { interp: raw_interp }
    }

    pub fn as_ptr(&self) -> *const Interpreter {
        self.interp
    }

    pub fn as_ref(&self) -> &Interpreter {
        Interpreter::from_raw(self.as_ptr()).unwrap()
    }
}
