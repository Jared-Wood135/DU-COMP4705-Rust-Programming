//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 03
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
//! - hw03 Functions
//!     - count_occurences
//!     - square_elements
//!     - map_elements
//!     - count_true_and_false
//!     - subtract_one
//! - Test Functions
//!     - test_count_occurences
//!     - test_count_occurences_empty
//!     - test_square_elements
//!     - test_square_elements_empty
//!     - test_map_elements
//!     - test_map_elements_empty
//!     - test_count_true_and_false
//!     - test_count_true_and_false_empty
//!     - test_subtract_one
//!     - test_subtract_one_panic
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// None

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START hw03 Functions
// ================================================================================================

/// About
/// -----
/// - Takes a single character and an array of characters
/// - Returns the number of times the single character occurs in the array
/// 
/// Parameters
/// ----------
/// - target (char)
///     - Character to find in `arr`
/// - arr (&[char])
///     - Array of characters to search for `char`
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - Number of times `char` occurs in `arr`
fn count_occurences(
    target: char,
    arr: &[char]
) -> u32 {
    let mut count = 0;
    
    for &val in arr {
        if val == target {
            count += 1;
        }
    }

    count
}


/// About
/// -----
/// - Takes an array of 32-bit unsigned integers and modifies the array so that each element is the square of the original elements
/// - For example, if [1, 3, 5] is passed to square_elements, the array would be changed to [1, 9, 25]
/// 
/// Parameters
/// ----------
/// - arr (&mut [u32])
///     - Array of unsigned integers to be squared
///     - This array will be mutated and returned
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - arr (&mut [u32])
///     - Array of squared unsigned integers from `arr`
fn square_elements(arr: &mut [u32]) {
    for element in arr.iter_mut() {
        *element = *element * *element;
    }
}


/// About
/// -----
/// - Takes a function (which takes an unsigned 32-bit integer and produces a 32-bit unsigned integer) 
///   and an array of 32-bit unsigned integers and modifies the array so that each element is the function 
///   applied to each of the original elements
/// - For example, if a function to subtract 1 and the array [1, 3, 5] are passed to map_elements, 
///   the array would be changed to [0, 2, 4]
/// 
/// Parameters
/// ----------
/// - function (fn(u32) -> u32)
///     - Intended to be a function that performs a specific arithmetic on the passed u32 variable
///     - See the `subtract_one` function for reference
/// - arr (&mut [u32])
///     - Array of unsigned integers to be squared
///     - This array will be mutated and returned
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - arr (&mut [u32])
///     - Array of mapped unsigned integers in `arr`
fn map_elements(
    function: fn(u32) -> u32, 
    arr: &mut [u32]
) {
    for element in arr.iter_mut() {
        *element = function(*element);
    }
}


/// About
/// -----
/// - Takes an array of boolean values
/// - Produces a tuple whose first element is the number of true values in the array and the second element is the number of false values
/// 
/// Parameters
/// ----------
/// - arr (&[bool])
///     - Array of booleans to count T/F values
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - (u32, u32)
///     - (# of true in `&arr`, # of false in `&arr`)
fn count_true_and_false(arr: &[bool]) -> (u32, u32) {
    let mut trues = 0;
    let mut falses = 0;

    for &val in arr {
        if val {
            trues += 1;
        } 
        else {
            falses += 1;
        }
    }

    (trues, falses)
}


/// About
/// -----
/// - Takes a single unsigned variable and subtracts 1 and returns the difference
/// - x -1
/// 
/// Parameters
/// ----------
/// - x (u32)
///     - Value to subtract 1 from
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - Difference of x - 1
fn subtract_one(x: u32) -> u32 {
        x - 1
    }

// ================================================================================================
// END hw03 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurences() {
        let chars = ['a', 'b', 'c', 'a', 'a'];
        assert_eq!(count_occurences('a', &chars), 3);
    }

    #[test]
    fn test_count_occurences_empty() {
        let chars: [char; 0] = [];
        assert_eq!(count_occurences('a', &chars), 0);
    }

    #[test]
    fn test_square_elements() {
        let mut numbers = [1, 3, 5];
        square_elements(&mut numbers);
        assert_eq!(numbers, [1, 9, 25]);
    }

    #[test]
    fn test_square_elements_empty() {
        let mut numbers = [];
        square_elements(&mut numbers);
        assert_eq!(numbers, []);
    }

    #[test]
    fn test_map_elements() {
        let mut numbers = [1, 2, 3];
        map_elements(subtract_one, &mut numbers);
        assert_eq!(numbers, [0, 1, 2]);
    }

    #[test]
    fn test_map_elements_empty() {
        let mut numbers = [];
        map_elements(subtract_one, &mut numbers);
        assert_eq!(numbers, []);
    }

    #[test]
    fn test_count_true_and_false() {
        let bools = [true, false, true, true, false];
        assert_eq!(count_true_and_false(&bools), (3, 2));
    }

    #[test]
    fn test_count_true_and_false_empty() {
        let bools: [bool; 0] = [];
        assert_eq!(count_true_and_false(&bools), (0, 0));
    }

    #[test]
    fn test_subtract_one() {
        assert_eq!(subtract_one(1), 0);
    }

    #[test]
    #[should_panic]
    fn test_subtract_one_panic() {
        subtract_one(0);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs and prints the above functions for rapid validation of operability
///     - count_occurences
///     - square_elements
///     - map_elements
///     - count_true_and_false
fn main() {
    println!("================ Count Occurences ================");
    let char_arr = ['a', 'b', 'c', 'a', 'a'];
    let target = 'a';
    println!(
        "Occurrences of '{}' in {:?}: {}",
        target,
        char_arr,
        count_occurences(target, &char_arr)
    );
    println!();

    println!("================= Square Elements =================");
    let mut square_arr = [1, 3, 5];
    println!("Before: {:?}", square_arr);
    square_elements(&mut square_arr);
    println!("After:  {:?}", square_arr);
    println!();

    println!("================== Map Elements ===================");
    let mut map_arr = [1, 2, 3];
    println!("Before mapping:     {:?}", map_arr);
    map_elements(subtract_one, &mut map_arr);
    println!("After mapping (-1): {:?}", map_arr);
    println!();

    println!("================ Count True And False =============");
    let bool_arr = [true, false, true, true, false];
    let (trues, falses) = count_true_and_false(&bool_arr);
    println!("Array: {:?}", bool_arr);
    println!("True count: {}, False count: {}", trues, falses);
    println!();
}

// ================================================================================================
// END Main Function
// ================================================================================================