//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 02
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
//! - hw02 Functions
//!     - multiply_array
//!     - all_different
//!     - all_different_except_zeros
//!     - dot_product
//! - Test Functions
//!     - test_multiply_array_empty_array
//!     - test_multiply_array_zeros
//!     - test_multiply_array_ones
//!     - test_multiply_array_normal_behavior
//!     - test_all_different_empty_array
//!     - test_all_different_single_array
//!     - test_all_different_true
//!     - test_all_different_false
//!     - test_all_different_except_zeros_empty_array
//!     - test_all_different_except_zeros_single_array
//!     - test_all_different_except_zeros_true
//!     - test_all_different_except_zeros_false
//!     - test_dot_product_panic_array_length
//!     - test_dot_product_empty_arrays
//!     - test_dot_product_zeros
//!     - test_dot_product_ones
//!     - test_dot_product_normal_behavior
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// NA

// ----- Global Variables -------------------------------------------------------------------------
// NA

// ================================================================================================
// END File Overview, Imports, Global Variables
// START hw02 Functions
// ================================================================================================

/// About
/// -----
/// - Takes an Array of 32-bit unsigned integers
/// - Returns the result of multiplying all the array's entries together
///
/// Parameters
/// ----------
/// - arr: &[u32]
///     - An array of 32-bit unsigned integers to be multiplied together
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - u32
///     - 32-bit unsigned integer of values in 'arr' multiplied together
fn multiply_array(arr: &[u32]) -> u32 {

    // Catch edge-case
    if arr.len() == 0 {
        return 0;
    }

    // Process all other case
    let mut result: u32 = 1;
    for val in arr {
        result *= val;
    }

    result
}


/// About
/// -----
/// - Takes an Array of 32-bit unsigned integers
/// - Returns true if the numbers are all different, false otherwise
///
/// Parameters
/// ----------
/// - arr: &[u32]
///     - An array of 32-bit unsigned integers to be checked for all different values
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - bool
///     - true if all values of 'arr' are different
///     - false otherwise
fn all_different(arr: &[u32]) -> bool {

    // ===== Validate Trivial Case ==================================
    if arr.len() <= 1 {
        return true;
    }

    // ===== Main Loop Logic ========================================
    for i in 0..arr.len() {

        // Check for any matching values
        for j in (i+1)..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }

    // No matching values found
    true
}


