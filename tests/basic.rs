use std::cell::RefCell;

use rtea::*;

mod common;

use common::TestInterpreter;

#[test]
fn eval() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    let interp = test_interp.as_ref();
    assert_eq!(interp.eval("expr 5 + 5")?, TclStatus::Ok);
    let result = interp.get_obj_result();
    assert_eq!(interp.get_string(result), "10");
    Ok(())
}

#[test]
fn create_command() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    let interp = test_interp.as_ref();

    fn cmd(interp: &Interpreter, args: Vec<&str>) -> Result<TclStatus, String> {
        if args[1] == "fail" {
            Err("doing as told".to_string())
        } else {
            interp.set_result("pass");
            Ok(TclStatus::Ok)
        }
    }

    interp.create_command("mycmd", cmd)?;
    interp.eval("mycmd not_fail")?;
    let result = interp.get_obj_result();
    assert_eq!(interp.get_string(result), "pass");

    let eval = interp
        .eval("mycmd fail")
        .expect_err("cmd should error on 'fail' as argv[1]");
    let result = interp.get_string(interp.get_obj_result());
    assert_eq!(eval, result);

    Ok(())
}

#[test]
fn create_stateful_command() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    let interp = test_interp.as_ref();

    fn cmd(
        interp: &Interpreter,
        counter: &RefCell<usize>,
        _args: Vec<&str>,
    ) -> Result<TclStatus, String> {
        let mut val = counter.borrow_mut();
        interp.set_result(&val.to_string());
        *val += 1;

        Ok(TclStatus::Ok)
    }

    StatefulCommand::new(cmd, RefCell::<usize>::new(0)).attach_command(interp, "counter")?;
    for i in 0..10 {
        interp.eval("counter")?;
        assert_eq!(i.to_string(), interp.get_string(interp.get_obj_result()));
    }

    Ok(())
}

#[module_init(Bogus, "0.0.0")]
fn test_init(interp: &Interpreter) -> Result<TclStatus, String> {
    fn cmd(interp: &Interpreter, _args: Vec<&str>) -> Result<TclStatus, String> {
        interp.set_result("pass");
        Ok(TclStatus::Ok)
    }

    interp.create_command("mycmd", cmd)
}

#[test]
fn init_wrapper() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    assert_eq!(Bogus_Init(test_interp.as_ptr()), TclStatus::Ok);

    let interp = test_interp.as_ref();
    interp.eval("mycmd")?;
    let result = interp.get_obj_result();
    assert_eq!(interp.get_string(result), "pass");

    Ok(())
}
