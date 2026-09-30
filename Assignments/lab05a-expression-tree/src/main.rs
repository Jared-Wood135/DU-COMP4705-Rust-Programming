//! FILE OVERVIEW:
//! - COMP3705 Lab 05a
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
//!     - impl fmt::Display
//! - lab05a Functions
//!     - count_nodes
//!     - depth
//!     - collect_numbers
//!     - to_rpn
//! - Test Functions
//!     - test_expr_number
//!     - test_expr_negate
//!     - test_expr_add
//!     - test_expr_subtract
//!     - test_expr_multiply
//!     - test_expr_divide
//!     - test_expr_nested
//!     - test_format_expr
//!     - test_validate_op_tree_struct
//!     - test_count_nodes
//!     - test_depth
//!     - test_collect_numbers
//!     - test_to_rpn
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
use std::fmt;

// ----- Global Variables -------------------------------------------------------------------------
// None

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
#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
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
    /// - Expr
    ///     - The number expression variant
    fn number(val: f64) -> Expr {
        Expr::Number(val)
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
    /// - Expr
    ///     - Negation variant of the input expression
    fn negate(val: Box<Expr>) -> Expr {
        Expr::Negate(val)
    }

    /// About
    /// -----
    /// - The addition of two expressions
    /// 
    /// Parameters
    /// ----------
    /// - left (Box<Expr>)
    ///     - The left expression to add
    /// - right (Box<Expr>)
    ///     - The right expression to add
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The sum variant of the left and right expressions
    fn add(left: Box<Expr>, right: Box<Expr>) -> Expr {
        Expr::Add(left, right)
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
    /// - Expr
    ///     - The difference variant of the left and right expressions
    fn subtract(left: Box<Expr>, right: Box<Expr>) -> Expr {
        Expr::Subtract(left, right)
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
    /// - Expr
    ///     - The product variant of the left and right expressions
    fn multiply(left: Box<Expr>, right: Box<Expr>) -> Expr {
        Expr::Multiply(left, right)
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
    /// - Expr
    ///     - The quotient variant of the left and right expressions
    fn divide(left: Box<Expr>, right: Box<Expr>) -> Expr {
        Expr::Divide(left, right)
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
        Expr::Negate(inner) => -evaluate(inner),

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
/// - Takes a reference to an `Expr` and returns a String containing a human-readable representation of the expression
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
            format!("({} + {})", format_expr(l), format_expr(r))
        }

        // Format Subtract
        Expr::Subtract(l, r) => {
            format!("({} - {})", format_expr(l), format_expr(r))
        }

        // Format Multiply
        Expr::Multiply(l, r) => {
            format!("({} * {})", format_expr(l), format_expr(r))
        }

        // Format Divide
        Expr::Divide(l, r) => {
            format!("({} / {})", format_expr(l), format_expr(r))
        }
    }
}


/// About
/// -----
/// - Trait for `Expr` so that println!("{}", expr) uses `format_expr` function
/// 
/// Parameters
/// ----------
/// - f (&mut fmt::Formatter<'_>)
///     - Original expression
/// 
/// Returns
/// -------
/// - fmt::Result
///     - Formatted expression
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_expr(self))
    }
}

// ================================================================================================
// END Helper Functions
// START Lab05a Functions
// ================================================================================================

/// About
/// -----
/// - Returns the total number of nodes in an expression tree
/// - A Number is one node
/// - A Negate has one node plus the count of its child
/// - A binary operation like Add has one node plus the counts of both children
/// 
/// Parameters
/// ----------
/// - expr (&Expr)
///     - Expression to count how many nodes exist
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - usize
///     - Total number of nodes in `expr`
fn count_nodes(expr: &Expr) -> usize {
    match expr {
        Expr::Number(_) => 1,
        Expr::Negate(inner) => 1 + count_nodes(inner),
                                   Expr::Add(l, r) | 
                                   Expr::Subtract(l, r) | 
                                   Expr::Multiply(l, r) | 
                                   Expr::Divide(l, r) => {
                                       1 + count_nodes(l) + count_nodes(r)
                                   }
    }
}


/// About
/// -----
/// - That returns the depth of the deepest node in the tree
/// - A Number has depth 1
/// - For a unary or binary operation, the depth is 1 plus the maximum depth of its children
/// 
/// Parameters
/// ----------
/// - expr (&Expr)
///     - Expression to determine how deep the expression goes
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - usize
///     - Depth of `expr`
fn depth(expr: &Expr) -> usize {
    match expr {
        Expr::Number(_) => 1,
        Expr::Negate(inner) => 1 + depth(inner),
                                   Expr::Add(l, r) | 
                                   Expr::Subtract(l, r) | 
                                   Expr::Multiply(l, r) | 
                                   Expr::Divide(l, r) => {
                                       1 + std::cmp::max(depth(l), depth(r))
                                   }
    }
}


