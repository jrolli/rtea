use rtea::*;

#[link(name = "tcl")]
extern "C" {
    // fn Tcl_FindExecutable(name: *const c_char);
    fn Tcl_CreateInterp() -> *mut Interpreter;
    fn Tcl_DeleteInterp(interp: *mut Interpreter);
}

pub struct TestInterpreter {
    interp: *mut Interpreter,
}

impl Drop for TestInterpreter {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.interp) }
    }
}

impl TestInterpreter {
    pub fn new() -> TestInterpreter {
        let raw_interp = unsafe { Tcl_CreateInterp() };
        assert_ne!(raw_interp, std::ptr::null_mut());
        TestInterpreter { interp: raw_interp }
    }

    pub fn as_ptr(&self) -> *mut Interpreter {
        self.interp
    }

    pub fn as_ref(&self) -> &Interpreter {
        Interpreter::from_raw(self.as_ptr()).unwrap()
    }
}
