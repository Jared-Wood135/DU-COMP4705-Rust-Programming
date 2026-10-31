//! FILE OVERVIEW:
//! - File for COMP3705 Homework 09
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - Completed
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - Homework 09 Functions
//!     - smaller_than
//!     - get_values
//!     - only_some
//! - Test Functions
//!     - test_smaller_than_normal
//!     - test_smaller_than_empty_input
//!     - test_smaller_than_empty_output
//!     - test_smaller_than_all_less_than
//!     - test_get_values_normal
//!     - test_get_values_empty_output
//!     - test_get_values_empty_input
//!     - test_only_some_normal
//!     - test_only_some_all_none
//!     - test_only_some_all_some
//!     - test_only_some_empty_input
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// None

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START Homework 09 Functions
// ================================================================================================

/// About
/// -----
/// - Takes an unsigned 32-bit integer and a Vector of unsigned 32-bit integers and produces a new Vector of unsigned 32-bit integers
/// - The new Vector should contain only the values smaller than the given unsigned 32-bit integer
/// - The function should use an iterator and the filter() function and should only be "one" line of code
/// 
/// Parameters
/// ----------
/// - limit (u32)
///     - The unsigned 32-bit integer to filter through `v` and return a `Vector` of values who are strictly smaller than `limit`
/// - v (Vec<u32>)
///     - The `Vector` of unsigned 32-bit integers to be filtered through
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<u32>
///     - The `Vector` of unsigned 32-bit integers whose values are strictly less than `limit`
fn smaller_than(limit: u32, v: Vec<u32>) -> Vec<u32> {
    v.into_iter().filter(|&x| x < limit).collect()
}


/// About
/// -----
/// - Takes an unsigned 32-bit integer and a Vector of tuples (u32, &str) and produces a new Vector of &str
/// - The new Vector should contain only the &str which are in a tuple that matches the given unsigned 32-bit integer
/// - The function should use an iterator and the filter() function and should only be "one" line of code
/// 
/// Parameters
/// ----------
/// - key_filter (u32)
///     - The unsigned 32-bit integer key to filter through `v` and return a new `Vector` with only the strings whose key matches `key_filter`
/// - v (Vec<u32, &'a str>)
///     - The `Vector` of (key, str) pairs to be filtered through by `key_filter`
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<&'a str>
///     - The `Vector` of strings whose key pairs matched `key_filter`
fn get_values<'a>(key_filter: u32, v: Vec<(u32, &'a str)>) -> Vec<&'a str> {
    v.into_iter().filter(|(n, _)| *n == key_filter).map(|(_, s)| s).collect()
}


/// About
/// -----
/// - Takes a Vector of Option<T> and produces a new Vector of T
/// - The function should produce a new Vector which does not contain any None entries
/// - This function should use an iterator and the filter() function and should only be "one" line of code
/// 
/// Parameters
/// ----------
/// - v (Vec<Option<T>>)
///     - The `Vector` filled with Some() and None entries to filter out all the None entries
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<T>
///     - The `Vector` filled with only not None entries from `v`
fn only_some<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}

// ================================================================================================
// END Homework 09 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_than_normal() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let result = smaller_than(4, v);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_smaller_than_empty_input() {
        let empty_v: Vec<u32> = vec![];
        assert_eq!(smaller_than(5, empty_v), vec![]);
    }

    #[test]
    fn test_smaller_than_empty_output() {
        let high_v = vec![2, 3, 4, 5, 6];
        assert_eq!(smaller_than(1, high_v), vec![]);
    }

    #[test]
    fn test_smaller_than_all_less_than() {
        let low_v = vec![1, 2, 3];
        assert_eq!(smaller_than(5, low_v), vec![1, 2, 3]);
    }

    #[test]
    fn test_get_values_normal() {
        let v = vec![
            (1, "apple"),
            (2, "banana"),
            (1, "apricot"),
            (3, "cherry"),
        ];
        let result = get_values(1, v);
        assert_eq!(result, vec!["apple", "apricot"]);        
    }

    #[test]
    fn test_get_values_empty_output() {
        let v = vec![
            (2, "banana"),
            (3, "cherry"),
        ];
        assert_eq!(get_values(1, v), Vec::<&str>::new());
    }

    #[test]
    fn test_get_values_empty_input() {
        let v = vec![];
        assert_eq!(get_values(1, v), Vec::<&str>::new());
    }

    #[test]
    fn test_only_some_normal() {
        let v = vec![Some(1), None, Some(2), None, Some(3)];
        let result = only_some(v);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_only_some_all_none() {
        let all_none: Vec<Option<i32>> = vec![None, None];
        assert_eq!(only_some(all_none), Vec::<i32>::new());
    }

    #[test]
    fn test_only_some_all_some() {
        let all_some = vec![Some("a"), Some("b")];
        assert_eq!(only_some(all_some), vec!["a", "b"]);
    }

    #[test]
    fn test_only_some_empty_input() {
        let empty_opt: Vec<Option<u32>> = vec![];
        assert_eq!(only_some(empty_opt), Vec::<u32>::new());
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs the 3 above functions for rapid verification of functionality
fn main() {
    println!("\x1b[35m==================== smaller_than ====================\x1b[0m");
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let limit = 4;
    let filtered_numbers = smaller_than(limit, numbers.clone());
    println!("Limit:\t\t\t{}", limit);
    println!("Original vector:\t{:?}", numbers);
    println!("New vector:\t\t{:?}", filtered_numbers);

    println!("\n\x1b[35m==================== get_values ======================\x1b[0m");
    let pairs = vec![
        (1, "YAY!"),
        (2, "Avoid"),
        (1, "Hidden"),
        (3, "Me"),
        (1, "Messages!"),
    ];
    let key = 1;
    let matched_strings = get_values(key, pairs.clone());
    println!("Key filter:\t\t{}", key);
    println!("Original tuples:\t{:?}", pairs);
    println!("New tuples:\t\t{:?}", matched_strings);

    println!("\n\x1b[35m==================== only_some =======================\x1b[0m");
    let options = vec![Some(1), None, Some(2), Some(3), None, Some(4)];
    let unwrapped_options = only_some(options.clone());
    println!("Original options:\t{:?}", options);
    println!("New options:\t\t{:?}", unwrapped_options);
}

// ================================================================================================
// END Main Function
// ================================================================================================