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
        parser::{Assign, FunctionCall, Node, Number, Value, Var},
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
fn test_math_function_error_handling() {
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
        Value::Number(val) => assert!(val.is_nan(), "sqrt(-1) should return NaN"),
        _ => panic!("sqrt(-1) should return a Number value"),
    }

    // Test sqrt with zero - should work fine
    let sqrt_zero = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = sqrt_zero.accept(&mut interpreter);
    assert_eq!(result, Value::Number(0.0));

    // Test sqrt with very small negative number
    let sqrt_small_negative = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("-0.001"))],
    );
    let result = sqrt_small_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "sqrt(-0.001) should return NaN"),
        _ => panic!("sqrt(-0.001) should return a Number value"),
    }
}

#[test]
fn test_logarithm_error_handling() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test log with zero - should return NaN
    let log_zero = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("0"))],
    );
    let result = log_zero.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "log(0) should return NaN"),
        _ => panic!("log(0) should return a Number value"),
    }

    // Test log with negative input - should return NaN
    let log_negative = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("-1"))],
    );
    let result = log_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "log(-1) should return NaN"),
        _ => panic!("log(-1) should return a Number value"),
    }

    // Test log with positive input - should work fine
    let log_positive = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("2.718281828"))],
    );
    let result = log_positive.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1.0).abs() < 1e-6, "log(e) should be approximately 1"),
        _ => panic!("log(e) should return a Number value"),
    }

    // Test log with very small positive number
    let log_small_positive = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Number::parse("0.001"))],
    );
    let result = log_small_positive.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_finite() && !val.is_nan(), "log(0.001) should return a finite number"),
        _ => panic!("log(0.001) should return a Number value"),
    }
}

#[test]
fn test_trigonometric_edge_cases() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test tan with π/2 (90 degrees) - should return a very large number
    let tan_pi_half = FunctionCall::parse(
        "tan".to_string(),
        vec![Box::new(Number::parse("1.5707963267948966"))], // π/2
    );
    let result = tan_pi_half.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.abs() > 1e10, "tan(π/2) should return a very large number"),
        _ => panic!("tan(π/2) should return a Number value"),
    }

    // Test sin with very large input
    let sin_large = FunctionCall::parse(
        "sin".to_string(),
        vec![Box::new(Number::parse("1000000"))],
    );
    let result = sin_large.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val >= -1.0 && val <= 1.0, "sin should always be between -1 and 1"),
        _ => panic!("sin should return a Number value"),
    }

    // Test cos with very large input
    let cos_large = FunctionCall::parse(
        "cos".to_string(),
        vec![Box::new(Number::parse("1000000"))],
    );
    let result = cos_large.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val >= -1.0 && val <= 1.0, "cos should always be between -1 and 1"),
        _ => panic!("cos should return a Number value"),
    }
}

#[test]
fn test_extreme_value_handling() {
    use crate::{
        interpreter::Interpreter,
        parser::{FunctionCall, Node, Number, Value},
    };

    let mut interpreter = Interpreter::new();

    // Test exp with very large input - should return infinity
    let exp_large = FunctionCall::parse(
        "exp".to_string(),
        vec![Box::new(Number::parse("1000"))],
    );
    let result = exp_large.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_infinite(), "exp(1000) should return infinity"),
        _ => panic!("exp(1000) should return a Number value"),
    }

    // Test exp with very negative input - should return value close to zero
    let exp_negative = FunctionCall::parse(
        "exp".to_string(),
        vec![Box::new(Number::parse("-1000"))],
    );
    let result = exp_negative.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val >= 0.0 && val < 1e-100, "exp(-1000) should return a very small positive number"),
        _ => panic!("exp(-1000) should return a Number value"),
    }

    // Test sqrt with very large input
    let sqrt_large = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Number::parse("1000000"))],
    );
    let result = sqrt_large.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!((val - 1000.0).abs() < 1e-10, "sqrt(1000000) should be 1000"),
        _ => panic!("sqrt(1000000) should return a Number value"),
    }

    // Test abs with extreme values
    let abs_large_negative = FunctionCall::parse(
        "abs".to_string(),
        vec![Box::new(Number::parse("-999999999"))],
    );
    let result = abs_large_negative.accept(&mut interpreter);
    assert_eq!(result, Value::Number(999999999.0));
}

#[test]
fn test_chained_error_propagation() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, FunctionCall, Node, Number, Value, Var},
    };

    let mut interpreter = Interpreter::new();

    // Assign a negative value to a variable
    let assignment = Assign::parse(
        "negative_val".to_string(),
        Box::new(Number::parse("-4")),
    );
    assignment.accept(&mut interpreter);

    // Use the variable in sqrt function - should return NaN
    let sqrt_var = FunctionCall::parse(
        "sqrt".to_string(),
        vec![Box::new(Var::parse("negative_val".to_string()))],
    );
    let result = sqrt_var.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "sqrt of negative variable should return NaN"),
        _ => panic!("sqrt should return a Number value"),
    }

    // Assign zero to a variable
    let zero_assignment = Assign::parse(
        "zero_val".to_string(),
        Box::new(Number::parse("0")),
    );
    zero_assignment.accept(&mut interpreter);

    // Use the variable in log function - should return NaN
    let log_var = FunctionCall::parse(
        "log".to_string(),
        vec![Box::new(Var::parse("zero_val".to_string()))],
    );
    let result = log_var.accept(&mut interpreter);
    match result {
        Value::Number(val) => assert!(val.is_nan(), "log of zero variable should return NaN"),
        _ => panic!("log should return a Number value"),
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
