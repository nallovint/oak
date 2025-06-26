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

#[test]
fn test_building_stability_verification() {
    use crate::math::MathModule;

    // Test case 1: Stable building
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm (kN/m²)
        1.0,    // wind_load_per_sqm (kN/m²)
        20.0,   // building_length_a (m)
        15.0,   // building_width_b (m)
        30.0,   // building_height (m)
        10.0,   // num_floors
        15.0,   // wind_force_height (m) - mid-height
    ).unwrap();

    // Verify the building is stable (Me/Mv > 3)
    assert!(result.is_stable, "Building should be stable");
    assert!(result.stability_ratio > 3.0, "Stability ratio should be > 3.0");
    assert!(result.safety_margin > 0.0, "Safety margin should be positive");

    // Verify calculations
    let expected_total_dead_load = 5.0 * 20.0 * 15.0 * 10.0; // 15000 kN
    let expected_center_to_corner = MathModule::sqrt((20.0_f64/2.0).powi(2) + (15.0_f64/2.0).powi(2)); // 12.5 m
    let expected_resisting_moment = expected_total_dead_load * expected_center_to_corner;
    let expected_wind_force = 1.0 * 30.0 * 20.0; // 600 kN
    let expected_overturning_moment = expected_wind_force * 15.0; // 9000 kN·m

    assert!((result.resisting_moment - expected_resisting_moment).abs() < 1e-6);
    assert!((result.overturning_moment - expected_overturning_moment).abs() < 1e-6);
}

#[test]
fn test_building_stability_unstable() {
    use crate::math::MathModule;

    // Test case 2: Unstable building (high wind load, low dead load)
    let result = MathModule::verify_building_stability(
        1.0,    // dead_load_per_sqm (kN/m²) - very low
        5.0,    // wind_load_per_sqm (kN/m²) - very high
        10.0,   // building_length_a (m)
        10.0,   // building_width_b (m)
        20.0,   // building_height (m)
        5.0,    // num_floors
        10.0,   // wind_force_height (m)
    ).unwrap();

    // Verify the building is unstable (Me/Mv < 3)
    assert!(!result.is_stable, "Building should be unstable");
    assert!(result.stability_ratio < 3.0, "Stability ratio should be < 3.0");
    assert!(result.safety_margin < 0.0, "Safety margin should be negative");
}

#[test]
fn test_building_stability_edge_cases() {
    use crate::math::MathModule;

    // Test case 3: Very tall, narrow building
    let result = MathModule::verify_building_stability(
        8.0,    // dead_load_per_sqm (kN/m²)
        2.0,    // wind_load_per_sqm (kN/m²)
        5.0,    // building_length_a (m) - narrow
        5.0,    // building_width_b (m) - narrow
        100.0,  // building_height (m) - very tall
        20.0,   // num_floors
        50.0,   // wind_force_height (m) - mid-height
    ).unwrap();

    // This should be unstable due to high wind moment on narrow base
    assert!(!result.is_stable, "Tall narrow building should be unstable");
}

#[test]
fn test_building_stability_validation_errors() {
    use crate::math::MathModule;

    // Test negative dead load
    let result = MathModule::verify_building_stability(
        -1.0,   // negative dead load
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    );
    assert!(result.is_err(), "Should return error for negative dead load");

    // Test zero wind load
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        0.0,    // zero wind load
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    );
    assert!(result.is_err(), "Should return error for zero wind load");

    // Test invalid wind force height
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        35.0,   // wind_force_height > building_height
    );
    assert!(result.is_err(), "Should return error for invalid wind force height");
}

#[test]
fn test_calculate_minimum_dead_load() {
    use crate::math::MathModule;

    // Test case: Calculate minimum dead load for stability
    let min_dead_load = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm (kN/m²)
        15.0,   // building_length_a (m)
        12.0,   // building_width_b (m)
        25.0,   // building_height (m)
        8.0,    // num_floors
        12.5,   // wind_force_height (m) - mid-height
        3.0,    // safety_factor
    ).unwrap();

    // Verify the calculated minimum dead load is positive
    assert!(min_dead_load > 0.0, "Minimum dead load should be positive");

    // Verify that using this minimum dead load results in exactly Me/Mv = 3.0
    let result = MathModule::verify_building_stability(
        min_dead_load,  // calculated minimum dead load
        2.0,            // wind_load_per_sqm
        15.0,           // building_length_a
        12.0,           // building_width_b
        25.0,           // building_height
        8.0,            // num_floors
        12.5,           // wind_force_height
    ).unwrap();

    assert!((result.stability_ratio - 3.0).abs() < 1e-6, 
        "Stability ratio should be exactly 3.0 for minimum dead load");
}

#[test]
fn test_calculate_minimum_dead_load_validation() {
    use crate::math::MathModule;

    // Test negative wind load
    let result = MathModule::calculate_minimum_dead_load(
        -1.0,   // negative wind load
        15.0,   // building_length_a
        12.0,   // building_width_b
        25.0,   // building_height
        8.0,    // num_floors
        12.5,   // wind_force_height
        3.0,    // safety_factor
    );
    assert!(result.is_err(), "Should return error for negative wind load");

    // Test zero safety factor
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm
        15.0,   // building_length_a
        12.0,   // building_width_b
        25.0,   // building_height
        8.0,    // num_floors
        12.5,   // wind_force_height
        0.0,    // zero safety factor
    );
    assert!(result.is_err(), "Should return error for zero safety factor");
}

