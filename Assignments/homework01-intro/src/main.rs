//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 01
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - NA
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - hw01 Functions
//!     - quadratic
//!     - scale_vector
//!     - dot_product
//!     - hadamard_product
//! - Test Functions
//!     - test_quadratic_ones
//!     - test_quadratic_zeros
//!     - test_quadratic_normal_behavior
//!     - test_scale_vector_ones
//!     - test_scale_vector_zeros
//!     - test_scale_vector_negative
//!     - test_scale_vector_normal_behavior
//!     - test_dot_product_ones
//!     - test_dot_product_zeros
//!     - test_dot_product_negative
//!     - test_dot_product_normal_behavior
//!     - test_hadamard_product_ones
//!     - test_hadamard_product_zeros
//!     - test_hadamard_product_negative
//!     - test_hadamard_product_normal_behavior
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// NA

// ----- Global Variables -------------------------------------------------------------------------
// NA

// ===============================================================================================
// END File Overview, Imports, Global Variables
// START hw01 Functions
// ===============================================================================================

/// About
/// -----
/// - Takes four unsigned integer parameters: a, b, c, and x
/// - The function returns the value of a + b*x + c*x^2
/// - Function input and output are printed for active confirmation of expectations
/// 
/// Parameters
/// ----------
/// - a: u32
///     - Unsigned integer for variable a
/// - b: u32
///     - Unsigned integer for variable b
/// - c: u32
///     - Unsigned integer for variable c
/// - x: u32
///     - Unsignd integer for variable x
/// 
/// Exceptions
/// ----------
/// - NA
/// 
/// Output
/// ------
/// - u32
///     - Unsigned integer after quadratic formula conducted (a + b*x + c*x^2)
fn quadratic(a: u32, b: u32, c: u32, x: u32) -> u32 {
    println!("quadratic input: a={}, b={}, c={}, x={}", a, b, c, x);
    let result: u32 = a + b*x + c*x.pow(2);
    println!("quadratic result: {}", result);
    result
}


/// About
/// -----
/// - Takes a single floating point number along with a 2-tuple which represents a two-dimensional vector
/// - The function returns a 2-tuple which represents the vector scaled by the value
/// - For example, scale_vector of 5 and (3, 4) produces (15, 20)
/// - Function input and output are printed for active confirmation of expectations
/// 
/// Parameters
/// ----------
/// - scalar: f32
///     - Floating point value of the scalar to be applied to vector
/// -  vector: (f32, f32)
///     - A tuple of two floating point values to be scaled
/// 
/// Exceptions
/// ----------
/// - NA
/// 
/// Output
/// ------
/// - (f32, f32)
///     - A tuple of two floating point values after scaling is performed (scalar * vector)
fn scale_vector(scalar: f32, vector: (f32, f32)) -> (f32, f32) {
    println!("scale_vector input: scalar={}, vector={:?}", scalar, vector);
    let result: (f32, f32) = (scalar * vector.0 as f32, scalar * vector.1 as f32);
    println!("scale_vector result: {:?}", result);
    result
}


/// About
/// -----
/// - Takes two 2-tuples which represent two-dimensional vectors and calculates the dot product of the vectors
/// - Function input and output are printed for active confirmation of expectations
/// 
/// Parameters
/// ----------
/// - vector1: (f32, f32)
///     - A tuple of two floating point values representing the first vector to use in dot product arithmetic
/// - vector2: (f32, f32)
///     - A tuple of two floating point values representing the second vector to  use in dot product arithmetic
/// 
/// Exceptions
/// ----------
/// - NA
/// 
/// Output
/// ------
/// - f32
///     - A floating point value representing the dot product of vector1 and vector2
fn dot_product(vector1: (f32, f32), vector2: (f32, f32)) -> f32 {
    println!("dot_product input: vector1={:?}, vector2{:?}", vector1, vector2);
    let result: f32 = (vector1.0 * vector2.0) + (vector1.1 * vector2.1);
    println!("dot_product result: {}", result);
    result
}


/// About
/// -----
/// - Takes two 2-tuples which represent two-dimensional vectors and calculates the Hadamard product of the vectors
/// - Function input and output are printed for active confirmation of expectations
/// 
/// Parameters
/// ----------
/// - vector1: (f32, f32)
///     - A tuple of two floating point values representing the first vector to use in hadamard product arithmetic
/// - vector2: (f32, f32)
///     - A tuple of two floating point values representing the second vector to use in hadamard product arithmetic
/// 
/// Exceptions
/// ----------
/// - NA
/// 
/// Output
/// ------
/// - (f32, f32)
///     - A tuple of two floating point values representing the hadamard product of vector1 and vector2
fn hadamard_product(vector1: (f32, f32), vector2: (f32, f32)) -> (f32, f32) {
    println!("hadamard_product input: vector1={:?}, vector2={:?}", vector1, vector2);
    let result: (f32, f32) = ((vector1.0 * vector2.0 as f32), (vector1.1 * vector2.1 as f32));
    println!("hadamard_product result: {:?}", result);
    result
}

