//! FILE OVERVIEW:
//! - COMP3705 Daily Homework 07
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
//! - Structures
//!     - Direction
//! - Test Functions
//!     - test_direction_creation_i32
//!     - test_direction_creation_f64
//!     - test_custom_add_method
//!     - test_custom_sub_method
//!     - test_operator_add
//!     - test_operator_sub
//!     - test_display_trait
//!     - test_print_method_execution
//!     - test_negative_and_zero_values
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
use std::fmt::Display;
use std::ops::{Add, Sub};

// ----- Global Variables -------------------------------------------------------------------------


// ================================================================================================
// END File Overview, Imports, Global Variables
// START Structures
// ================================================================================================

/// About
/// -----
/// - Represents a two-dimensional vector
/// - Allows for .clone()
/// 
/// Parameters
/// ----------
/// - T
///     - The same variable type to set for both self.x and self.y
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - `Direction` object
#[derive(Clone)]
struct Direction<T> {
    x: T,
    y: T,
}

/// About
/// -----
/// - Prints off information about the this `Direction` object
/// 
/// Parameters
/// ----------
/// - &self
impl<T: Display> Direction<T> {
    fn print(&self) {
        print!("Direction({}, {})", self.x, self.y);
    }
}

/// About
/// -----
/// - Adds self and another `Direction` object together for another `Direction` object
/// 
/// Parameters
/// ----------
/// - &self
/// - other (Direction)
///     - The other `Direction` object for adding
/// 
/// Returns
/// -------
/// - Direction
///     - New `Direction` object consisting of adding `self` and `other` together
impl<T: Add<Output = T> + Sub<Output = T> + Clone> Direction<T> {
    fn add(&self, other: Direction<T>) -> Direction<T> {
        Direction {
            x: self.x.clone() + other.x,
            y: self.y.clone() + other.y,
        }
    }

    /// About
    /// -----
    /// - Subtracts self and another `Direction` object together for another `Direction` object
    /// 
    /// Parameters
    /// ----------
    /// - &self
    /// - other (Direction)
    ///     - The other `Direction` object for subtraction
    /// 
    /// Returns
    /// -------
    /// - Direction
    ///     - New `Direction` object consisting of subtracting `other` from `self`
    fn sub(&self, other: Direction<T>) -> Direction<T> {
        Direction {
            x: self.x.clone() - other.x,
            y: self.y.clone() - other.y,
        }
    }
}

/// About
/// -----
/// - println! override using the same print output as the `Direction` object print
/// 
/// Parameters
/// ----------
/// - &self
/// - f (&mut std::fmt::Formatter<'_>)
/// 
/// Returns
/// -------
/// - Overridden println! for `Direction` objects
impl<T: Display> std::fmt::Display for Direction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction({}, {})", self.x, self.y)
    }
}

/// About
/// -----
/// - Addition override for `Direction` objects
/// 
/// Parameters
/// ----------
/// - self (Direction)
/// - other (Direction)
/// 
/// Returns
/// -------
/// - Direction
///     - New `Direction` object from addition arithmetic
impl<T: Add<Output = T>> Add for Direction<T> {
    type Output = Direction<T>;

    fn add(self, other: Direction<T>) -> Direction<T> {
        Direction {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// About
/// -----
/// - Subtraction override for `Direction` objects
/// 
/// Parameters
/// ----------
/// - self (Direction)
/// - other (Direction)
/// 
/// Returns
/// -------
/// - Direction
///     - New `Direction` object from subtracting `other` from `self`
impl<T: Sub<Output = T>> Sub for Direction<T> {
    type Output = Direction<T>;

    fn sub(self, other: Direction<T>) -> Direction<T> {
        Direction {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// ================================================================================================
// END Structures
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// About
    /// -----
    /// - Test basic creation of Direction with integer values
    #[test]
    fn test_direction_creation_i32() {
        let d = Direction { x: 1, y: 2 };
        assert_eq!(d.x, 1);
        assert_eq!(d.y, 2);
    }

    /// About
    /// -----
    /// - Test basic creation of Direction with floating-point values
    #[test]
    fn test_direction_creation_f64() {
        let d = Direction { x: 1.5, y: -1.5 };
        assert_eq!(d.x, 1.5);
        assert_eq!(d.y, -1.5);
    }

    /// About
    /// -----
    /// - Test the custom `.add()` method implementation
    #[test]
    fn test_custom_add_method() {
        let d1 = Direction { x: 1, y: 2 };
        let d2 = Direction { x: 2, y: 1 };
        let result = d1.add(d2);
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
    }

    /// About
    /// -----
    /// - Test the custom `.sub()` method implementation
    #[test]
    fn test_custom_sub_method() {
        let d1 = Direction { x: 1, y: 2 };
        let d2 = Direction { x: 1, y: 2 };
        let result = d1.sub(d2);
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    /// About
    /// -----
    /// - Test the standard `Add` operator (`+`) implementation
    #[test]
    fn test_operator_add() {
        let d1 = Direction { x: 1, y: 2 };
        let d2 = Direction { x: 2, y: 1 };
        let result = d1 + d2;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
    }

    /// About
    /// -----
    /// - Test the standard `Sub` operator (`-`) implementation
    #[test]
    fn test_operator_sub() {
        let d1 = Direction { x: 1, y: 2 };
        let d2 = Direction { x: 1, y: 2 };
        let result = d1 - d2;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    /// About
    /// -----
    /// - Test the `Display` trait implementation format string
    #[test]
    fn test_display_trait() {
        let d = Direction { x: 1, y: 2 };
        let display_str = format!("{}", d);
        assert_eq!(display_str, "Direction(1, 2)");
    }

    /// About
    /// -----
    /// - Test that the `.print()` method executes successfully without panicking
    #[test]
    fn test_print_method_execution() {
        let d = Direction { x: 1, y: 2 };
        d.print();
    }

    /// About
    /// -----
    /// - Test combinations of negative numbers and zero values
    #[test]
    fn test_negative_and_zero_values() {
        let d1 = Direction { x: -1, y: 0 };
        let d2 = Direction { x: 1, y: -1 };
        let result = d1 + d2;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, -1);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs direction creation, addition, subtraction, and prints for rapid verification
fn main() {
    let d1 = Direction { x: 1, y: 2 };
    let d2 = Direction { x: 2, y: 1 };

    println!("\n==================== Direction Creation ======================");
    println!("Below uses Direction.print()");
    print!("D1: ");
    d1.print();
    print!("\nD2: ");
    d2.print();
    println!("\n");
    println!("Below uses println! (i.e. Direct calls to Direction object)");
    println!("D1: {}\nD2: {}", d1, d2);
    
    println!("\n==================== Direction Addition ======================");
    let add_fn = d1.clone().add(d2.clone());
    let add_op = d1.clone() + d2.clone();
    println!("d1.add(d2) = {}", add_fn);
    println!("d1 + d2 = {}\n", add_op);

    println!("\n==================== Direction Subtraction ===================");
    let sub_fn = d1.clone().sub(d2.clone());
    let sub_op = d1.clone() - d2.clone();
    println!("d1.sub(d2) = {}", sub_fn);
    println!("d1 - d2 = {}", sub_op);
}

// ================================================================================================
// END Main Function
// ================================================================================================