#[test]
fn test_stability_result_structure() {
    use crate::math::MathModule;

    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    ).unwrap();

    // Verify all fields are present and have reasonable values
    assert!(result.resisting_moment > 0.0, "Resisting moment should be positive");
    assert!(result.overturning_moment > 0.0, "Overturning moment should be positive");
    assert!(result.stability_ratio > 0.0, "Stability ratio should be positive");
    assert!(result.safety_margin > -3.0, "Safety margin should be > -3.0");

    // Test that the result can be cloned
    let cloned_result = result.clone();
    assert_eq!(result.stability_ratio, cloned_result.stability_ratio);
    assert_eq!(result.is_stable, cloned_result.is_stable);
}

#[test]
fn test_building_stability_extreme_values() {
    use crate::math::MathModule;

    // Test extremely small building dimensions
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        0.05,   // building_length_a - too small
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    );
    assert!(result.is_err(), "Should return error for extremely small building length");

    // Test extremely large building dimensions
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20000.0, // building_length_a - too large
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    );
    assert!(result.is_err(), "Should return error for extremely large building length");

    // Test extremely small building width
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        0.05,   // building_width_b - too small
        30.0,   // building_height
        10.0,   // num_floors
        15.0,   // wind_force_height
    );
    assert!(result.is_err(), "Should return error for extremely small building width");
}

#[test]
fn test_calculate_minimum_dead_load_extreme_values() {
    use crate::math::MathModule;

    // Test extremely small building dimensions
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm
        0.05,   // building_length_a - too small
        12.0,   // building_width_b
        25.0,   // building_height
        8.0,    // num_floors
        12.5,   // wind_force_height
        3.0,    // safety_factor
    );
    assert!(result.is_err(), "Should return error for extremely small building length");

    // Test extremely large building dimensions
    let result = MathModule::calculate_minimum_dead_load(
        2.0,    // wind_load_per_sqm
        20000.0, // building_length_a - too large
        12.0,   // building_width_b
        25.0,   // building_height
        8.0,    // num_floors
        12.5,   // wind_force_height
        3.0,    // safety_factor
    );
    assert!(result.is_err(), "Should return error for extremely large building length");
}

#[test]
fn test_building_stability_overflow_protection() {
    use crate::math::MathModule;

    // Test with very large values that might cause overflow
    let result = MathModule::verify_building_stability(
        1e6,    // very large dead load
        1e6,    // very large wind load
        1000.0, // large building length
        1000.0, // large building width
        1000.0, // large building height
        100.0,  // many floors
        500.0,  // wind force height
    );

    // This should either succeed with valid results or fail with overflow error
    match result {
        Ok(stability_result) => {
            // If it succeeds, verify the results are reasonable
            assert!(stability_result.resisting_moment > 0.0);
            assert!(stability_result.overturning_moment > 0.0);
            assert!(stability_result.stability_ratio > 0.0);
            assert!(!stability_result.stability_ratio.is_infinite());
            assert!(!stability_result.stability_ratio.is_nan());
        }
        Err(error_msg) => {
            // If it fails, it should be due to overflow protection
            assert!(error_msg.contains("overflow") || error_msg.contains("invalid value"));
        }
    }
}

#[test]
fn test_calculate_minimum_dead_load_overflow_protection() {
    use crate::math::MathModule;

    // Test with very large values that might cause overflow
    let result = MathModule::calculate_minimum_dead_load(
        1e6,    // very large wind load
        1000.0, // large building length
        1000.0, // large building width
        1000.0, // large building height
        100.0,  // many floors
        500.0,  // wind force height
        3.0,    // safety_factor
    );

    // This should either succeed with valid results or fail with overflow error
    match result {
        Ok(min_dead_load) => {
            // If it succeeds, verify the result is reasonable
            assert!(min_dead_load > 0.0);
            assert!(!min_dead_load.is_infinite());
            assert!(!min_dead_load.is_nan());
        }
        Err(error_msg) => {
            // If it fails, it should be due to overflow protection
            assert!(error_msg.contains("overflow") || error_msg.contains("invalid value"));
        }
    }
}

#[test]
fn test_building_stability_zero_overturning_moment() {
    use crate::math::MathModule;

    // Test case where overturning moment would be zero (edge case)
    // This should result in infinite stability ratio
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        0.0,    // wind_force_height - zero (invalid)
    );
    assert!(result.is_err(), "Should return error for zero wind force height");

    // Test with very small wind force height (should work)
    let result = MathModule::verify_building_stability(
        5.0,    // dead_load_per_sqm
        1.0,    // wind_load_per_sqm
        20.0,   // building_length_a
        15.0,   // building_width_b
        30.0,   // building_height
        10.0,   // num_floors
        0.1,    // very small wind force height
    ).unwrap();

    // Should be very stable due to small overturning moment
    assert!(result.is_stable, "Building should be stable with very small overturning moment");
    assert!(result.stability_ratio > 3.0, "Stability ratio should be > 3.0");
}
