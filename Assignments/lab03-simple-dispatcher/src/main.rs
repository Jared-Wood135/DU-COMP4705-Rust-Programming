//! FILE OVERVIEW:
//! - COMP3705 Lab 03
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
//! - lab03 Functions
//!     - add
//!     - subtract
//!     - multiply
//!     - divide
//!     - build_dispatch_table
//!     - evaluate
//! - Test Functions
//!     - test_evaluate_add
//!     - test_evaluate_subtract
//!     - test_evaluate_multiply
//!     - test_evaluate_divide
//!     - test_evaluate_panic_invalid_operator
//!     - test_evaluate_panic_divide_by_zero
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// None

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START lab03 Functions
// ================================================================================================

/// About
/// -----
/// - Performs addition arithmetic of two unsigned variables
/// - a + b = sum
/// 
/// Parameters
/// ----------
/// - a (u32)
///     - Unsigned first variable for addition
/// - b (u32)
///     - Unsigned second variable for addition
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - a + b
fn add(a: u32, b: u32) -> u32 {
    a + b
}


/// About
/// -----
/// - Performs subtraction arithmetic of two unsigned variables
/// - a - b = difference
/// 
/// Parameters
/// ----------
/// - a (u32)
///     - Unsigned first variable for subtraction
/// - b (u32)
///     - Unsigned second variable for subtraction
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - a - b
fn subtract(a: u32, b: u32) -> u32 {
    a - b
}


/// About
/// -----
/// - Performs multiplication arithmetic of two unsigned variables
/// - a * b = product
/// 
/// Parameters
/// ----------
/// - a (u32)
///     - Unsigned first variable for multiplication
/// - b (u32)
///     - Unsigned second variable for multiplication
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - a * b
fn multiply(a: u32, b: u32) -> u32 {
    a * b
}


/// About
/// -----
/// - Performs division arithmetic of two unsigned variables
/// - a / b = quotient
/// 
/// Parameters
/// ----------
/// - a (u32)
///     - Unsigned first variable for division
/// - b (u32)
///     - Unsigned second variable for division
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - u32
///     - a / b
fn divide(a: u32, b: u32) -> u32 {
    a / b
}


/// About
/// -----
/// - Maintains a table of valid arithmetic operations mapped to function pointers
/// 
/// Parameters
/// ----------
/// - None
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - [(char, fn(u32, u32) -> u32); 4]
///     - List of valid arithmetic operators and their respective function names
fn build_dispatch_table() -> [(char, fn(u32, u32) -> u32); 4] {
    [
        ('+', add),
        ('-', subtract),
        ('*', multiply),
        ('/', divide),
    ]
}


/// About
/// -----
/// - Takes an arithmetic operator, two unsigned variables, and an operation lookup table
/// - Performs the corresponding arithmetic function if the operator exists in the table
///
/// Parameters
/// ----------
/// - operator: char
///     - The arithmetic operator character ('+', '-', '*', '/')
/// - a: u32
///     - Unsigned variable of the first operand
/// - b: u32
///     - Unsigned variable of the second operand
/// - arithmetic_table: &[(char, fn(u32, u32) -> u32)]
///     - Slice referencing pairs of operators and function pointers
///
/// Panics
/// ------
/// - Panics if the operator is not found in `arithmetic_table`
/// - Panics on division by zero or underflow in subtraction
///
/// Output
/// ------
/// - u32
///     - Result of executing the matching operator function on `a` and `b`
fn evaluate(
    operator: char, 
    a: u32, 
    b: u32, 
    arithmetic_table: &[(char, fn(u32, u32) -> u32)]
) -> u32 {
    for &(key, func) in arithmetic_table {
        if key == operator {
            return func(a, b);
        }
    }
    panic!("Unsupported operator: {}", operator);
}

// ================================================================================================
// END lab03 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test the evaluate function with an addition operator
    #[test]
    fn test_evaluate_add() {
        let table = build_dispatch_table();
        assert_eq!(evaluate('+', 3, 4, &table), 7);
    }

    /// About
    /// -----
    /// - Test the evaluate function with a subtraction operator
    #[test]
    fn test_evaluate_subtract() {
        let table = build_dispatch_table();
        assert_eq!(evaluate('-', 10, 4, &table), 6);
    }

    /// About
    /// -----
    /// - Test the evaluate function with a multiplication operator
    #[test]
    fn test_evaluate_multiply() {
        let table = build_dispatch_table();
        assert_eq!(evaluate('*', 3, 4, &table), 12);
    }

    /// About
    /// -----
    /// - Test the evaluate function with a division operator
    #[test]
    fn test_evaluate_divide() {
        let table = build_dispatch_table();
        assert_eq!(evaluate('/', 12, 4, &table), 3);
    }

    /// About
    /// -----
    /// - Test the evaluate function's panic on invalid arithmetic operator
    #[test]
    #[should_panic]
    fn test_evaluate_panic_invalid_operator() {
        let table = build_dispatch_table();
        evaluate('%', 3, 4, &table);
    }

    /// About
    /// -----
    /// - Test the evaluate function's panic on divide by zero scenario
    #[test]
    #[should_panic]
    fn test_evaluate_panic_divide_by_zero() {
        let table = build_dispatch_table();
        evaluate('/', 5, 0, &table);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs and prints off the desired arithmetic functions via the evaluate function:
///     - add
///     - subtract
///     - multiply
///     - divide
fn main() {
    let table = build_dispatch_table();
    println!("==================== Evaluate Add ====================");
    println!("10 + 5 = {}", evaluate('+', 10, 5, &table));
    println!();

    println!("================== Evaluate Subtract =================");
    println!("10 - 5 = {}", evaluate('-', 10, 5, &table));
    println!();

    println!("================== Evaluate Multiply =================");
    println!("10 * 5 = {}", evaluate('*', 10, 5, &table));
    println!();

    println!("=================== Evaluate Divide ==================");
    println!("10 / 5 = {}", evaluate('/', 10, 5, &table));
    println!();
}

// ================================================================================================
// END Main Function
// ================================================================================================