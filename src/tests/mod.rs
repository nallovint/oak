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
