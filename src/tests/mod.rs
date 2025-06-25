#[cfg(test)]
#[allow(warnings)]
#[test]
fn test_binary_operation() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, BinOp, Node, Number, Value, Var},
    };

    let expr = BinOp::parse(
        Box::new(Number::parse("3")),
        "+".to_string(),
        Box::new(Number::parse("4")),
    );

    let assignment = Assign::parse("x".to_string(), Box::new(expr));
    let mut interpreter = Interpreter::new();

    assignment.accept(&mut interpreter);

    let var = Var::parse("x".to_string());
    let result = var.accept(&mut interpreter);

    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_math_functions() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sin function
    let sin_call = FunctionCall::parse(
        "sin".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = sin_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(0.0));

    // Test cos function
    let cos_call = FunctionCall::parse(
        "cos".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = cos_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(1.0));

    // Test sqrt function
    let sqrt_call = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("4"))],
    );
    let result = sqrt_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(2.0));

    // Test abs function
    let abs_call = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("-5"))],
    );
    let result = abs_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_math_functions_error_handling() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sqrt with negative input - should return NaN
    let sqrt_negative = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = sqrt_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("sqrt(-1) should return NaN"),
    }

    // Test log with zero - should return NaN
    let log_zero = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = log_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(0) should return NaN"),
    }

    // Test log with negative input - should return NaN
    let log_negative = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = log_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(-1) should return NaN"),
    }
}

#[test]
fn test_angle_conversion_functions() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test to_radians function
    let to_radians_call = FunctionCall::parse(
        "to_radians".to_string(),
        vec![Box::new(Number::parse("180"))],
    );
    let result = to_radians_call.accept(&mut interpreter);
    match result {
        Value::Number(val) => {
            assert!((val - std::f64::consts::PI).abs() < 1e-10);
        }
        _ => panic!("to_radians(180) should return PI"),
    }

    // Test to_degrees function
    let to_degrees_call = FunctionCall::parse(
        "to_degrees".to_string(),
        vec![Box::new(Number::parse(&std::f64::consts::PI.to_string()))],
    );
    let result = to_degrees_call.accept(&mut interpreter);
    match result {
        Value::Number(val) => {
            assert!((val - 180.0).abs() < 1e-10);
        }
        _ => panic!("to_degrees(PI) should return 180"),
    }
}

#[test]
fn test_math_constants() {
    use crate::{
        interpreter::Interpreter,
        parser::{Node, Value, Var},
    };

    let mut interpreter = Interpreter::new();

    // Test PI constant
    let pi_var = Var::parse("PI".to_string());
    let result = pi_var.accept(&mut interpreter);
    match result {
        Value::Number(pi_value) => {
            assert!((pi_value - std::f64::consts::PI).abs() < 1e-10);
        }
        _ => panic!("PI should be a number"),
    }

    // Test E constant
    let e_var = Var::parse("E".to_string());
    let result = e_var.accept(&mut interpreter);
    match result {
        Value::Number(e_value) => {
            assert!((e_value - std::f64::consts::E).abs() < 1e-10);
        }
        _ => panic!("E should be a number"),
    }
}

#[test]
fn test_math_function_with_variable() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, FunctionCall, Node, Number, Value, Var},
    };

    let mut interpreter = Interpreter::new();

    // Assign a value to a variable
    let assignment = Assign::parse(
        "x".to_string(),
        Box::new(Number::parse("16")),
    );
    assignment.accept(&mut interpreter);

    // Use the variable in a math function
    let sqrt_call = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Var::parse("x".to_string()))],
    );
    let result = sqrt_call.accept(&mut interpreter);
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_runtime_script_parsing() {
    use crate::parser::parse_script;

    let script_source: String = "./test.oak".to_string();

    if let Err(_) = parse_script(script_source) {
        println!("Failed to assert the result of file parsing was ok!");
        std::process::exit(1);
    } else {
        println!("File parsing result was ok!");
        std::process::exit(0);
    }
}

#[test]
fn test_math_functions_error_handling() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sqrt with negative input - should return NaN
    let sqrt_negative = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = sqrt_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("sqrt(-1) should return NaN"),
    }

    // Test log with zero - should return NaN
    let log_zero = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = log_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(0) should return NaN"),
    }

    // Test log with negative input - should return NaN
    let log_negative = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = log_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan()),
        _ => panic!("log(-1) should return NaN"),
    }

    // Test tan(PI/2) - should return NaN (undefined)
    let tan_pi_over_2 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("1.5707963267948966"))], // PI/2
    );
    let result = tan_pi_over_2.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "tan(PI/2) should return NaN, got {}", val),
        _ => panic!("tan(PI/2) should return NaN"),
    }

    // Test tan(3*PI/2) - should return NaN (undefined)
    let tan_3pi_over_2 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("4.71238898038469"))], // 3*PI/2
    );
    let result = tan_3pi_over_2.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "tan(3*PI/2) should return NaN, got {}", val),
        _ => panic!("tan(3*PI/2) should return NaN"),
    }

    // Test tan(0) - should return 0 (defined)
    let tan_zero = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = tan_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "tan(0) should return 0, got {}", val),
        _ => panic!("tan(0) should return 0"),
    }

    // Test tan(PI) - should return 0 (defined)
    let tan_pi = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("3.141592653589793"))], // PI
    );
    let result = tan_pi.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "tan(PI) should return 0, got {}", val),
        _ => panic!("tan(PI) should return 0"),
    }

    // Test tan(PI/4) - should return 1 (defined)
    let tan_pi_over_4 = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("0.7853981633974483"))], // PI/4
    );
    let result = tan_pi_over_4.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1.0).abs() < 1e-10, "tan(PI/4) should return 1, got {}", val),
        _ => panic!("tan(PI/4) should return 1"),
    }
}

#[test]
fn test_math_functions_edge_cases() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test sqrt(0) - should return 0
    let sqrt_zero = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = sqrt_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "sqrt(0) should return 0, got {}", val),
        _ => panic!("sqrt(0) should return 0"),
    }

    // Test log(1) - should return 0
    let log_one = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("1"))],
    );
    let result = log_one.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "log(1) should return 0, got {}", val),
        _ => panic!("log(1) should return 0"),
    }

    // Test exp(0) - should return 1
    let exp_zero = FunctionCall::parse(
        "exp".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = exp_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1.0).abs() < 1e-10, "exp(0) should return 1, got {}", val),
        _ => panic!("exp(0) should return 1"),
    }

    // Test abs(0) - should return 0
    let abs_zero = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = abs_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "abs(0) should return 0, got {}", val),
        _ => panic!("abs(0) should return 0"),
    }

    // Test abs(-0) - should return 0
    let abs_negative_zero = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("-0"))],
    );
    let result = abs_negative_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 0.0).abs() < 1e-10, "abs(-0) should return 0, got {}", val),
        _ => panic!("abs(-0) should return 0"),
    }
}
