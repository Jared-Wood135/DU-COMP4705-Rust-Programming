//! FILE OVERVIEW:
//! - COMP3705 Assignment 01
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
//! - Enumerations
//!     - Expr
//!         - Number
//!         - Negate
//!         - Add
//!         - Subtract
//!         - Multiply
//!         - Divide
//! - Primary Functions
//!     - evaluate
//! - Helper Functions
//!     - format_expr
//! - Test Functions
//!     - test_Expr_Number
//!     - test_Expr_Negate
//!     - test_Expr_Add
//!     - test_Expr_Subtract
//!     - test_Expr_Multiply
//!     - test_Expr_Divide
//!     - test_Expr_nested
//!     - test_format_expr
//!     - test_validate_op_tree_struct
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------


// ----- Global Variables -------------------------------------------------------------------------


// ================================================================================================
// END File Overview, Imports, Global Variables
// START Enumerations
// ================================================================================================

/// About
/// -----
/// - Expr represents a mathematical expression
/// - The enumeration matches the following variants:
///     - Number
///     - Negate
///     - Add
///     - Subtract
///     - Multiply
///     - Divide
#[derive(Debug)]
enum Expr {
    Number,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Expr {
    /// About
    /// -----
    /// - The literal number as type f64
    /// 
    /// Parameters
    /// ----------
    /// - val (f64)
    ///     - The number expression
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - The number expression
    fn Number(val: f64) -> f64 {
        val
    }

    /// About
    /// -----
    /// - The negation of an expression (e.g., -5)
    /// 
    /// Parameters
    /// ----------
    /// - val (Box<Expr>)
    ///     - The mathematical expression to be negated
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - Negation of the input parameter
    fn Negate(val: Box<Expr>) -> f64 {
        val * -1
    }

    /// About
    /// -----
    /// - The addition of two expressions
    /// 
    /// Parameters
    /// ----------
    /// - left (Box<Expr>)
    ///     - The left expression to add
    /// - right (Box<Expr)
    ///     - The right expression to add
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - The sum of the left and right expressions
    fn Add(left: Box<Expr>, right: Box<Expr>) —> f64 {
        left + right
    }

    /// About
    /// -----
    /// - The subtraction of two expressions
    /// 
    /// Parameters
    /// ----------
    /// - left (Box<Expr>)
    ///     - The left expression to subtract from based on the right
    /// - right (Box<Expr>)
    ///     - The right expression amount to subtract from the left
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - The difference of the left and right expressions
    fn Subtract(left: Box<Expr>, right: Box<Expr>) —> f64 {
        left - right
    }

    /// About
    /// -----
    /// - The multiplication of two expressions
    /// 
    /// Parameters
    /// ----------
    /// - left (Box<Expr>)
    ///     - The left expression to multiply with
    /// - right (Box<Expr>)
    ///     - The right expression to multiply with
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - The product of the left and right expressions
    fn Multiply(left: Box<Expr>, right: Box<Expr>) —> f64 {
        left * right
    }

    /// About
    /// -----
    /// - The division of two expressions
    /// 
    /// Parameters
    /// ----------
    /// - left (Box<Expr>)
    ///     - The left expression to represent the numerator of division
    /// - right (Box<Expr>)
    ///     - The right expression to represent the denominator of division
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - f64
    ///     - The quotient of the left and right expressions
    fn Divide(left: Box<Expr>, right: Box<Expr>) —> f64 {
        left / right
    }
}

// ================================================================================================
// END Enumerations
// START Primary Functions
// ================================================================================================

/// About
/// -----
/// - Takes a reference to an `Expr` and returns an f64
/// - The function uses a match expression to handle each variant:
///     - Number
///     - Negate
///     - Add
///     - Subtract
///     - Multiply
///     - Divide
///
/// Parameters
/// ----------
/// - expr (&Expr)
///     - The expression of value(s) to be evaluated
///
/// Panics
/// ------
/// - NA
///
/// Output
/// ------
/// - f64
///     - The evaluation from the passed in `Expr`
fn evaluate(expr: &Expr) -> f64 {
    match expr {
        // Return the number
        Expr::Number(n) => *n,

        // Evaluate the inner expression and negate the result
        Expr::Negate(expr) => expr * -1,

        // Evaluate both sides and add them
        Expr::Add(left, right) => {
            evaluate(left) + evaluate(right)
        }

        // Evaluate both sides and subtract
        Expr::Subtract(left, right) => {
            evaluate(left) - evaluate(right)
        }

        // Evaluate both sides and multiply
        Expr::Multiply(left, right) => {
            evaluate(left) * evaluate(right)
        }

        // Evaluate both sides and divide
        Expr::Divide(left, right) => {
            evaluate(left) / evaluate(right)
        }

    }
}

// ================================================================================================
// END Primary Functions
// START Helper Functions
// ================================================================================================

/// About
/// -----
/// - Takes a reference to an `Expr`` and returns a String containing a human-readable representation of the expression
/// - For example, Add(Number(1), Number(2)) might produce "(1 + 2)"
/// - This will be useful for debugging
/// 
/// Parameters
/// ----------
/// - expr (&Expr)
///     - The expression to reformat into a human-readable string
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - String
///     - The human-readable string of the passed in `expr`
fn format_expr(expr: &Expr) -> String {
    match expr {
        // Format Number
        Expr::Number(n) => format!("{}", n),

        // Format Negate
        Expr::Negate(inner) => {
            format!("(-{})", format_expr(inner))
        }

        // Format Add
        Expr::Add(l, r) => {
            format!("({} + {})",
            format_expr(l), format_expr(r))
        }

        // Format Subtract
        Expr::Subtract(l, r) => {
            format!("({} - {})",
            format_expr(l), format_expr(r))
        }

        // Format Multiply
        Expr::Multiply(l, r) => {
            format!("({} * {})",
            format_expr(l), format_expr(r))
        }

        // Format Divide
        Expr::Divide(l, r) => {
            format!("({} / {})",
            format_expr(l), format_expr(r))
        }
    }
}

// ================================================================================================
// END Helper Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test for Expr::Number
    #[test]
    fn test_Expr_Number() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for Expr::Negate
    #[test]
    fn test_Expr_Negate() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for Expr::Add
    #[test]
    fn test_Expr_Add() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for Expr::Subtract
    #[test]
    fn test_Expr_Subtract() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for Expr::Multiply
    #[test]
    fn test_Expr_Multiply() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for Expr::Divide
    #[test]
    fn test_Expr_Divide() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for nested expressions
    #[test]
    fn test_Expr_nested() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for format_expr
    #[test]
    fn test_format_expr() {
        assert_eq!(1, 1);
    }

    /// About
    /// -----
    /// - Test for validating operator tree structure
    #[test]
    fn test_validate_op_tree_struct() {
        assert_eq!(1, 1);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply prints off the desired assignment implementations for rapid validation
fn main() {
    println!("\n==================== Expr::Number ====================");
    
    println!("\n==================== Expr::Negate ====================");

    println!("\n==================== Expr::Add =======================");

    println!("\n==================== Expr::Subtract ==================");

    println!("\n==================== Expr::Multiply ==================");

    println!("\n==================== Expr::Divide ====================");

    println!("\n==================== Expr - Nested ===================");

    println!("\n==================== format_expr =====================");

    println!("\n==================== Op Tree Struct ==================");
}

// ================================================================================================
// END Main Function
// ================================================================================================