/// About
/// -----
/// - Returns a vector of all the numeric values in the expression, in the order they appear from left to right
/// 
/// Parameters
/// ----------
/// - expr (&Expr)
///     - Expression to count how many numbers exist from
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<f64>
///     - A Vector object with all numeric values from `expr`
fn collect_numbers(expr: &Expr) -> Vec<f64> {
    let mut nums = Vec::new();
    match expr {
        Expr::Number(n) => nums.push(*n),
        Expr::Negate(inner) => nums.extend(collect_numbers(inner)),
                               Expr::Add(l, r) | 
                               Expr::Subtract(l, r) | 
                               Expr::Multiply(l, r) | 
                               Expr::Divide(l, r) => {
                                   nums.extend(collect_numbers(l));
                                   nums.extend(collect_numbers(r));
                               }
    }
    nums
}


/// About
/// -----
/// - Converts an expression tree into Reverse Polish Notation (RPN), also called postfix notation
/// - In RPN, the operator comes after its operands
/// - This is the notation used by HP calculators and the Forth programming language
/// 
/// Parameters
/// ----------
/// - expr (&Expr)
///     - The expression to convert into RPN
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - String
///     - The converted RPN from `expr`
fn to_rpn(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => format!("{}", n),
        Expr::Negate(inner) => format!("{} NEG", to_rpn(inner)),
        Expr::Add(l, r) => format!("{} {} +", to_rpn(l), to_rpn(r)),
        Expr::Subtract(l, r) => format!("{} {} -", to_rpn(l), to_rpn(r)),
        Expr::Multiply(l, r) => format!("{} {} *", to_rpn(l), to_rpn(r)),
        Expr::Divide(l, r) => format!("{} {} /", to_rpn(l), to_rpn(r)),
    }
}

