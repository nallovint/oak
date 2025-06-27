// Math module providing mathematical functions
use std::f64::consts::PI;

/// Mathematical functions for the Oak programming language
pub struct MathModule;

/// Building stability verification result
///
/// - If `overturning_moment` is near zero, `stability_ratio` will be 1e6 ("perfect stability").
/// - If `overturning_moment` is negative, the function returns an error.
#[derive(Debug, Clone)]
pub struct StabilityResult {
    pub resisting_moment: f64,
    pub overturning_moment: f64,
    pub stability_ratio: f64,
    pub is_stable: bool,
    pub safety_margin: f64,
}

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

    // Helper functions for building stability calculations

    /// Validate building dimension parameters
    /// 
    /// # Arguments
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// 
    /// # Returns
    /// * `Ok(())` if all parameters are valid
    /// * `Err(String)` with error message if validation fails
    fn validate_building_parameters(
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
    ) -> Result<(), String> {
        if building_length_a <= 0.0 {
            return Err("Building length must be positive".to_string());
        }
        if building_width_b <= 0.0 {
            return Err("Building width must be positive".to_string());
        }
        if building_height <= 0.0 {
            return Err("Building height must be positive".to_string());
        }
        if num_floors == 0 {
            return Err("Number of floors must be at least 1".to_string());
        }

        // Check for extremely small buildings that might cause numerical issues
        if building_length_a < 0.1 || building_width_b < 0.1 {
            return Err("Building dimensions must be at least 0.1 meters".to_string());
        }

        // Check for extremely large values that might cause overflow
        if building_length_a > 10000.0 || building_width_b > 10000.0 || building_height > 10000.0 {
            return Err("Building dimensions exceed maximum allowed values (10,000 m)".to_string());
        }

        Ok(())
    }

    /// Validate wind-related parameters
    /// 
    /// # Arguments
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `wind_force_height` - Height where wind force acts (m)
    /// * `building_height` - Total height of building (m)
    /// 
    /// # Returns
    /// * `Ok(())` if all parameters are valid
    /// * `Err(String)` with error message if validation fails
    fn validate_wind_parameters(
        wind_load_per_sqm: f64,
        wind_force_height: f64,
        building_height: f64,
    ) -> Result<(), String> {
        if wind_load_per_sqm <= 0.0 {
            return Err("Wind load per square meter must be positive".to_string());
        }
        if wind_force_height <= 0.0 || wind_force_height > building_height {
            return Err("Wind force height must be positive and not exceed building height".to_string());
        }
        Ok(())
    }

    /// Validate calculation result for overflow or NaN
    /// 
    /// # Arguments
    /// * `value` - The calculated value to validate
    /// * `calculation_name` - Name of the calculation for error messages
    /// 
    /// # Returns
    /// * `Ok(())` if the value is valid
    /// * `Err(String)` with error message if validation fails
    fn validate_calculation_result(value: f64, calculation_name: &str) -> Result<(), String> {
        if value.is_infinite() || value.is_nan() {
            return Err(format!("{} resulted in invalid value (overflow or NaN)", calculation_name));
        }
        Ok(())
    }

    /// Calculate center to corner distance (diagonal distance from center to corner)
    /// 
    /// # Arguments
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// 
    /// # Returns
    /// * `Ok(f64)` - The center to corner distance
    /// * `Err(String)` with error message if calculation fails
    fn calculate_center_to_corner_distance(
        building_length_a: f64,
        building_width_b: f64,
    ) -> Result<f64, String> {
        let center_to_corner_distance = MathModule::sqrt(
            (building_length_a / 2.0).powi(2) + (building_width_b / 2.0).powi(2)
        );

        // Check for invalid center to corner distance
        MathModule::validate_calculation_result(center_to_corner_distance, "Center to corner distance calculation")?;

        // Check for division by zero
        if center_to_corner_distance == 0.0 {
            return Err("Center to corner distance cannot be zero".to_string());
        }

        Ok(center_to_corner_distance)
    }

    /// Verify building stability against overturning due to wind loads
    /// 
    /// # Arguments
    /// * `dead_load_per_sqm` - Dead load per square meter (kN/m²)
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// * `wind_force_height` - Height where wind force acts (m), typically h/2
    /// 
    /// # Returns
    /// * `StabilityResult` with detailed calculation results
    ///
    /// # Special Cases
    /// * If overturning moment is near zero (abs < f64::EPSILON), returns a stability ratio of 1e6 ("perfect stability").
    /// * If overturning moment is negative, returns an error (physically impossible).
    ///
    /// # Safety Criterion
    /// The building is considered stable if Me/Mv >= 3
    /// where Me is the resisting moment and Mv is the overturning moment
    ///
    /// # Example
    /// ```rust
    /// use oak::MathModule;
    /// let result = MathModule::verify_building_stability(
    ///     5.0, 1.0, 20.0, 15.0, 30.0, 10, 15.0
    /// );
    /// assert!(result.is_ok());
    /// let stability = result.unwrap();
    /// assert!(stability.is_stable);
    /// ```
    pub fn verify_building_stability(
        dead_load_per_sqm: f64,
        wind_load_per_sqm: f64,
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
        wind_force_height: f64,
    ) -> Result<StabilityResult, String> {
        // Validate input parameters
        if dead_load_per_sqm <= 0.0 {
            return Err("Dead load per square meter must be positive".to_string());
        }
        MathModule::validate_building_parameters(building_length_a, building_width_b, building_height, num_floors)?;
        MathModule::validate_wind_parameters(wind_load_per_sqm, wind_force_height, building_height)?;

        // Calculate total dead load G
        let total_dead_load = dead_load_per_sqm * building_length_a * building_width_b * num_floors as f64;
        MathModule::validate_calculation_result(total_dead_load, "Dead load calculation")?;

        // Calculate distance from center of gravity to furthest corner (da)
        let center_to_corner_distance = MathModule::calculate_center_to_corner_distance(building_length_a, building_width_b)?;

        // Calculate resisting moment Me = G * da
        let resisting_moment = total_dead_load * center_to_corner_distance;
        MathModule::validate_calculation_result(resisting_moment, "Resisting moment calculation")?;

        // Calculate wind force W = qw * h * a
        let wind_force = wind_load_per_sqm * building_height * building_length_a;
        MathModule::validate_calculation_result(wind_force, "Wind force calculation")?;

        // Calculate overturning moment Mv = W * d
        let overturning_moment = wind_force * wind_force_height;
        MathModule::validate_calculation_result(overturning_moment, "Overturning moment calculation")?;

        // Calculate stability ratio with division by zero and negative protection
        let stability_ratio = if overturning_moment > f64::EPSILON {
            let ratio = resisting_moment / overturning_moment;
            if ratio.is_infinite() || ratio.is_nan() {
                return Err("Stability ratio calculation resulted in invalid value".to_string());
            }
            ratio
        } else if overturning_moment.abs() < f64::EPSILON {
            // Special case: no overturning moment means perfect stability
            // Use a large finite value to indicate this
            1e6
        } else {
            return Err("Negative overturning moment is physically impossible".to_string());
        };

        // Check stability criterion (Me/Mv >= 3)
        let is_stable = stability_ratio >= 3.0;
        let safety_margin = stability_ratio - 3.0;

        // Final validation of result values
        MathModule::validate_calculation_result(safety_margin, "Safety margin calculation")?;

        Ok(StabilityResult {
            resisting_moment,
            overturning_moment,
            stability_ratio,
            is_stable,
            safety_margin,
        })
    }

    /// Calculate the minimum required dead load for stability
    /// 
    /// # Arguments
    /// * `wind_load_per_sqm` - Wind load per square meter (kN/m²)
    /// * `building_length_a` - Length of windward face (m)
    /// * `building_width_b` - Width perpendicular to wind (m)
    /// * `building_height` - Total height of building (m)
    /// * `num_floors` - Number of floors (integer)
    /// * `wind_force_height` - Height where wind force acts (m)
    /// * `safety_factor` - Required safety factor (default 3.0)
    /// 
    /// # Returns
    /// * Minimum dead load per square meter required for stability
    pub fn calculate_minimum_dead_load(
        wind_load_per_sqm: f64,
        building_length_a: f64,
        building_width_b: f64,
        building_height: f64,
        num_floors: u32,
        wind_force_height: f64,
        safety_factor: f64,
    ) -> Result<f64, String> {
        // Validate input parameters
        MathModule::validate_building_parameters(building_length_a, building_width_b, building_height, num_floors)?;
        MathModule::validate_wind_parameters(wind_load_per_sqm, wind_force_height, building_height)?;
        if safety_factor <= 0.0 {
            return Err("Safety factor must be positive".to_string());
        }

        // Calculate wind force
        let wind_force = wind_load_per_sqm * building_height * building_length_a;
        MathModule::validate_calculation_result(wind_force, "Wind force calculation")?;
        
        // Calculate overturning moment
        let overturning_moment = wind_force * wind_force_height;
        MathModule::validate_calculation_result(overturning_moment, "Overturning moment calculation")?;
        
        // Calculate center to corner distance
        let center_to_corner_distance = MathModule::calculate_center_to_corner_distance(building_length_a, building_width_b)?;
        
        // Calculate required resisting moment
        let required_resisting_moment = overturning_moment * safety_factor;
        MathModule::validate_calculation_result(required_resisting_moment, "Required resisting moment calculation")?;
        
        // Calculate required total dead load
        let required_total_dead_load = required_resisting_moment / center_to_corner_distance;
        MathModule::validate_calculation_result(required_total_dead_load, "Required total dead load calculation")?;
        
        // Calculate building area
        let building_area = building_length_a * building_width_b * num_floors as f64;
        
        // Check for division by zero
        if building_area == 0.0 {
            return Err("Building area cannot be zero".to_string());
        }
        
        // Calculate required dead load per square meter
        let required_dead_load_per_sqm = required_total_dead_load / building_area;
        
        // Final validation of result
        MathModule::validate_calculation_result(required_dead_load_per_sqm, "Required dead load per square meter calculation")?;
        
        Ok(required_dead_load_per_sqm)
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