// ===============================================================================================
// END hw01 Functions
// START Test Functions
// ===============================================================================================

#[cfg(test)]
mod tests{
    use super::*;

    /// About
    /// -----
    /// - Test for quadratic function
    /// - Simple all ones test
    #[test]
    fn test_quadratic_ones() {
        assert_eq!(quadratic(1, 1, 1, 1), 3);
    }

    /// About
    /// -----
    /// - Test for quadratic function
    /// - Simple all zeros test
    #[test]
    fn test_quadratic_zeros() {
        assert_eq!(quadratic(0, 0, 0, 0), 0);
    }

    /// About
    /// -----
    /// - Test for quadratic function
    /// - Simple test of a normal expected input
    #[test]
    fn test_quadratic_normal_behavior() {
        assert_eq!(quadratic(4, 2, 2, 5), 64);
    }

    /// About
    /// -----
    /// - Test for scale_vector function
    /// - Simple all ones test
    #[test]
    fn test_scale_vector_ones() {
        assert_eq!(scale_vector(1.0, (1.0, 1.0)), (1.0, 1.0));
    }

    /// About
    /// -----
    /// - Test for scale_vector function
    /// - Simple all zeros test
    #[test]
    fn test_scale_vector_zeros() {
        assert_eq!(scale_vector(0.0, (1.0, 1.0)), (0.0, 0.0));
    }

    /// About
    /// -----
    /// - Test for scale_vector function
    /// - Simple test with a negative output
    #[test]
    fn test_scale_vector_negative() {
        assert_eq!(scale_vector(-1.0, (1.0, 1.0)), (-1.0, -1.0));
    }
    /// About
    /// -----
    /// - Test for scale_vector function
    /// - Simple test of a normal expected input
    #[test]
    fn test_scale_vector_normal_behavior() {
        assert_eq!(scale_vector(2.0, (2.0, 2.0)), (4.0, 4.0));
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple all ones test
    #[test]
    fn test_dot_product_ones() {
        assert_eq!(dot_product((1.0, 1.0), (1.0, 1.0)), 2.0);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple all zeros test
    #[test]
    fn test_dot_product_zeros() {
        assert_eq!(dot_product((0.0, 0.0), (0.0, 0.0)), 0.0);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple test with a negative output
    #[test]
    fn test_dot_product_negative() {
        assert_eq!(dot_product((-1.0, -1.0), (1.0, 1.0)), -2.0);
    }
    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple test of a normal expected input
    #[test]
    fn test_dot_product_normal_behavior() {
        assert_eq!(dot_product((1.0, 1.0), (2.0, 2.0)), 4.0);
    }

    /// About
    /// -----
    /// - Test for hadamard_product function
    /// - Simple all ones test
    #[test]
    fn test_hadamard_product_ones() {
        assert_eq!(hadamard_product((1.0, 1.0), (1.0, 1.0)), (1.0, 1.0));
    }

    /// About
    /// -----
    /// - Test for hadamard_product function
    /// - Simple all zeros test
    #[test]
    fn test_hadamard_product_zeros() {
        assert_eq!(hadamard_product((0.0, 0.0), (0.0, 0.0)), (0.0, 0.0));
    }

    /// About
    /// -----
    /// - Test for hadamard_product function
    /// - Simple test with a negative output
    #[test]
    fn test_hadamard_product_negative() {
        assert_eq!(hadamard_product((-1.0, -1.0), (1.0, 1.0)), (-1.0, -1.0));
    }
    /// About
    /// -----
    /// - Test for hadamard_product function
    /// - Simple test of a normal expected input
    #[test]
    fn test_hadamard_product_normal_behavior() {
        assert_eq!(hadamard_product((1.0, 1.0), (2.0, 2.0)), (2.0, 2.0));
    }
}
// ===============================================================================================
// END Test Functions
// START Main Function
// ===============================================================================================

/// About
/// -----
/// - Simply runs the 4 functions to rapidly verify functionality
///     - quadratic
///     - scale_vector
///     - dot_product
///     - hadamard_product
fn main() {
    println!("==================== Quadratic ====================");
    quadratic(4, 2, 2, 5);
    println!();

    println!("================== Scale Vector ===================");
    scale_vector(2.0, (2.0, 2.0));
    println!();

    println!("=================== Dot Product ===================");
    dot_product((1.0,1.0), (2.0,2.0));
    println!();

    println!("================ Hadamard Product =================");
    hadamard_product((2.0,2.0), (3.0,3.0));
}

// ===============================================================================================
// END Main Function
// ===============================================================================================