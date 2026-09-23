//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 05
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - 
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - hw05 Functions
//!     - read_program_file
//!     - is_keyword
//!     - split_string
//! - Test Functions
//!     - test_read_program_file
//!     - test_is_keyword
//!     - test_split_string
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
use std::fs;

// ----- Global Variables -------------------------------------------------------------------------
// NA

// ================================================================================================
// END File Overview, Imports, Global Variables
// START hw05 Functions
// ================================================================================================

/// About
/// -----
/// - Takes a `&str` parameter representing a filename and produces a Vector of String
/// - Each element of the Vector should contain one of the lines from the file
///
/// Parameters
/// ----------
/// - filename: &str
///     - Name of the file to read from
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - Vec<String>
///     - Vector where each element is a single line from `filename`
fn read_program_file(filename: &str) -> Vec<String> {
    // Instantiate return variable
    let mut lines: Vec<String> = Vec::new();

    // Try reading the file
    let contents = fs::read_to_string(&filename)
        .expect("could not read file");
        
    // Add lines to Vector
    for line in contents.lines() {
        lines.push(line.trim().to_string());
    }

    // Return Vector
    lines
}


/// About
/// -----
/// - Takes a `&str` parameter and determines if the parameter is one of the following words: 
///     - and
///     - class
///     - else
///     - false
///     - for
///     - fun
///     - if
///     - nil
///     - or
///     - print
///     - return
///     - super
///     - this
///     - true
///     - var
///     - while
///
/// Parameters
/// ----------
/// - word: &str
///     - String to check if it's one of the keywords
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - bool
///     - True if the word is one of the keywords
fn is_keyword(word: &str) -> bool {
    match word {
        "and"    | 
        "class"  | 
        "else"   | 
        "false"  | 
        "for"    | 
        "fun"    | 
        "if"     | 
        "nil"    | 
        "or"     | 
        "print"  | 
        "return" | 
        "super"  | 
        "this"   | 
        "true"   | 
        "var"    | 
        "while"  => true, 
        _ => false,
    }
}


/// About
/// -----
/// - Takes a `&str` parameter and produces a Vector of String
/// - The Vector should contain the individual words when the parameter is divided by whitespace characters
/// - The whitespace characters should not be included as words in the result
///
/// Parameters
/// ----------
/// - line: &str
///     - Line that is split by whitespace to be returned as a Vector with each word as an element
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - Vec<String>
///     - Vector of `line` where each element is an individual word
fn split_string(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// ================================================================================================
// END hw05 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test for read_program_file
    #[test]
    fn test_read_program_file() {
        // Expected line return
        let line1: String = "This is a test file".to_string();
        let line2: String = "This is the second line of the test file".to_string();

        // Actual line return
        let lines: Vec<String> = read_program_file("src/some_file.txt");

        // Compare expected and actual
        assert_eq!(lines[0], line1);
        assert_eq!(lines[1], line2);
    }

    /// About
    /// -----
    /// - Test for is_keyword
    #[test]
    fn test_is_keyword() {
        // Make some test keywords
        let keywords = ["and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return", "super", "this", "true", "var", "while"];
        let non_keywords = ["These", " ", "are", " ", "not", " ", "keywords", "."];

        // Ensure all of these are true
        for &word in &keywords {
            assert_eq!(is_keyword(word), true);
        }

        // Ensure all of these are false
        for &word in &non_keywords {
            assert_eq!(is_keyword(word), false);
        }
    }

    /// About
    /// -----
    /// - Test for split_string
    #[test]
    fn test_split_string() {
        // Make some test strings
        let words: String = "Split me!".to_string();
        let spec_chars: String = "- = + _".to_string();
        let nums: String = "1 2 3 4".to_string();
        let wrapped_words: String = " Starts and ends with whitespace".to_string();
        let empty: String = "     ".to_string();

        // Compare test strings to function returns
        assert_eq!(split_string(&words), vec!["Split", "me!"]);
        assert_eq!(split_string(&spec_chars), vec!["-", "=", "+", "_"]);
        assert_eq!(split_string(&nums), vec!["1", "2", "3", "4"]);
        assert_eq!(split_string(&wrapped_words), vec!["Starts", "and", "ends", "with", "whitespace"]);
        assert_eq!(split_string(&empty), Vec::<String>::new());
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply prints inputs and outputs of the three functions for rapid validation of functionality
///     - read_program_file()
///     - is_keyword()
///     - split_string()
fn main() {
    println!("\n==================== Reading File ====================");
    println!("Input File: src/some_file.txt");
    println!("1st Line: This is a test file");
    println!("2nd Line: This is the second line of the test file");
    println!("Actual Output: {:?}\n", read_program_file("src/some_file.txt"));

    println!("\n==================== Is Keyword ======================");
    println!("True Keyword: {}\nFunction Return: {}", "class", is_keyword("class"));
    println!("False Keyword: {}\nFunction Return: {}\n", "I am not a keyword, but rather an inane sentence!", is_keyword("I am not a keyword, but rather an inane sentence!"));

    println!("\n==================== Split String ====================");
    println!("Input String: 'Split me!'");
    println!("Output: {:?}", split_string("Split me!"));
}

// ================================================================================================
// END Main Function
// ================================================================================================