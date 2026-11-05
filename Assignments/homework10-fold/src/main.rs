//! FILE OVERVIEW:
//! - File for COMP3705 Homework 10
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
//! - Homework 10 Functions
//!     - smallest_word
//!     - any_smaller
//!     - first_some
//! - Test Functions
//!     - test_smallest_word_standard
//!     - test_smallest_word_ties
//!     - test_smallest_word_single
//!     - test_smallest_word_empty_string
//!     - test_any_smaller_true
//!     - test_any_smaller_false
//!     - test_any_smaller_empty
//!     - test_any_smaller_boundary
//!     - test_first_some_early
//!     - test_first_some_all_none
//!     - test_first_some_strings
//!     - test_first_some_empty
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// None

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START Homework 10 Functions
// ================================================================================================

/// About
/// -----
/// - Takes a Vector of &str and produces the first &str in the Vector which has the shortest length
/// - The function should use an iterator and the fold() function and should only be "one" line of code
///
/// Parameters
/// ----------
/// - words: Vec<&str>
///     - Vector of string slices
///
/// Panics
/// ------
/// - None
/// 
/// Output
/// ------
/// - &str
///     - The shortest string slice
fn smallest_word(words: Vec<&str>) -> &str {
    words.into_iter().fold(None, |acc: Option<&str>, w| Some(match acc { None => w, Some(curr) => if w.len() < curr.len() { w } else { curr } })).unwrap_or("")
}


/// About
/// -----
/// - Takes an unsigned 32-bit integer and a Vector of tuples (u32, u32) and produces true if any of the tuples have components which are both smaller then the specified integer and false otherwise
/// - The function should use an iterator and the fold() function and should only be "one" line of code
///
/// Parameters
/// ----------
/// - limit: u32
///     - The threshold integer
/// - tuples: Vec<(u32, u32)>
///     - Vector of integer pairs
///
/// Panics
/// ------
/// - None
/// 
/// Output
/// ------
/// - bool
///     - True if any pair has both elements smaller than the limit, false otherwise
fn any_smaller(limit: u32, tuples: Vec<(u32, u32)>) -> bool {
    tuples.into_iter().fold(false, |acc, (a, b)| acc || (a < limit && b < limit))
}


/// About
/// -----
/// - Takes a Vector of Option<T> and produces the first entry in the Vector which is a Some and produces None otherwise
/// - This function should use an iterator and the fold() function and should only be "one" line of code
///
/// Parameters
/// ----------
/// - vector: Vec<Option<T>>
///     - Vector of optional values
///
/// Output
/// ------
/// - Option<T>
///     - The first Some entry encountered or None
fn first_some<T>(vector: Vec<Option<T>>) -> Option<T> {
    vector.into_iter().fold(None, |acc, item| if acc.is_none() { item } else { acc })
}

// ================================================================================================
// END Homework 10 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_word_standard() {
        let words = vec!["apple", "cat", "banana", "dog", "elephant"];
        assert_eq!(smallest_word(words), "cat");
    }

    #[test]
    fn test_smallest_word_ties() {
        let words = vec!["sun", "sky", "cloud", "rain"];
        assert_eq!(smallest_word(words), "sun");
    }

    #[test]
    fn test_smallest_word_single() {
        let words = vec!["single"];
        assert_eq!(smallest_word(words), "single");
    }

    #[test]
    fn test_smallest_word_empty_string() {
        let words = vec!["hello", "", "world"];
        assert_eq!(smallest_word(words), "");
    }

    #[test]
    fn test_any_smaller_true() {
        let tuples = vec![(10, 20), (5, 4), (15, 15)];
        assert_eq!(any_smaller(10, tuples), true);
    }

    #[test]
    fn test_any_smaller_false() {
        let tuples = vec![(10, 20), (15, 15), (10, 5)];
        assert_eq!(any_smaller(5, tuples), false);
    }

    #[test]
    fn test_any_smaller_empty() {
        let tuples = vec![];
        assert_eq!(any_smaller(10, tuples), false);
    }

    #[test]
    fn test_any_smaller_boundary() {
        let tuples = vec![(10, 10), (9, 10)];
        assert_eq!(any_smaller(10, tuples), false);
    }

    #[test]
    fn test_first_some_early() {
        let vector = vec![None, Some(42), Some(100), None];
        assert_eq!(first_some(vector), Some(42));
    }

    #[test]
    fn test_first_some_all_none() {
        let vector: Vec<Option<i32>> = vec![None, None, None];
        assert_eq!(first_some(vector), None);
    }

    #[test]
    fn test_first_some_strings() {
        let vector = vec![None, None, Some("hello"), Some("world")];
        assert_eq!(first_some(vector), Some("hello"));
    }

    #[test]
    fn test_first_some_empty() {
        let vector: Vec<Option<i32>> = vec![];
        assert_eq!(first_some(vector), None);
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
    println!("\x1b[35m==================== smallest_word ====================\x1b[0m");
    let words = vec!["programming", "in", "rust"];
    println!("Input words: {:?}", words);
    println!("Smallest word: {}", smallest_word(words));
    println!();

    println!("\x1b[35m==================== any_smaller ======================\x1b[0m");
    let tuples = vec![(20, 30), (3, 4)];
    println!("Input values: {:?}", tuples);
    println!("Any smaller (limit 10): {}", any_smaller(10, tuples));
    println!();

    println!("\x1b[35m==================== first_some =======================\x1b[0m");
    let options = vec![None, Some("success"), None];
    println!("Input values: {:?}", options);
    println!("First some: {:?}", first_some(options));
}

// ================================================================================================
// END Main Function
// ================================================================================================