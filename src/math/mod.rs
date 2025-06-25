// Math module providing mathematical functions
use std::f64::consts::PI;

/// Mathematical functions for the Oak programming language
pub struct MathModule;

impl MathModule {
    /// Calculate the sine of an angle in radians
    /// Always defined for all real numbers
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// Calculate the cosine of an angle in radians
    /// Always defined for all real numbers
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Calculate the tangent of an angle in radians
    /// Returns NaN for undefined values (e.g., tan(PI/2), tan(3*PI/2))
    pub fn tan(x: f64) -> f64 {
        // Check for undefined tangent values (where cos(x) = 0)
        // These occur at PI/2 + n*PI for any integer n
        let cos_val = x.cos();
        if cos_val.abs() < f64::EPSILON {
            f64::NAN
        } else {
            x.tan()
        }
    }

    /// Calculate the square root of a number
    /// Returns NaN for negative numbers
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 {
            f64::NAN
        } else {
            x.sqrt()
        }
    }

    /// Calculate the natural logarithm of a number
    /// Returns NaN for non-positive numbers
    pub fn log(x: f64) -> f64 {
        if x <= 0.0 {
            f64::NAN
        } else {
            x.ln()
        }
    }

    /// Calculate e raised to the power of x
    /// Always defined for all real numbers
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    /// Calculate the absolute value of a number
    /// Always defined for all real numbers
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    /// Convert degrees to radians
    /// Always defined for all real numbers
    pub fn to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    /// Convert radians to degrees
    /// Always defined for all real numbers
    pub fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    /// Get the value of PI
    pub fn pi() -> f64 {
        PI
    }

    /// Get the value of e
    pub fn e() -> f64 {
        std::f64::consts::E
    }

    /// Check if a value is NaN (Not a Number)
    pub fn is_nan(x: f64) -> bool {
        x.is_nan()
    }

    /// Check if a value is infinite
    pub fn is_infinite(x: f64) -> bool {
        x.is_infinite()
    }

    /// Check if a value is finite (not NaN and not infinite)
    pub fn is_finite(x: f64) -> bool {
        x.is_finite()
    }
}

/// Function registry for math functions
pub fn get_math_functions() -> std::collections::HashMap<String, fn(f64) -> f64> {
    let mut functions = std::collections::HashMap::new();
    
    functions.insert("sin".to_string(), MathModule::sin as fn(f64) -> f64);
    functions.insert("cos".to_string(), MathModule::cos as fn(f64) -> f64);
    functions.insert("tan".to_string(), MathModule::tan as fn(f64) -> f64);
    functions.insert("sqrt".to_string(), MathModule::sqrt as fn(f64) -> f64);
    functions.insert("log".to_string(), MathModule::log as fn(f64) -> f64);
    functions.insert("exp".to_string(), MathModule::exp as fn(f64) -> f64);
    functions.insert("abs".to_string(), MathModule::abs as fn(f64) -> f64);
    functions.insert("to_radians".to_string(), MathModule::to_radians as fn(f64) -> f64);
    functions.insert("to_degrees".to_string(), MathModule::to_degrees as fn(f64) -> f64);
    
    functions
}

/// Function registry for math constants
pub fn get_math_constants() -> std::collections::HashMap<String, f64> {
    let mut constants = std::collections::HashMap::new();
    
    constants.insert("PI".to_string(), MathModule::pi());
    constants.insert("E".to_string(), MathModule::e());
    
    constants
} 