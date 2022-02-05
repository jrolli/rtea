use std::cell::RefCell;

use rtea::*;

mod common;

use common::TestInterpreter;

#[module_init(RteaTest, "0.0.0")]
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
    assert_eq!(RteaTest_Init(test_interp.as_ptr()), TclStatus::Ok);

    let interp = test_interp.as_ref();
    let result = interp
        .eval("mycmd")
        .map_err(|obj| obj.get_string().to_string())?;
    assert_eq!(result.get_string(), "pass");

    Ok(())
}

#[test]
fn eval() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    assert_eq!(RteaTest_Init(test_interp.as_ptr()), TclStatus::Ok);
    let interp = test_interp.as_ref();
    let result = interp
        .eval("expr 5 + 5")
        .map_err(|obj| obj.get_string().to_string())?;
    assert_eq!(result.get_string(), "10");
    Ok(())
}

#[test]
fn create_command() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    assert_eq!(RteaTest_Init(test_interp.as_ptr()), TclStatus::Ok);
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
    let result = interp
        .eval("mycmd not_fail")
        .map_err(|obj| obj.get_string().to_string())?;
    assert_eq!(result.get_string(), "pass");

    let result = interp
        .eval("mycmd fail")
        .expect_err("cmd should error on 'fail' as argv[1]");
    assert_eq!("doing as told", result.get_string());

    Ok(())
}

#[test]
fn create_stateful_command() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    assert_eq!(RteaTest_Init(test_interp.as_ptr()), TclStatus::Ok);
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
        interp
            .eval("counter")
            .map_err(|obj| obj.get_string().to_string())?;
        assert_eq!(i.to_string(), interp.get_obj_result().get_string());
    }

    Ok(())
}
