//! FILE OVERVIEW:
//! - COMP3705 Assignment 3
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
//! - Structures
//!     - Parser
//!         - init_parser
//!         - peek
//!         - previous
//!         - is_at_end
//!         - advance
//!         - check
//!         - match_token
//!         - consume
//!         - parse
//!         - parse_expression
//!         - parse_term
//!         - parse_unary
//!         - parse_factor
//! - Assignment 03 Functions
//!     - scan_all
//!     - interpret
//!     - make_test_token
//! - Test Functions
//!     - test_parse_number
//!     - test_addition
//!     - test_precedence
//!     - test_parentheses
//!     - test_left_associative
//!     - test_unary_negation
//!     - test_complex
//!     - test_single_number
//!     - test_multiplication
//!     - test_negation
//!     - test_nested_parens
//!     - test_complex_expression
//!     - test_left_associative_division
//!     - test_all_operators
//!     - test_negation_in_expression
//!     - test_negation_after_operator
//!     - test_double_negation
//!     - test_triple_negation
//!     - test_deeply_nested_parentheses
//!     - test_division_and_precedence
//!     - test_left_associativity_mixed
//!     - test_negation_with_parentheses_and_operators
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// expr.rs imports
mod expr;
use expr::{Expr};
use expr::evaluate;

// scanner.rs imports
mod scanner;
use scanner::{Scanner, Token, TokenType};

// ----- Global Variables -------------------------------------------------------------------------


// ================================================================================================
// END File Overview, Imports, Global Variables
// START Structures
// ================================================================================================

