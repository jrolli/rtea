use std::cell::RefCell;
use std::str::FromStr;

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
fn interpreter() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    test_interp.as_ref().init_global_functions();

    assert_eq!(
        test_interp
        .as_ref()
        .eval("return $tcl_version")
        .expect("interp failed to return result")
        .get_string()
        .to_string()
        , "9.0");
    Ok(())
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

#[derive(Debug, Clone, TclObjectType, PartialEq)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn add(&self, pt: &Point3D) -> Point3D {
        Point3D {
            x: self.x + pt.x,
            y: self.y + pt.y,
            z: self.z + pt.z,
        }
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[test]
fn obj_wrapper() -> Result<(), String> {
    let test_interp = TestInterpreter::new();
    assert_eq!(RteaTest_Init(test_interp.as_ptr()), TclStatus::Ok);

    let interp = test_interp.as_ref();
    // Register the object
    interp.register_obj_type::<Point3D>();

    fn cmd(interp: &Interpreter, args: Vec<Object>) -> Result<TclStatus, Object> {
        let pt1 = Point3D::from_object(&args[1]).unwrap();
        let pt2 = Point3D::from_object(&args[2]).unwrap();

        interp.set_obj_result(&pt1.add(pt2).into());
        Ok(TclStatus::Ok)
    }

    fn new_cmd(interp: &Interpreter, args: Vec<Object>) -> Result<TclStatus, Object> {
        if args.len() != 4 {
            return Err(Object::new_string(&format!("usage: {} x y z", args[0])));
        }

        let pt = Point3D {
            x: f64::from_str(args[1].get_string()).unwrap(),
            y: f64::from_str(args[2].get_string()).unwrap(),
            z: f64::from_str(args[3].get_string()).unwrap(),
        };

        interp.set_obj_result(&pt.into());
        Ok(TclStatus::Ok)
    }

    interp.create_obj_command("create_point", new_cmd)?;
    interp.create_obj_command("add_points", cmd)?;

    let result = interp
        .eval("add_points [create_point 1.0 2.0 3.0] [create_point 3.0 2.0 1.0]")
        .map_err(|obj| obj.get_string().to_string())?;
    assert_eq!(result.get_string(), "(4, 4, 4)");

    Ok(())
}
