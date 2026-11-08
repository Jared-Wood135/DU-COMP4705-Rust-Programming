//! FILE OVERVIEW:
//! - COMP3705 Assignment 04
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - Initial copied file from COMP3705 Homework 08
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - Enumerations
//!     - pub Value
//!     - pub Expr
//!         - Number
//!         - Negate
//!         - Add
//!         - Subtract
//!         - Multiply
//!         - Divide
//!         - True
//!         - False
//!         - Nil
//!         - Not
//!         - NotEqual
//!         - Greater
//!         - GreaterEqual
//!         - Less
//!         - LessEqual
//! - Primary Functions
//!     - pub is_truthy
//!     - pub evaluate
//! - Helper Functions
//!     - format_expr
//!     - impl fmt::Display (Expr)
//!     - impl fmt::Display (Value)
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
///     - True
///     - False
///     - Nil
///     - Not
///     - Equal
///     - NotEqual
///     - Greater
///     - GreaterEqual
///     - Less
///     - LessEqual
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Negate(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    True,
    False,
    Nil,
    Not(Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
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


/// About
/// -----
/// - Defines whether a value is a number, bool, or nil
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    ValNumber(f64),
    ValBool(bool),
    ValNil,
}

// ================================================================================================
// END Enumerations
// START Primary Functions
// ================================================================================================

/// About
/// -----
/// - Determines truthiness of a Lox value: nil and false are falsy, everything else is truthy
/// 
/// Parameters
/// ----------
/// - val (&Value)
///     - The value to be evaluated
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - bool
///     - True if `val` is not nil or false
///     - False otherwise
pub fn is_truthy(val: &Value) -> bool {
    match val {
        Value::ValBool(false) | Value::ValNil => false,
        _ => true,
    }
}


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
///     - True
///     - False
///     - Nil
///
/// Parameters
/// ----------
/// - expr (&Expr)
///     - The expression of value(s) to be evaluated
///
/// Panics
/// ------
/// - Generic panic if any of the expression matches fail
///
/// Output
/// ------
/// - Value
///     - The `Value` associated with the respective `Expr`
pub fn evaluate(expr: &Expr) -> Value {
    match expr {
        Expr::Number(n) => Value::ValNumber(*n),
        Expr::True => Value::ValBool(true),
        Expr::False => Value::ValBool(false),
        Expr::Nil => Value::ValNil,
        Expr::Negate(inner) => {
            let val = evaluate(inner);
            match val {
                Value::ValNumber(n) => Value::ValNumber(-n),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Not(inner) => {
            let val = evaluate(inner);
            Value::ValBool(!is_truthy(&val))
        }
        Expr::Add(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a + b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Subtract(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a - b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Multiply(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a * b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Divide(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValNumber(a / b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Equal(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            Value::ValBool(l == r)
        }
        Expr::NotEqual(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            Value::ValBool(l != r)
        }
        Expr::Greater(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValBool(a > b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::GreaterEqual(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValBool(a >= b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::Less(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValBool(a < b),
                _ => panic!("Operands must be numbers."),
            }
        }
        Expr::LessEqual(left, right) => {
            let l = evaluate(left);
            let r = evaluate(right);
            match (l, r) {
                (Value::ValNumber(a), Value::ValNumber(b)) => Value::ValBool(a <= b),
                _ => panic!("Operands must be numbers."),
            }
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
        Expr::Number(n) => format!("{}", n),
        Expr::True => format!("true"),
        Expr::False => format!("false"),
        Expr::Nil => format!("nil"),
        Expr::Negate(inner) => format!("(-{})", format_expr(inner)),
        Expr::Add(l, r) => format!("({} + {})", format_expr(l), format_expr(r)),
        Expr::Subtract(l, r) => format!("({} - {})", format_expr(l), format_expr(r)),
        Expr::Multiply(l, r) => format!("({} * {})", format_expr(l), format_expr(r)),
        Expr::Divide(l, r) => format!("({} / {})", format_expr(l), format_expr(r)),
        Expr::Not(inner) => format!("(!{})", format_expr(inner)),
        Expr::Equal(l, r) => format!("({} == {})", format_expr(l), format_expr(r)),
        Expr::NotEqual(l, r) => format!("({} != {})", format_expr(l), format_expr(r)),
        Expr::Greater(l, r) => format!("({} > {})", format_expr(l), format_expr(r)),
        Expr::GreaterEqual(l, r) => format!("({} >= {})", format_expr(l), format_expr(r)),
        Expr::Less(l, r) => format!("({} < {})", format_expr(l), format_expr(r)),
        Expr::LessEqual(l, r) => format!("({} <= {})", format_expr(l), format_expr(r)),
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


/// About
/// -----
/// - Trait for `Value` so that println!("{}", Value) uses `format_expr` function
/// 
/// Parameters
/// ----------
/// - f (&mut fmt::Formatter<'_>)
///     - Original expression
/// 
/// Returns
/// -------
/// - fmt::Result
///     - Formatted Value
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::ValNumber(n) => write!(f, "{}", n),
            Value::ValBool(b) => write!(f, "{}", b),
            Value::ValNil => write!(f, "nil"),
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
    fn test_expr_number() {
        let e = Expr::Number(42.0);
        assert_eq!(evaluate(&e), Value::ValNumber(42.0));
        let x = Expr::Number(-1.0);
        assert_eq!(evaluate(&x), Value::ValNumber(-1.0));
        let p = Expr::Number(-123456789.123456789);
        assert_eq!(evaluate(&p), Value::ValNumber(-123456789.123456789));
        let r = Expr::Number(0.0);
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for Expr::Negate
    #[test]
    fn test_expr_negate() {
        let e = Expr::Negate(Box::new(Expr::Number(42.0)));
        assert_eq!(evaluate(&e), Value::ValNumber(-42.0));
        let x = Expr::Negate(Box::new(Expr::Number(-1.0)));
        assert_eq!(evaluate(&x), Value::ValNumber(1.0));
        let p = Expr::Negate(Box::new(Expr::Number(-123456789.123456789)));
        assert_eq!(evaluate(&p), Value::ValNumber(123456789.123456789));
        let r = Expr::Negate(Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for Expr::Add
    #[test]
    fn test_expr_add() {
        let e = Expr::Add(Box::new(Expr::Number(40.0)), Box::new(Expr::Number(2.0)));
        assert_eq!(evaluate(&e), Value::ValNumber(42.0));
        let x = Expr::Add(Box::new(Expr::Number(-1.0)), Box::new(Expr::Number(-1.0)));
        assert_eq!(evaluate(&x), Value::ValNumber(-2.0));
        let p = Expr::Add(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(123456789.123456789)));
        assert_eq!(evaluate(&p), Value::ValNumber(0.0));
        let r = Expr::Add(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for Expr::Subtract
    #[test]
    fn test_expr_subtract() {
        let e = Expr::Subtract(Box::new(Expr::Number(50.0)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&e), Value::ValNumber(42.0));
        let x = Expr::Subtract(Box::new(Expr::Number(-10.0)), Box::new(Expr::Number(10.0)));
        assert_eq!(evaluate(&x), Value::ValNumber(-20.0));
        let p = Expr::Subtract(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(-123456789.123456789)));
        assert_eq!(evaluate(&p), Value::ValNumber(0.0));
        let r = Expr::Subtract(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for Expr::Multiply
    #[test]
    fn test_expr_multiply() {
        let e = Expr::Multiply(Box::new(Expr::Number(5.25)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&e), Value::ValNumber(42.0));
        let x = Expr::Multiply(Box::new(Expr::Number(-5.25)), Box::new(Expr::Number(8.0)));
        assert_eq!(evaluate(&x), Value::ValNumber(-42.0));
        let p = Expr::Multiply(Box::new(Expr::Number(-123456789.123456789)), Box::new(Expr::Number(1.0)));
        assert_eq!(evaluate(&p), Value::ValNumber(-123456789.123456789));
        let r = Expr::Multiply(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(0.0)));
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for Expr::Divide
    #[test]
    fn test_expr_divide() {
        let e = Expr::Divide(Box::new(Expr::Number(84.0)), Box::new(Expr::Number(2.0)));
        assert_eq!(evaluate(&e), Value::ValNumber(42.0));
        let x = Expr::Divide(Box::new(Expr::Number(84.0)), Box::new(Expr::Number(-2.0)));
        assert_eq!(evaluate(&x), Value::ValNumber(-42.0));
        let p = Expr::Divide(Box::new(Expr::Number(21.0)), Box::new(Expr::Number(0.5)));
        assert_eq!(evaluate(&p), Value::ValNumber(42.0));
        let r = Expr::Divide(Box::new(Expr::Number(0.0)), Box::new(Expr::Number(1.0)));
        assert_eq!(evaluate(&r), Value::ValNumber(0.0));
    }

    /// About
    /// -----
    /// - Test for nested expressions
    #[test]
    fn test_expr_nested() {
        // ((1 + 2) * 3) = 9.0
        let inner_add = Expr::Add(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(2.0)));
        let outer_mult = Expr::Multiply(Box::new(inner_add), Box::new(Expr::Number(3.0)));
        assert_eq!(evaluate(&outer_mult), Value::ValNumber(9.0));
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
        assert_eq!(evaluate(&outer_div), Value::ValNumber(1.0));
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
    let num = Expr::Number(42.0);
    println!("Expr: {:?}", num);
    println!("Formatted: {}", num);
    println!("Evaluated: {}", evaluate(&num));

    println!("\n==================== Expr::Negate ====================");
    let neg = Expr::Negate(Box::new(Expr::Number(42.0)));
    println!("Expr: {:?}", neg);
    println!("Formatted: {}", neg);
    println!("Evaluated: {}", evaluate(&neg));

    println!("\n==================== Expr::Add =======================");
    let add = Expr::Add(Box::new(Expr::Number(21.0)), Box::new(Expr::Number(21.0)));
    println!("Expr: {:?}", add);
    println!("Formatted: {}", add);
    println!("Evaluated: {}", evaluate(&add));

    println!("\n==================== Expr::Subtract ==================");
    let sub = Expr::Subtract(Box::new(Expr::Number(50.0)), Box::new(Expr::Number(8.0)));
    println!("Expr: {:?}", sub);
    println!("Formatted: {}", sub);
    println!("Evaluated: {}", evaluate(&sub));

    println!("\n==================== Expr::Multiply ==================");
    let mul = Expr::Multiply(Box::new(Expr::Number(5.25)), Box::new(Expr::Number(8.0)));
    println!("Expr: {:?}", mul);
    println!("Formatted: {}", mul);
    println!("Evaluated: {}", evaluate(&mul));

    println!("\n==================== Expr::Divide ====================");
    let div = Expr::Divide(Box::new(Expr::Number(21.0)), Box::new(Expr::Number(0.5)));
    println!("Expr: {:?}", div);
    println!("Formatted: {}", div);
    println!("Evaluated: {}", evaluate(&div));

    println!("\n==================== Expr - Nested ===================");
    let nested = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Number(3.0)),
    );
    println!("Formatted: {}", nested);
    println!("Evaluated: {}", evaluate(&nested));
    println!("Tree Structure Displayed via Debug: {:#?}", nested);
}

// ================================================================================================
// END Main Function
// ================================================================================================