/// About
/// -----
/// - Represents the current state of the parsing process
/// 
/// Parameters
/// ----------
/// - tokens (Vec<Token>)
///     - All the tokens produced by the scanner
/// - current (usize)
///     - Index into the tokens vector, tracking which token we are about to consume
/// - had_error (bool)
///     - Indicating whether any errors occurred during parsing
struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    /// About
    /// -----
    /// - Takes a Vec<Token> and returns a new Parser with current set to 0 and had_error set to false
    /// 
    /// Parameters
    /// ----------
    /// - tokens (Vec<Token>)
    ///     - Tokens to be parsed through
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Parser
    ///     - Initialized with starting states
    fn init_parser(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    /// About
    /// -----
    /// - Returns a reference to the token at the current index
    /// - This is the token we are about to consume
    /// 
    /// Parameters
    /// ----------
    /// - &self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - &Token
    ///     - The token we are about to consume
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// About
    /// -----
    /// - Returns a reference to the token at current - 1
    /// - This is the token we most recently consumed
    /// 
    /// Parameters
    /// ----------
    /// - &self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - &Token
    ///     - The token we most recently consumed
    fn previous(&self) -> &Token {
        if self.current > 0 {
            &self.tokens[self.current - 1]
        } 
        else {
            &self.tokens[0]
        }
    }

    /// About
    /// -----
    /// - Returns true if the current token is TokenEof
    /// 
    /// Parameters
    /// ----------
    /// - &self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - bool
    ///     - true if the current token is TokenEof; Otherwise false
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::TokenEof
    }

    /// About
    /// -----
    /// - If the parser is not at the end, increment current by 1
    /// - Returns a reference to the token that was just consumed (i.e., the token at the old current position)
    /// 
    /// Parameters
    /// ----------
    /// - & mut self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - &Token
    ///     - Reference to the token that was just consumed
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// About
    /// -----
    /// - Takes a TokenType and returns true if the current token has that type
    /// - Returns false if the parser is at the end
    /// 
    /// Parameters
    /// ----------
    /// - &self
    /// - tt (TokenType)
    ///     - The type of token to match on
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - bool
    ///     - true if the current token matches `tt`; Otherwise false
    fn check(&self, tt: TokenType) -> bool {
        self.peek().token_type == tt
    }

    /// About
    /// -----
    /// - Takes a slice of TokenType values (e.g., &[TokenType::TokenPlus, TokenType::TokenMinus])
    /// - If the current token matches any of them, call advance and return true
    /// - Otherwise return false
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// - types (&[TokenType])
    ///     - Types of tokens to match any on the current token
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - bool
    ///     - true if any `types` matched; Otherwise false
    fn match_token(&mut self, types: &[TokenType],) -> bool {
        for tt in types {
            if self.check(*tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// About
    /// -----
    /// - Takes a TokenType and an error message &str
    /// - If the current token matches the given type, call advance
    /// - Otherwise, report the error by printing the message and setting had_error to true
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// - tt (TokenType)
    ///     - Type of token to match the current token on
    /// - message (&str)
    ///     - The error message to be printed upon mismatched `tt`
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - None; Printed error message if `tt` mismatched
    fn consume(&mut self, tt: TokenType, message: &str,) {
        if self.check(tt) {
            self.advance();
            return;
        }
        println!("[line {}] Error: {}", self.peek().line, message);
        self.had_error = true;
    }

    /// About
    /// -----
    /// - The entry point
    /// - This method calls parse_expression and returns the resulting Expr
    /// - After parsing, it should verify that the remaining token is TokenEof using consume
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The resulting Expr from `parse_expression`
    fn parse(&mut self) -> Expr {
        let expr = self.parse_expression();
        self.consume(TokenType::TokenEof, "Expected end of expression.",);
        expr
    }

    /// About
    /// -----
    /// - Implements the rule expression → term (('+' | '-') term)*:
    ///     1. Call parse_term and store the result as left
    ///     2. While match_token succeeds for TokenPlus or TokenMinus:
    ///         - Check which operator was matched (use previous to see the consumed token)
    ///         - Call parse_term to get the right operand
    ///         - Combine left and right into an Expr::Add or Expr::Subtract node, and update left
    ///     3. Return left.
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - unreachable!
    ///     - If the operator is neither + or -
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The `Expr` resulting from `parse_term`
    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();
        while self.match_token(&[TokenType::TokenPlus, TokenType::TokenMinus,]) {
            let op = self.previous().token_type;
            let right = self.parse_term();
            left = match op {
                TokenType::TokenPlus =>
                    Expr::Add(
                        Box::new(left), Box::new(right)),
                TokenType::TokenMinus =>
                    Expr::Subtract(
                        Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Implements the rule term → unary (('*' | '/') unary)*:
    ///     - Same pattern as parse_expression, but matches TokenStar and TokenSlash, calls parse_unary, and builds Expr::Multiply or Expr::Divide nodes
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - unreachable!
    ///     - If the operator is neither * or /
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The `Expr` resulting from `parse_unary`
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while self.match_token(&[TokenType::TokenStar, TokenType::TokenSlash,]) {
            let op = self.previous().token_type;
            let right = self.parse_unary();
            left = match op {
                TokenType::TokenStar =>
                    Expr::Multiply(
                        Box::new(left), Box::new(right)),
                    TokenType::TokenSlash =>
                        Expr::Divide(
                            Box::new(left), Box::new(right)),
                    _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Implements the rule unary → '-' unary | factor:
    ///     1. If match_token succeeds for TokenMinus:
    ///         - Call parse_unary recursively (this handles --5 correctly)
    ///         - Wrap the result in Expr::Negate and return it
    ///     2. Otherwise, call parse_factor and return its result.
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The result from handling double negations or `parse_factor`
    fn parse_unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::TokenMinus]) {
            let operand = self.parse_unary();
            return Expr::Negate(Box::new(operand));
        }
        self.parse_factor()
    }

    /// About
    /// -----
    /// - Implements the rule factor → number | '(' expression ')':
    ///     1. If match_token succeeds for TokenNumber:
    ///         - Get the value of the previous token
    ///         - Convert it from bytes to a String, then parse it as an f64
    ///         - Return Expr::Number(value)
    ///     2. If match_token succeeds for TokenLeftParen:
    ///         - Call parse_expression to parse the inner expression
    ///         - Call consume with TokenRightParen and the message "Expected ')' after expression."
    ///         - Return the inner expression
    ///     3. Otherwise, report an error ("Expected expression.") and return Expr::Number(0.0) as a fallback
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - None
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - A Number, inner expression, or fallback 0.0
    fn parse_factor(&mut self) -> Expr {
        // number
        if self.match_token(&[TokenType::TokenNumber]) {
            let text = String::from_utf8(self.previous().value.clone()).unwrap_or_default();
            let value: f64 = text.parse().expect("invalid number");
            return Expr::Number(value);
        }
        // '(' expression ')'
        if self.match_token(&[TokenType::TokenLeftParen]) {
            let inner = self.parse_expression();
            self.consume(TokenType::TokenRightParen, "Expected ')' after expression.",);
            return inner;
        }
        println!("[line {}] Error: Expected expression.", self.peek().line);
        self.had_error = true;
        Expr::Number(0.0)
    }
}

// ================================================================================================
// END Structures
// START Assignment 03 Functions
// ================================================================================================

/// About
/// -----
/// - Takes a &str of source code, creates a Scanner, and calls scan_token in a loop until TokenEof is reached
/// - It should return a Vec<Token> containing all the tokens (including the final TokenEof)
/// 
/// Parameters
/// ----------
/// - source (&str)
///     - The source code to be scanned
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<Token>
///     - All tokens from scanning `source`
fn scan_all(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::init_scanner(source);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.scan_token();
        let is_eof =
        token.token_type == TokenType::TokenEof;
        tokens.push(token);
        if is_eof { break; }
    }

    tokens // always ends with Eof - the parser relies on it
}


/// About
/// -----
/// - Takes a &str of source code and returns an f64:
///     1. Call scan_all to get the tokens
///     2. Create a Parser with init_parser and call parse to get the Expr
///     3. Call evaluate on the Expr and return the result
/// 
/// Parameters
/// ----------
/// - source (&str)
///     - The source code to be scanned
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - f64
///     - The evaluated Expr resulting from scanning `source`
fn interpret(source: &str) -> f64 {
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    let expr = parser.parse();
    evaluate(&expr)
}

/// About
/// -----
/// - Helper function for unit testing to easily create a Token
/// 
/// Parameters
/// ----------
/// - tt (TokenType)
///     - The type of token to be associated with this token
/// - value (&str)
///     - The value to be associated with this token
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Token
///     - Created token intended for unit testing purposes
fn make_test_token(token_type: TokenType, value: &str) -> Token {
    Token {
        token_type,
        value: value.as_bytes().to_vec(),
        line: 1,
    }
}

// ================================================================================================
// END Assignment 03 Functions
// START Test Functions
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        // Tokens for: 42
        let tokens = vec![
            make_test_token(TokenType::TokenNumber, "42"),
            make_test_token(TokenType::TokenEof, ""),
        ];
        let mut parser = Parser::init_parser(tokens);
        let expr = parser.parse();
        assert_eq!(evaluate(&expr), 42.0);
    }

    #[test]
    fn test_addition() {
        assert_eq!(interpret("1 + 2"), 3.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(interpret("1 + 2 * 3"), 7.0);
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(interpret("(1 + 2) * 3"), 9.0);
    }

    #[test]
    fn test_left_associative() {
        assert_eq!(interpret("10 - 2 - 3"), 5.0);
    }

    #[test]
    fn test_unary_negation() {
        assert_eq!(interpret("-5"), -5.0);
    }

    #[test]
    fn test_complex() {
        assert_eq!(interpret("(8 - 3) * (2 + 2)"), 20.0);
    }

    #[test]
    fn test_single_number() {
        assert_eq!(interpret("42"), 42.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(interpret("3 * 4"), 12.0);
    }

    #[test]
    fn test_negation() {
        assert_eq!(interpret("-5"), -5.0);
    }

    #[test]
    fn test_nested_parens() {
        assert_eq!(interpret("((1 + 2))"), 3.0);
    }

    #[test]
    fn test_complex_expression() {
    // (8 - 3) * (2 + 2) = 5 * 4 = 20
        assert_eq!(interpret("(8 - 3) * (2 + 2)"), 20.0);
    }

    #[test]
    fn test_left_associative_division() {
    // 24 / 4 / 2 = (24 / 4) / 2 = 3
        assert_eq!(interpret("24 / 4 / 2"), 3.0);
    }

    #[test]
    fn test_all_operators() {
    // 2 + 3 * 5 - 4 / 2 = 2 + 15 - 2 = 15
        assert_eq!(interpret("2 + 3 * 5 - 4 / 2"), 15.0);
    }

    #[test]
    fn test_negation_in_expression() {
        assert_eq!(interpret("-5 + 3"), -2.0);
    }

    #[test]
    fn test_negation_after_operator() {
        assert_eq!(interpret("2 * -3"), -6.0);
    }

    #[test]
    fn test_double_negation() {
        // Double negation (--5) should evaluate to 5.0
        assert_eq!(interpret("--5"), 5.0);
    }

    #[test]
    fn test_triple_negation() {
        // Triple negation (---5) should evaluate to -5.0
        assert_eq!(interpret("---5"), -5.0);
    }

    #[test]
    fn test_deeply_nested_parentheses() {
        // Deeply nested parentheses (((42)))
        assert_eq!(interpret("(((42)))"), 42.0);
        // Deeply nested arithmetic (((1 + 2) * (3 + 4))) = 3 * 7 = 21.0
        assert_eq!(interpret("(((1 + 2) * (3 + 4)))"), 21.0);
    }

    #[test]
    fn test_division_and_precedence() {
        // Basic division
        assert_eq!(interpret("10 / 2"), 5.0);
        // Floating point result
        assert_eq!(interpret("5 / 2"), 2.5);
        // Mixed precedence with division: 10 + 20 / 4 - 2 = 10 + 5 - 2 = 13.0
        assert_eq!(interpret("10 + 20 / 4 - 2"), 13.0);
    }

    #[test]
    fn test_left_associativity_mixed() {
        // Left associativity for subtraction and division: 100 / 10 / 2 = 5.0
        assert_eq!(interpret("100 / 10 / 2"), 5.0);
        // 20 - 10 - 5 - 2 = ((20 - 10) - 5) - 2 = 3.0
        assert_eq!(interpret("20 - 10 - 5 - 2"), 3.0);
    }

    #[test]
    fn test_negation_with_parentheses_and_operators() {
        // -(5 + 3) * -2 = -8 * -2 = 16.0
        assert_eq!(interpret("-(5 + 3) * -2"), 16.0);
        // -(-5) = 5.0
        assert_eq!(interpret("-(-5)"), 5.0);
    }

    #[test]
    fn test_error_missing_right_paren() {
        // Missing closing parenthesis: (1 + 2
        let tokens = vec![
            make_test_token(TokenType::TokenLeftParen, "("),
            make_test_token(TokenType::TokenNumber, "1"),
            make_test_token(TokenType::TokenPlus, "+"),
            make_test_token(TokenType::TokenNumber, "2"),
            make_test_token(TokenType::TokenEof, ""),
        ];
        let mut parser = Parser::init_parser(tokens);
        let _ = parser.parse();
        assert!(parser.had_error, "Expected parser to flag an error for missing ')'");
    }

    #[test]
    fn test_error_unexpected_trailing_tokens() {
        // Extra token at the end: 1 + 2 3
        let tokens = vec![
            make_test_token(TokenType::TokenNumber, "1"),
            make_test_token(TokenType::TokenPlus, "+"),
            make_test_token(TokenType::TokenNumber, "2"),
            make_test_token(TokenType::TokenNumber, "3"), // Unexpected
            make_test_token(TokenType::TokenEof, ""),
        ];
        let mut parser = Parser::init_parser(tokens);
        let _ = parser.parse();
        assert!(parser.had_error, "Expected parser to flag an error for trailing tokens");
    }

    #[test]
    fn test_error_empty_expression() {
        let tokens = vec![
            make_test_token(TokenType::TokenEof, ""),
        ];
        let mut parser = Parser::init_parser(tokens);
        let _ = parser.parse();
        assert!(parser.had_error, "Expected parser to flag an error for an empty expression");
    }

    #[test]
    fn test_division_by_zero() {
        let result = interpret("42.0 / 0.0");
        assert!(result.is_infinite(), "Division by zero should result in infinity");
        assert!(result > 0.0);
    }

    #[test]
    fn test_complex_precedence_and_unary() {
        // -2 * (3 + -5) / 2 = -2 * (-2) / 2 = 4 / 2 = 2.0
        assert_eq!(interpret("-2 * (3 + -5) / 2"), 2.0);
    }
}

// ================================================================================================
// END Test Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply runs the assignment provided tests with additional output for rapid visual verification of functionality
fn main() {
    println!("\x1b[35m==================== Provided Tests ====================\x1b[0m");
    println!("1 + 2 * 3 = {}", interpret("1 + 2 * 3"));
    println!("(1 + 2) * 3 = {}", interpret("(1 + 2) * 3"));
    
    println!("\n\x1b[35m============= Double Negations & Negatives =============\x1b[0m");
    println!("--5 = {}", interpret("--5"));
    println!("-5 + 3 = {}", interpret("-5 + 3"));
    println!("2 * -3 = {}", interpret("2 * -3"));
    
    println!("\n\x1b[35m============== Division/Left Associativity =============\x1b[0m");
    println!("24 / 4 / 2 = {}", interpret("24 / 4 / 2"));
    println!("10 + 20 / 4 - 2 = {}", interpret("10 + 20 / 4 - 2"));
    
    println!("\n\x1b[35m================== Complex Expressions =================\x1b[0m");
    println!("(8 - 3) * (2 + 2) = {}", interpret("(8 - 3) * (2 + 2)"));

    println!("\n\x1b[35m==================== Error Handling ====================\x1b[0m");
    let source = "(1 + 2";
    println!("Parsing invalid expression: \"{}\"", source);
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    let _expr = parser.parse();
    if parser.had_error {
        println!("\x1b[32m-> Parser successfully flagged `had_error = true` for the syntax error!\x1b[0m");
    }
    else {
        println!("\x1b[31m-> Parser failed to flag `had_error = false` for the syntax error!\x1b[0m");
    }
}

// ================================================================================================
// END Main Function
// ================================================================================================