// ================================================================================================
// END Lab05a Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test for Expr::Number
    #[test]
    fn test_expr_number() {
        let e = Expr::Number(42.0);
        assert_eq!(evaluate(&e), 42.0);
        let x = Expr::Number(-1.0);
        assert_eq!(evaluate(&x), -1.0);
        let p = Expr::Number(-123456789.123456789);
        assert_eq!(evaluate(&p), -123456789.123456789);
        let r = Expr::Number(0.0);
        assert_eq!(evaluate(&r), 0.0);
    }

    /// About
    /// -----
    /// - Test for Expr::Negate
    #[test]
    fn test_expr_negate() {
        let e = Expr::Negate(Box::new(Expr::Number(42.0)));
        assert_eq!(evaluate(&e), -42.0);
        let x = Expr::Negate(Box::new(Expr::Number(-1.0)));
        assert_eq!(evaluate(&x), 1.0);
        let p = Expr::Negate(Box::new(Expr::Number(-123456789.123456789)));
        assert_eq!(evaluate(&p), 123456789.123456789);
        let r = Expr::Negate(Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), 0.0);
    }

    /// About
    /// -----
    /// - Test for Expr::Add
    #[test]
    fn test_expr_add() {
        let e = Expr::Add(Box::new(Expr::Number(40.0)), Box::new(Expr::Number(2.0)));
        assert_eq!(evaluate(&e), 42.0);
        let x = Expr::Add(Box::new(Expr::Number(-1.0)), Box::new(Expr::Number(-1.0)));
        assert_eq!(evaluate(&x), -2.0);
        let p = Expr::Add(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(123456789.123456789)));
        assert_eq!(evaluate(&p), 0.0);
        let r = Expr::Add(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), 0.0);
    }

    /// About
    /// -----
    /// - Test for Expr::Subtract
    #[test]
    fn test_expr_subtract() {
        let e = Expr::Subtract(Box::new(Expr::Number(50.0)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&e), 42.0);
        let x = Expr::Subtract(Box::new(Expr::Number(-10.0)), Box::new(Expr::Number(10.0)));
        assert_eq!(evaluate(&x), -20.0);
        let p = Expr::Subtract(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(-123456789.123456789)));
        assert_eq!(evaluate(&p), 0.0);
        let r = Expr::Subtract(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), 0.0);
    }

    /// About
    /// -----
    /// - Test for Expr::Multiply
    #[test]
    fn test_expr_multiply() {
        let e = Expr::Multiply(Box::new(Expr::Number(5.25)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&e), 42.0);
        let x = Expr::Multiply(Box::new(Expr::Number(-5.25)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&x), -42.0);
        let p = Expr::Multiply(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(1.0)));
        assert_eq!(evaluate(&p), -123456789.123456789);
        let r = Expr::Multiply(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), 0.0)
    }

    /// About
    /// -----
    /// - Test for Expr::Divide
    #[test]
    fn test_expr_divide() {
        let e = Expr::Divide(Box::new(Expr::Number(84.0)), Box::new(Expr::Number(2.0)));
        assert_eq!(evaluate(&e), 42.0);
        let x = Expr::Divide(Box::new(Expr::Number(84.0)), Box::new(Expr::Number(-2.0)));
        assert_eq!(evaluate(&x), -42.0);
        let p = Expr::Divide(Box::new(Expr::Number(21.0)), Box::new(Expr::Number(0.5)));
        assert_eq!(evaluate(&p), 42.0);
        let r = Expr::Divide(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(1.0)));
        assert_eq!(evaluate(&r), 0.0);
    }

    /// About
    /// -----
    /// - Test for nested expressions
    #[test]
    fn test_expr_nested() {
        // ((1 + 2) * 3) = 9.0
        let inner_add = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
        let outer_mult = Expr::Multiply(Box::new(inner_add), Box::new(Expr::Number(3.0)));
        assert_eq!(evaluate(&outer_mult), 9.0);
    }

    /// About
    /// -----
    /// - Test for format_expr
    #[test]
    fn test_format_expr() {
        let inner_add = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
        let outer_mult = Expr::Multiply(Box::new(inner_add), Box::new(Expr::Number(3.0)));
        assert_eq!(format_expr(&outer_mult), "((1 + 2) * 3)");
    }

    /// About
    /// -----
    /// - Test for validating operator tree structure
    #[test]
    fn test_validate_op_tree_struct() {
        let inner_add = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
        let outer_div = Expr::Divide(Box::new(inner_add), Box::new(Expr::Number(3.0)));
        assert_eq!(format_expr(&outer_div), "((1 + 2) / 3)");
        assert_eq!(evaluate(&outer_div), 1.0);
    }

    /// About
    /// -----
    /// - Test for count_nodes
    #[test]
    fn test_count_nodes() {
        let e = Expr::Add(
            Box::new(Expr::Number(5.0)), 
            Box::new(Expr::Negate(Box::new(Expr::Number(3.0))))
        );
        assert_eq!(count_nodes(&e), 4);
    }

    /// About
    /// -----
    /// - Test for test_depth
    #[test]
    fn test_depth() {
       let e = Expr::Add(
            Box::new(Expr::Number(5.0)), 
            Box::new(Expr::Negate(Box::new(Expr::Number(3.0))))
        );
        assert_eq!(depth(&e), 3);
    }

    /// About
    /// -----
    /// - Test for count_numbers
    #[test]
    fn test_count_numbers() {
        let e = Expr::Multiply(
            Box::new(Expr::Number(10.0)), 
            Box::new(Expr::Subtract(Box::new(Expr::Number(5.0)), Box::new(Expr::Number(2.0))))
        );
        assert_eq!(collect_numbers(&e), vec![10.0, 5.0, 2.0]);
    }

    /// About
    /// -----
    /// - Test for to_rpn
    #[test]
    fn test_to_rpn() {
        let e = Expr::Multiply(
            Box::new(Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)))), 
            Box::new(Expr::Number(3.0))
        );
        assert_eq!(to_rpn(&e), "1 2 + 3 *");
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
    // Example Expression: ((1 + 2) * 3) = 6
    let inner_add = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
    let expr = Expr::Multiply(Box::new(inner_add), Box::new(Expr::Number(3.0)));
    println!("Target Expression: {}", expr);

    println!("\n==================== count_nodes =========================");
    println!("Total nodes: {}", count_nodes(&expr));

    println!("\n==================== depth ===============================");
    println!("Tree depth: {}", depth(&expr));
    
    println!("\n==================== collect_numbers =======================");
    println!("Collected numbers: {:?}", collect_numbers(&expr));
    
    println!("\n==================== to_rpn ==============================");
    println!("RPN representation: {}", to_rpn(&expr));
}

// ================================================================================================
// END Main Function
// ================================================================================================