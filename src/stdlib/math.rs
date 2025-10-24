// Math Standard Library for Jounce
// Comprehensive mathematical functions and constants

pub const MATH_DEFINITION: &str = r#"
// Mathematical Constants
namespace Math {
    // Pi (π) - ratio of circle's circumference to diameter
    pub const PI: f64 = 3.141592653589793;

    // Euler's number (e) - base of natural logarithm
    pub const E: f64 = 2.718281828459045;

    // Tau (τ) - 2π, full circle in radians
    pub const TAU: f64 = 6.283185307179586;

    // Square root of 2
    pub const SQRT_2: f64 = 1.4142135623730951;

    // 1 / Square root of 2
    pub const FRAC_1_SQRT_2: f64 = 0.7071067811865476;

    // Pi / 2
    pub const FRAC_PI_2: f64 = 1.5707963267948966;

    // Pi / 4
    pub const FRAC_PI_4: f64 = 0.7853981633974483;

    // 1 / Pi
    pub const FRAC_1_PI: f64 = 0.3183098861837907;

    // 2 / Pi
    pub const FRAC_2_PI: f64 = 0.6366197723675814;

    // Natural logarithm of 2
    pub const LN_2: f64 = 0.6931471805599453;

    // Natural logarithm of 10
    pub const LN_10: f64 = 2.302585092994046;

    // Log base 2 of e
    pub const LOG2_E: f64 = 1.4426950408889634;

    // Log base 10 of e
    pub const LOG10_E: f64 = 0.4342944819032518;

    // Infinity
    pub const INFINITY: f64 = 1.0 / 0.0;

    // Negative infinity
    pub const NEG_INFINITY: f64 = -1.0 / 0.0;

    // Not a Number
    pub const NAN: f64 = 0.0 / 0.0;

    // ========== Basic Operations ==========

    /// Returns the absolute value of a number
    pub fn abs(x: f64) -> f64 {
        if x < 0.0 {
            -x
        } else {
            x
        }
    }

    /// Returns the absolute value of an integer
    pub fn abs_i32(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }

