//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 06
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - None
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - hw06 Functions
//!     - add_one
//!     - get_total
//!     - sum_tuple
//! - Test Functions
//!     - test_add_one
//!     - test_get_total
//!     - test_sum_tuple
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// None

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START hw06 Functions
// ================================================================================================

/// About
/// -----
/// - Takes a mutable reference to a Vector of unsigned 32-bit integers
/// - The function should then add one to each element in the Vector (i.e., replace the value with the value + 1)
///
/// Parameters
/// ----------
/// - vec (&mut Vec<u32>)
///     - Mutable reference to the vector of unsigned 32-bit integers to add one to each element
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - &mut Vec<u32>
///     - `vec` with one added to each element
fn add_one(vec: &mut Vec<u32>) -> &mut Vec<u32> {
    for val in vec.iter_mut() {
        *val += 1;
    }
    vec
}


/// About
/// -----
/// - Takes a reference to a Vector of unsigned 32-bit integers
/// - The function should use an iterator and return the sum of the elements in the Vector
/// 
/// Parameters
/// ----------
/// - vec (&Vec<u32>)
///     - Reference to the vector of unsigned 32-bit integers to return the sum of all elements
/// 
/// Panics
/// ------
/// - None
/// 
/// Output
/// ------
/// - u32
///     - The sum of all elements from `vec`
fn get_total(vec: &Vec<u32>) -> u32 {
    vec.iter().sum()
}


/// About
/// -----
/// - Takes a Vector of (u32, u32) tuples and produces a new Vector of 32-bit integers
/// - This function should use an iterator and the map() function and should only be "one" line of code
/// 
/// Parameters
/// ----------
/// - vec (Vec<(u32, u32)>)
///     - Vector of unsigned 32-bit integer tuples to produce a new vector of 32-bit integers
/// 
/// Panics
/// ------
/// - None
/// 
/// Output
/// ------
/// - Vec<u32>
///     - Vector of the sum of `vec` tuples per element
fn sum_tuple(vec: Vec<(u32, u32)>) -> Vec<u32> {
    vec.into_iter().map(|(a, b)| a + b).collect()
}

// ================================================================================================
// END hw06 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test for add_one
    #[test]
    fn test_add_one() {
        let mut input = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![2, 3, 4, 5, 6, 7];
        add_one(&mut input);
        assert_eq!(input, expected);
    }

    /// About
    /// -----
    /// - Test for get_total
    #[test]
    fn test_get_total() {
        let input = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(get_total(&input), 21);
    }

    /// About
    /// -----
    /// - Test for sum_tuple
    #[test]
    fn test_sum_tuple() {
        let input = vec![(1, 2), (3, 4), (5, 6)];
        let expected = vec![3, 7, 11];
        assert_eq!(sum_tuple(input), expected);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply prints inputs and outputs of the three functions for rapid validation of functionality
///     - add_one()
///     - get_total()
///     - sum_tuple()
fn main() {
    println!("\n==================== add_one ======================");
    let mut vec1 = vec![1, 2, 3, 4, 5, 6];
    add_one(&mut vec1);
    println!("Before add_one: {:?}", vec1);
    println!("After add_one:  {:?}", vec1);

    println!("\n==================== get_total ====================");
    let vec2 = vec![1, 2, 3, 4, 5, 6];
    println!("Input vector: {:?}", vec2);
    println!("Total sum:    {}", get_total(&vec2));

    println!("\n==================== sum_tuple ====================");
    let vec3 = vec![(1, 2), (3, 4), (5, 6)];
    println!("Input tuple vector: {:?}", vec3);
    println!("Summed tuple vector: {:?}", sum_tuple(vec3));
}

// ================================================================================================
// END Main Function
// ================================================================================================