/// About
/// -----
/// - Takes an Array of 32-bit unsigned integers
/// - Returns true if all non-zero entries are all different, false otherwise
///
/// Parameters
/// ----------
/// - arr: &[u32]
///     - An array of 32-bit unsigned integers to be checked for all different values
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - bool
///     - true if all values of 'arr' are different
///     - false otherwise
fn all_different_except_zeros(arr: &[u32]) -> bool {

    // ===== Validate Trival Case ===================================
    if arr.len() <= 1 {
        return true;
    }

    // ===== Main Loop Logic ========================================
    for i in 0..arr.len() {

        // Ignore the value 0
        if arr[i] == 0 { continue; }

        // Check for any matching values
        for j in (i+1)..arr.len() {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }

    // No matching values found
    true
}


/// About
/// -----
/// - Takes two Arrays of 32-bit unsigned integers
/// - Returns the dot product of the values (multiply corresponding array values and add these multiplications together)
/// - The function can either panic or produce 0 (your choice) if the Arrays are not equal length
/// - This implementation will panic if arrays are not of equal length
///
/// Parameters
/// ----------
/// - arr1: &[u32]
///     - First array of 32-bit unsigned integers to be used in dot product arithmetic
///
/// - arr2: &[u32]
///     - Second array of 32-bit unsigned integers to be used in dot product arithmetic
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - u32
///     - Unsigned variable for blah
fn dot_product(arr1: &[u32], arr2: &[u32]) -> u32 {
    let mut result: u32 = 0;

    // ===== Validate Equal Array Length ============================
    if arr1.len() != arr2.len() {
        panic!("Arrays are not the same length! (arr1 length {} vs. arr2 length {}", arr1.len(), arr2.len());
    }

    // ===== Main Loop Logic ========================================
    for i in 0..arr1.len() {
        result += arr1[i] * arr2[i];
    }

    result
}

// ================================================================================================
// END hw02 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test for multiply_array function
    /// - Simple test for an empty array
    #[test]
    fn test_multiply_array_empty_array() {
        assert_eq!(multiply_array(&[]), 0);
    }

    /// About
    /// -----
    /// - Test for multiply_array function
    /// - Simple all zeros test
    #[test]
    fn test_multiply_array_zeros() {
        assert_eq!(multiply_array(&[0, 0]), 0);
    }

    /// About
    /// -----
    /// - Test for multiply_array function
    /// - Simple all ones test
    #[test]
    fn test_multiply_array_ones() {
        assert_eq!(multiply_array(&[1, 1]), 1);
    }

    /// About
    /// -----
    /// - Test for multiply_array function
    /// - Simple test of a normal expected output
    #[test]
    fn test_multiply_array_normal_behavior() {
        assert_eq!(multiply_array(&[1, 2]), 2);
    }

    /// About
    /// -----
    /// - Test for all_different function
    /// - Simple test of an empty array
    #[test]
    fn test_all_different_empty_array() {
        assert_eq!(all_different(&[]), true);
    }

    /// About
    /// -----
    /// - Test for all_different function
    /// - Simple test of a single element array
    #[test]
    fn test_all_different_single_array() {
        assert_eq!(all_different(&[1]), true);
    }

    /// About
    /// -----
    /// - Test for all_different function
    /// - Simple test of a true output
    #[test]
    fn test_all_different_true() {
        assert_eq!(all_different(&[1, 2]), true);
    }

    /// About
    /// -----
    /// - Test for all_different function
    /// - Simple test of a false output
    #[test]
    fn test_all_different_false() {
        assert_eq!(all_different(&[1, 1]), false);
    }

    /// About
    /// -----
    /// - Test for all_different_except_zeros function
    /// - Simple test of an empty array
    #[test]
    fn test_all_different_except_zeros_empty_array() {
        assert_eq!(all_different_except_zeros(&[]), true);
    }

    /// About
    /// -----
    /// - Test for all_different_except_zeros function
    /// - Simple test of a single element array
    #[test]
    fn test_all_different_except_zeros_single_array() {
        assert_eq!(all_different_except_zeros(&[1]), true);
    }

    /// About
    /// -----
    /// - Test for all_different_except_zeros function
    /// - Simple test of a true output
    #[test]
    fn test_all_different_except_zeros_true() {
        assert_eq!(all_different_except_zeros(&[0, 0]), true);
    }

    /// About
    /// -----
    /// - Test for all_different_except_zeros function
    /// - Simple test of a false output
    #[test]
    fn test_all_different_except_zeros_false() {
        assert_eq!(all_different_except_zeros(&[1, 1]), false);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple test for panic flag on mismatched array length
    #[test]
    #[should_panic]
    fn test_dot_product_panic_array_length() {
        dot_product(&[1], &[1, 2]);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple test for empty arrays
    #[test]
    fn test_dot_product_empty_arrays() {
        assert_eq!(dot_product(&[], &[]), 0);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple all zeros test
    #[test]
    fn test_dot_product_zeros() {
        assert_eq!(dot_product(&[0, 0], &[0, 0]), 0);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple all ones test
    #[test]
    fn test_dot_product_ones() {
        assert_eq!(dot_product(&[1, 1], &[1, 1]), 2);
    }

    /// About
    /// -----
    /// - Test for dot_product function
    /// - Simple test of a normal expected input
    #[test]
    fn test_dot_product_normal_behavior() {
        assert_eq!(dot_product(&[1, 2], &[1, 2]), 5);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs the 4 functions to rapidly verify functionality
///     - multiply_array
///     - all_different
///     - all_different_except_zeros
///     - dot_product
fn main() {
    // Simple arrays to pass into functions
    let arr1: [u32; 3] = [1, 2, 3];
    let arr2: [u32; 3] = [1, 2, 0];

    println!("==================== Multiply Array ====================");
    println!("Array: {:?}", arr1);
    println!("Result: {}", multiply_array(&arr1));
    println!();

    println!("==================== All Different =====================");
    println!("Array: {:?}", arr1);
    println!("Result: {}", all_different(&arr1));
    println!();

    println!("============== All Different Except Zeros ==============");
    println!("Array: {:?}", arr2);
    println!("Result: {}", all_different_except_zeros(&arr2));
    println!();

    println!("===================== Dot Product ======================");
    println!("Array 1: {:?}", arr1);
    println!("Array 2: {:?}", arr2);
    println!("Result: {}", dot_product(&arr1, &arr2));
}

// ================================================================================================
// END Main Function
// ================================================================================================