    /// Returns the smaller of two numbers
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b { a } else { b }
    }

    /// Returns the smaller of two integers
    pub fn min_i32(a: i32, b: i32) -> i32 {
        if a < b { a } else { b }
    }

    /// Returns the larger of two numbers
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    /// Returns the larger of two integers
    pub fn max_i32(a: i32, b: i32) -> i32 {
        if a > b { a } else { b }
    }

    /// Clamps a value between a minimum and maximum
    pub fn clamp(value: f64, min_val: f64, max_val: f64) -> f64 {
        if value < min_val {
            min_val
        } else if value > max_val {
            max_val
        } else {
            value
        }
    }

    /// Clamps an integer between a minimum and maximum
    pub fn clamp_i32(value: i32, min_val: i32, max_val: i32) -> i32 {
        if value < min_val {
            min_val
        } else if value > max_val {
            max_val
        } else {
            value
        }
    }

    /// Returns the sign of a number (-1.0, 0.0, or 1.0)
    pub fn sign(x: f64) -> f64 {
        if x < 0.0 {
            -1.0
        } else if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    /// Returns the sign of an integer (-1, 0, or 1)
    pub fn sign_i32(x: i32) -> i32 {
        if x < 0 {
            -1
        } else if x > 0 {
            1
        } else {
            0
        }
    }

    // ========== Power and Root Functions ==========

    /// Returns x raised to the power of y (x^y)
    /// Uses exponentiation by squaring for integer exponents
    pub fn pow(x: f64, y: f64) -> f64 {
        // For integer exponents, use fast algorithm
        let y_int = y as i32;
        if (y_int as f64) == y {
            return pow_int(x, y_int);
        }

        // For fractional exponents, use exp(y * ln(x))
        if x <= 0.0 {
            return 0.0; // Simplified handling
        }
        exp(y * ln(x))
    }

    /// Helper: x raised to integer power n
    fn pow_int(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut result = 1.0;
        let mut base = x;
        let mut exp = abs_i32(n);

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            exp = exp / 2;
        }

        if n < 0 {
            1.0 / result
        } else {
            result
        }
    }

    /// Returns the square root of x using Newton's method
    pub fn sqrt(x: f64) -> f64 {
        if x < 0.0 {
            return NAN;
        }
        if x == 0.0 {
            return 0.0;
        }

        // Newton's method: x_{n+1} = (x_n + a/x_n) / 2
        let mut guess = x / 2.0;
        let mut prev_guess = 0.0;
        let epsilon = 0.00000001;

        let mut iterations = 0;
        while abs(guess - prev_guess) > epsilon && iterations < 100 {
            prev_guess = guess;
            guess = (guess + x / guess) / 2.0;
            iterations = iterations + 1;
        }

        guess
    }

    /// Returns the cube root of x
    pub fn cbrt(x: f64) -> f64 {
        if x == 0.0 {
            return 0.0;
        }

        let is_negative = x < 0.0;
        let abs_x = abs(x);

        // Newton's method: x_{n+1} = (2*x_n + a/(x_n^2)) / 3
        let mut guess = abs_x / 3.0;
        let mut prev_guess = 0.0;
        let epsilon = 0.00000001;

        let mut iterations = 0;
        while abs(guess - prev_guess) > epsilon && iterations < 100 {
            prev_guess = guess;
            guess = (2.0 * guess + abs_x / (guess * guess)) / 3.0;
            iterations = iterations + 1;
        }

        if is_negative {
            -guess
        } else {
            guess
        }
    }

    /// Returns x raised to the power of 2 (x^2)
    pub fn square(x: f64) -> f64 {
        x * x
    }

    /// Returns x raised to the power of 3 (x^3)
    pub fn cube(x: f64) -> f64 {
        x * x * x
    }

    // ========== Exponential and Logarithmic Functions ==========

    /// Returns e raised to the power of x (e^x)
    /// Uses Taylor series: e^x = 1 + x + x^2/2! + x^3/3! + ...
    pub fn exp(x: f64) -> f64 {
        if x == 0.0 {
            return 1.0;
        }

        let mut sum = 1.0;
        let mut term = 1.0;
        let mut n = 1.0;

        // Taylor series expansion
        let mut i = 0;
        while i < 100 {
            term = term * x / n;
            sum = sum + term;

            if abs(term) < 0.00000001 {
                return sum;
            }

            n = n + 1.0;
            i = i + 1;
        }

        sum
    }

    /// Returns 2 raised to the power of x (2^x)
    pub fn exp2(x: f64) -> f64 {
        pow(2.0, x)
    }

    /// Returns the natural logarithm of x (ln(x))
    /// Uses series expansion around 1
    pub fn ln(x: f64) -> f64 {
        if x <= 0.0 {
            return NAN;
        }
        if x == 1.0 {
            return 0.0;
        }

        // For values far from 1, use reduction
        if x > 2.0 {
            let mut result = 0.0;
            let mut val = x;
            while val > 2.0 {
                val = val / E;
                result = result + 1.0;
            }
            return result + ln(val);
        }

        // Series: ln(x) = 2 * [(x-1)/(x+1) + 1/3*((x-1)/(x+1))^3 + ...]
        let y = (x - 1.0) / (x + 1.0);
        let y_squared = y * y;
        let mut sum = 0.0;
        let mut term = y;
        let mut n = 1.0;

        let mut i = 0;
        while i < 100 {
            sum = sum + term / n;
            term = term * y_squared;
            n = n + 2.0;

            if abs(term / n) < 0.00000001 {
                return 2.0 * sum;
            }

            i = i + 1;
        }

        2.0 * sum
    }

    /// Returns the base 2 logarithm of x (log2(x))
    pub fn log2(x: f64) -> f64 {
        ln(x) / LN_2
    }

    /// Returns the base 10 logarithm of x (log10(x))
    pub fn log10(x: f64) -> f64 {
        ln(x) / LN_10
    }

    /// Returns the logarithm of x with arbitrary base
    pub fn log(x: f64, base: f64) -> f64 {
        ln(x) / ln(base)
    }

    // ========== Rounding Functions ==========

    /// Rounds x to the nearest integer (away from zero for .5)
    pub fn round(x: f64) -> f64 {
        if x >= 0.0 {
            floor(x + 0.5)
        } else {
            ceil(x - 0.5)
        }
    }

    /// Returns the largest integer less than or equal to x
    pub fn floor(x: f64) -> f64 {
        let truncated = trunc(x);
        if x < 0.0 && truncated != x {
            truncated - 1.0
        } else {
            truncated
        }
    }

    /// Returns the smallest integer greater than or equal to x
    pub fn ceil(x: f64) -> f64 {
        let truncated = trunc(x);
        if x > 0.0 && truncated != x {
            truncated + 1.0
        } else {
            truncated
        }
    }

    /// Removes the fractional part of x
    pub fn trunc(x: f64) -> f64 {
        x as i32 as f64
    }

    /// Returns the fractional part of x
    pub fn fract(x: f64) -> f64 {
        x - trunc(x)
    }

    // ========== Trigonometric Functions ==========

    /// Returns the sine of x (x in radians)
    /// Uses Taylor series: sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
    pub fn sin(x: f64) -> f64 {
        // Reduce to range [-π, π]
        let mut angle = x;
        while angle > PI {
            angle = angle - TAU;
        }
        while angle < -PI {
            angle = angle + TAU;
        }

        let mut sum = angle;
        let mut term = angle;
        let x_squared = angle * angle;

        let mut i = 1;
        while i < 20 {
            term = -term * x_squared / ((2.0 * i as f64) * (2.0 * i as f64 + 1.0));
            sum = sum + term;

            if abs(term) < 0.00000001 {
                return sum;
            }

            i = i + 1;
        }

        sum
    }

    /// Returns the cosine of x (x in radians)
    /// Uses Taylor series: cos(x) = 1 - x^2/2! + x^4/4! - x^6/6! + ...
    pub fn cos(x: f64) -> f64 {
        // Reduce to range [-π, π]
        let mut angle = x;
        while angle > PI {
            angle = angle - TAU;
        }
        while angle < -PI {
            angle = angle + TAU;
        }

        let mut sum = 1.0;
        let mut term = 1.0;
        let x_squared = angle * angle;

        let mut i = 1;
        while i < 20 {
            term = -term * x_squared / ((2.0 * i as f64 - 1.0) * (2.0 * i as f64));
            sum = sum + term;

            if abs(term) < 0.00000001 {
                return sum;
            }

            i = i + 1;
        }

        sum
    }

    /// Returns the tangent of x (x in radians)
    pub fn tan(x: f64) -> f64 {
        let cos_x = cos(x);
        if abs(cos_x) < 0.00000001 {
            return INFINITY;
        }
        sin(x) / cos_x
    }

    /// Returns the arcsine of x (result in radians, range [-π/2, π/2])
    pub fn asin(x: f64) -> f64 {
        if x < -1.0 || x > 1.0 {
            return NAN;
        }
        if x == 1.0 {
            return FRAC_PI_2;
        }
        if x == -1.0 {
            return -FRAC_PI_2;
        }

        // Use atan2 for better accuracy
        atan2(x, sqrt(1.0 - x * x))
    }

    /// Returns the arccosine of x (result in radians, range [0, π])
    pub fn acos(x: f64) -> f64 {
        if x < -1.0 || x > 1.0 {
            return NAN;
        }

        FRAC_PI_2 - asin(x)
    }

    /// Returns the arctangent of x (result in radians, range [-π/2, π/2])
    /// Uses Taylor series and reduction
    pub fn atan(x: f64) -> f64 {
        if x > 1.0 {
            return FRAC_PI_2 - atan(1.0 / x);
        }
        if x < -1.0 {
            return -FRAC_PI_2 - atan(1.0 / x);
        }

        // Taylor series: atan(x) = x - x^3/3 + x^5/5 - x^7/7 + ...
        let mut sum = 0.0;
        let mut term = x;
        let x_squared = x * x;

        let mut i = 0;
        while i < 50 {
            let n = 2.0 * i as f64 + 1.0;
            if i % 2 == 0 {
                sum = sum + term / n;
            } else {
                sum = sum - term / n;
            }
            term = term * x_squared;

            if abs(term / n) < 0.00000001 {
                return sum;
            }

            i = i + 1;
        }

        sum
    }

    /// Returns the arctangent of y/x, handling quadrants correctly
    /// Result in radians, range [-π, π]
    pub fn atan2(y: f64, x: f64) -> f64 {
        if x > 0.0 {
            atan(y / x)
        } else if x < 0.0 && y >= 0.0 {
            atan(y / x) + PI
        } else if x < 0.0 && y < 0.0 {
            atan(y / x) - PI
        } else if x == 0.0 && y > 0.0 {
            FRAC_PI_2
        } else if x == 0.0 && y < 0.0 {
            -FRAC_PI_2
        } else {
            0.0  // x == 0 && y == 0
        }
    }

    // ========== Hyperbolic Functions ==========

    /// Returns the hyperbolic sine of x
    pub fn sinh(x: f64) -> f64 {
        (exp(x) - exp(-x)) / 2.0
    }

    /// Returns the hyperbolic cosine of x
    pub fn cosh(x: f64) -> f64 {
        (exp(x) + exp(-x)) / 2.0
    }

    /// Returns the hyperbolic tangent of x
    pub fn tanh(x: f64) -> f64 {
        let exp_2x = exp(2.0 * x);
        (exp_2x - 1.0) / (exp_2x + 1.0)
    }

    // ========== Angle Conversion ==========

    /// Converts degrees to radians
    pub fn to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    /// Converts radians to degrees
    pub fn to_degrees(radians: f64) -> f64 {
        radians * 180.0 / PI
    }

    // ========== Utility Functions ==========

    /// Returns the hypotenuse of a right triangle with sides x and y
    pub fn hypot(x: f64, y: f64) -> f64 {
        sqrt(x * x + y * y)
    }

    /// Linear interpolation between a and b by factor t
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    /// Checks if two numbers are approximately equal within epsilon
    pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        abs(a - b) < epsilon
    }

    /// Returns true if x is NaN
    pub fn is_nan(x: f64) -> bool {
        x != x
    }

    /// Returns true if x is infinite
    pub fn is_infinite(x: f64) -> bool {
        x == INFINITY || x == NEG_INFINITY
    }

    /// Returns true if x is finite (not NaN or infinite)
    pub fn is_finite(x: f64) -> bool {
        !is_nan(x) && !is_infinite(x)
    }

    /// Copies the sign of y to the magnitude of x
    pub fn copysign(x: f64, y: f64) -> f64 {
        if y >= 0.0 {
            abs(x)
        } else {
            -abs(x)
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_definition_not_empty() {
        assert!(!MATH_DEFINITION.is_empty());
        assert!(MATH_DEFINITION.contains("namespace Math"));
    }

    #[test]
    fn test_math_has_constants() {
        assert!(MATH_DEFINITION.contains("const PI"));
        assert!(MATH_DEFINITION.contains("const E"));
        assert!(MATH_DEFINITION.contains("const TAU"));
    }

    #[test]
    fn test_math_has_basic_functions() {
        assert!(MATH_DEFINITION.contains("fn abs"));
        assert!(MATH_DEFINITION.contains("fn min"));
        assert!(MATH_DEFINITION.contains("fn max"));
        assert!(MATH_DEFINITION.contains("fn clamp"));
    }

    #[test]
    fn test_math_has_trig_functions() {
        assert!(MATH_DEFINITION.contains("fn sin"));
        assert!(MATH_DEFINITION.contains("fn cos"));
        assert!(MATH_DEFINITION.contains("fn tan"));
        assert!(MATH_DEFINITION.contains("fn asin"));
        assert!(MATH_DEFINITION.contains("fn acos"));
        assert!(MATH_DEFINITION.contains("fn atan"));
        assert!(MATH_DEFINITION.contains("fn atan2"));
    }

    #[test]
    fn test_math_has_exp_log_functions() {
        assert!(MATH_DEFINITION.contains("fn exp"));
        assert!(MATH_DEFINITION.contains("fn ln"));
        assert!(MATH_DEFINITION.contains("fn log2"));
        assert!(MATH_DEFINITION.contains("fn log10"));
    }

    #[test]
    fn test_math_has_power_functions() {
        assert!(MATH_DEFINITION.contains("fn pow"));
        assert!(MATH_DEFINITION.contains("fn sqrt"));
        assert!(MATH_DEFINITION.contains("fn cbrt"));
    }

    #[test]
    fn test_math_has_rounding_functions() {
        assert!(MATH_DEFINITION.contains("fn round"));
        assert!(MATH_DEFINITION.contains("fn floor"));
        assert!(MATH_DEFINITION.contains("fn ceil"));
        assert!(MATH_DEFINITION.contains("fn trunc"));
    }
}
