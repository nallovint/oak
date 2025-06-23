# Math Functions in Oak

Oak provides a comprehensive set of mathematical functions for scientific and engineering calculations.

## Available Functions

### Trigonometric Functions
- `sin(x)` - Sine of x (x in radians)
- `cos(x)` - Cosine of x (x in radians)  
- `tan(x)` - Tangent of x (x in radians)

### Logarithmic and Exponential Functions
- `log(x)` - Natural logarithm of x
- `exp(x)` - e raised to the power of x

### Other Mathematical Functions
- `sqrt(x)` - Square root of x
- `abs(x)` - Absolute value of x
- `to_radians(degrees)` - Converts degrees to radians
- `to_degrees(radians)` - Converts radians to degrees

## Mathematical Constants

- `PI` - The mathematical constant π (approximately 3.14159)
- `E` - The mathematical constant e (approximately 2.71828)

## Usage Examples

### Basic Function Calls
```oak
BEGIN PROJ "math_example.project"
    BEGIN SECTION "main"
        var result := sqrt(16)
        print result  // Output: 4.0
        
        var angle := 0.5
        var sine_value := sin(angle)
        print sine_value
        
        var abs_value := abs(-5)
        print abs_value  // Output: 5.0
    END SECTION "main"
END PROJ "math_example.project"
```

### Using Mathematical Constants
```oak
BEGIN PROJ "constants_example.project"
    BEGIN SECTION "main"
        print PI   // Output: 3.141592653589793
        print E    // Output: 2.718281828459045
        
        var radius := 5
        var area := PI * radius * radius
        print area
    END SECTION "main"
END PROJ "constants_example.project"
```

### Combining Functions
```oak
BEGIN PROJ "combined_example.project"
    BEGIN SECTION "main"
        var x := 2
        var y := 3
        
        var result := sqrt(x * x + y * y)
        print result  // Output: 3.605551275463989
        
        var log_result := log(exp(2))
        print log_result  // Output: 2.0
    END SECTION "main"
END PROJ "combined_example.project"
```

### Angle Conversion Examples
```oak
BEGIN PROJ "angle_conversion.project"
    BEGIN SECTION "main"
        // Convert degrees to radians
        var degrees := 90
        var radians := to_radians(degrees)
        print radians  // Output: 1.5707963267948966
        
        // Convert radians to degrees
        var rad_value := PI / 4
        var deg_value := to_degrees(rad_value)
        print deg_value  // Output: 45.0
        
        // Using conversion with trigonometric functions
        var angle_deg := 30
        var angle_rad := to_radians(angle_deg)
        var sine_30 := sin(angle_rad)
        print sine_30  // Output: 0.5
        
        // Full circle conversion
        var full_circle_deg := 360
        var full_circle_rad := to_radians(full_circle_deg)
        print full_circle_rad  // Output: 6.283185307179586 (2π)
    END SECTION "main"
END PROJ "angle_conversion.project"
```

## Error Handling

- `sqrt(x)` returns `NaN` for negative values
- `log(x)` returns `NaN` for non-positive values
- All functions expect numeric arguments

## Implementation Details

The math functions are implemented using Rust's standard library mathematical functions, ensuring high precision and performance. The functions are available globally in any Oak script without requiring explicit imports.

## Future Enhancements

Planned additions to the math module include:
- Inverse trigonometric functions (asin, acos, atan)
- Hyperbolic functions (sinh, cosh, tanh)
- Power function (pow)
- Floor and ceiling functions
- Random number generation
- Statistical functions 