//! FILE OVERVIEW:
//! - COMP3705 Assignment 05
//!
//! =================================================
//!
//! MISC COMMENTS:
//! - Initial copied file from COMP3705 Lab 08
//!
//! =================================================
//!
//! FILE CONTENTS:
//! - File Overview, Imports, Global Variables
//! - Structures
//!     - pub Parser
//!         - pub init_parser
//!         - peek
//!         - previous
//!         - is_at_end
//!         - advance
//!         - check
//!         - match_token
//!         - consume
//!         - pub parse
//!         - parse_declaration
//!         - parse_var_declaration
//!         - parse_statement
//!         - parse_print_statement
//!         - parse_expression_statement
//!         - pub parse_expression
//!         - parse_equality
//!         - parse_comparison
//!         - parse_term
//!         - parse_unary
//!         - parse_factor
//!         - parse_primary
//! - Homework 08 / Assignment 05 Functions
//!     - pub scan_all
//!     - pub run
//!     - interpret
//!     - parse_source
//!     - make_test_token
//! - Test Functions
//!     - Expression and statement unit tests
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
use crate::expr::{Environment, Expr, Stmt, Value, evaluate, execute};
use crate::scanner::{Scanner, Token, TokenType};

// ----- Global Variables -------------------------------------------------------------------------
// None

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
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub had_error: bool,
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
    pub fn init_parser(tokens: Vec<Token>) -> Parser {
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
        } else {
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
    /// - &mut self
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
    fn match_token(&mut self, types: &[TokenType]) -> bool {
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
    pub fn consume(&mut self, tt: TokenType, message: &str) {
        if self.check(tt) {
            self.advance();
            return;
        }
        println!("[line {}] Error: {}", self.peek().line, message);
        self.had_error = true;
    }

    /// About
    /// -----
    /// - The entry point for parsing statements
    /// - Loops until `TokenEof` and calls `parse_declaration` on each iteration
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
    /// - Vec<Stmt>
    ///     - The collected list of parsed statements
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let prev_current = self.current;
            statements.push(self.parse_declaration());
            // Prevent infinite loops on unexpected tokens when an error occurs
            if self.current == prev_current && !self.is_at_end() {
                self.advance();
            }
        }
        statements
    }

    /// About
    /// -----
    /// - Implements the rule `declaration -> var_decl | statement`
    /// - If the current token is `TokenVar`, consumes it and calls `parse_var_declaration`
    /// - Otherwise calls `parse_statement`
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
    /// - Stmt
    ///     - The parsed declaration or statement
    fn parse_declaration(&mut self) -> Stmt {
        if self.match_token(&[TokenType::TokenVar]) {
            return self.parse_var_declaration();
        }
        self.parse_statement()
    }

    /// About
    /// -----
    /// - Implements the rule `var_decl -> "var" IDENTIFIER ( '=' expression )? ';'`
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
    /// - Stmt
    ///     - `Stmt::VarDecl` with the variable name and optional initializer
    fn parse_var_declaration(&mut self) -> Stmt {
        self.consume(TokenType::TokenIdentifier, "Expected variable name.");
        let name = String::from_utf8(self.previous().value.clone()).unwrap_or_default();

        let initializer = if self.match_token(&[TokenType::TokenEqual]) {
            Some(self.parse_expression())
        } else {
            None
        };

        self.consume(
            TokenType::TokenSemicolon,
            "Expected ';' after variable declaration.",
        );
        Stmt::VarDecl(name, initializer)
    }

    /// About
    /// -----
    /// - Implements the rule `statement -> print_stmt | expr_stmt`
    /// - If the current token is `TokenPrint`, consumes it and calls `parse_print_statement`
    /// - Otherwise calls `parse_expression_statement`
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
    /// - Stmt
    ///     - The parsed statement
    fn parse_statement(&mut self) -> Stmt {
        if self.match_token(&[TokenType::TokenPrint]) {
            return self.parse_print_statement();
        }
        self.parse_expression_statement()
    }

    /// About
    /// -----
    /// - Implements the rule `print_stmt -> "print" expression ';'`
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
    /// - Stmt
    ///     - `Stmt::Print` wrapping the expression to print
    fn parse_print_statement(&mut self) -> Stmt {
        let value = self.parse_expression();
        self.consume(TokenType::TokenSemicolon, "Expected ';' after value.");
        Stmt::Print(value)
    }

    /// About
    /// -----
    /// - Implements the rule `expr_stmt -> expression ';'`
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
    /// - Stmt
    ///     - `Stmt::Expression` wrapping the expression
    fn parse_expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expression();
        self.consume(TokenType::TokenSemicolon, "Expected ';' after expression.");
        Stmt::Expression(expr)
    }

    /// About
    /// -----
    /// - Calls `parse_equality` and returns the expression
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
    ///     - The `Expr` resulting from `parse_equality`
    pub fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    /// About
    /// -----
    /// - Parses tokens and determines the expression equality to return or the comparison to return
    /// - If not == or != or comparison, then panics
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - unreachable!
    ///     - If not == or != or comparison
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The equality or comparison expression
    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        while self.match_token(&[TokenType::TokenNotEqual, TokenType::TokenEqualEqual]) {
            let op = self.previous().token_type;
            let right = self.parse_comparison();
            left = match op {
                TokenType::TokenEqualEqual => Expr::Equal(Box::new(left), Box::new(right)),
                TokenType::TokenNotEqual => Expr::NotEqual(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Parses tokens and determines the expression comparison to return
    /// - If not a valid comparison, then panics
    /// 
    /// Parameters
    /// ----------
    /// - &mut self
    /// 
    /// Panics
    /// ------
    /// - unreachable!
    ///     - If not a valid comparison
    /// 
    /// Returns
    /// -------
    /// - Expr
    ///     - The comparison expression
    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_term();
        while self.match_token(&[
            TokenType::TokenGreater,
            TokenType::TokenGreaterEqual,
            TokenType::TokenLess,
            TokenType::TokenLessEqual,
        ]) {
            let op = self.previous().token_type;
            let right = self.parse_term();
            left = match op {
                TokenType::TokenGreater => Expr::Greater(Box::new(left), Box::new(right)),
                TokenType::TokenGreaterEqual => Expr::GreaterEqual(Box::new(left), Box::new(right)),
                TokenType::TokenLess => Expr::Less(Box::new(left), Box::new(right)),
                TokenType::TokenLessEqual => Expr::LessEqual(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Implements the rule term -> factor (('+' | '-') factor)*
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
    ///     - The `Expr` resulting from `parse_factor`
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();
        while self.match_token(&[TokenType::TokenPlus, TokenType::TokenMinus]) {
            let op = self.previous().token_type;
            let right = self.parse_factor();
            left = match op {
                TokenType::TokenPlus => Expr::Add(Box::new(left), Box::new(right)),
                TokenType::TokenMinus => Expr::Subtract(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Implements the rule unary -> ('-' | '!') unary | primary
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
    ///     - The result from handling unary operators or `parse_primary`
    fn parse_unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::TokenMinus]) {
            let operand = self.parse_unary();
            return Expr::Negate(Box::new(operand));
        }
        if self.match_token(&[TokenType::TokenNot]) {
            let operand = self.parse_unary();
            return Expr::Not(Box::new(operand));
        }
        self.parse_primary()
    }

    /// About
    /// -----
    /// - Implements the rule factor -> unary (('*' | '/') unary)*
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
    fn parse_factor(&mut self) -> Expr {
        let mut left = self.parse_unary();
        while self.match_token(&[TokenType::TokenStar, TokenType::TokenSlash]) {
            let op = self.previous().token_type;
            let right = self.parse_unary();
            left = match op {
                TokenType::TokenStar => Expr::Multiply(Box::new(left), Box::new(right)),
                TokenType::TokenSlash => Expr::Divide(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }
        left
    }

    /// About
    /// -----
    /// - Implements the rule primary -> true | false | nil | NUMBER | IDENTIFIER | '(' expression ')'
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
    ///     - A literal, variable reference, parenthesized expression, or fallback 0.0 on error
    fn parse_primary(&mut self) -> Expr {
        if self.match_token(&[TokenType::TokenTrue]) {
            return Expr::True;
        }
        if self.match_token(&[TokenType::TokenFalse]) {
            return Expr::False;
        }
        if self.match_token(&[TokenType::TokenNil]) {
            return Expr::Nil;
        }
        if self.match_token(&[TokenType::TokenNumber]) {
            let text = String::from_utf8(self.previous().value.clone()).unwrap_or_default();
            let value: f64 = text.parse().expect("invalid number");
            return Expr::Number(value);
        }
        if self.match_token(&[TokenType::TokenIdentifier]) {
            let name = String::from_utf8(self.previous().value.clone()).unwrap_or_default();
            return Expr::Variable(name);
        }
        if self.match_token(&[TokenType::TokenLeftParen]) {
            let inner = self.parse_expression();
            self.consume(TokenType::TokenRightParen, "Expected ')' after expression.");
            return inner;
        }
        println!("[line {}] Error: Expected expression.", self.peek().line);
        self.had_error = true;
        Expr::Number(0.0)
    }
}

// ================================================================================================
// END Structures
// START Homework 08 / Assignment 05 Functions
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
pub fn scan_all(source: &str) -> Vec<Token> {
    let mut scanner = Scanner::init_scanner(source);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.scan_token();
        let is_eof = token.token_type == TokenType::TokenEof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    tokens // always ends with Eof - the parser relies on it
}

/// About
/// -----
/// - Scans, parses, and executes a program of statements
/// - Used for programs with `print` and `var`
/// 
/// Parameters
/// ----------
/// - source (&str)
///     - The Lox program source code
/// 
/// Panics
/// ------
/// - Propagates runtime panics from `execute`
/// 
/// Returns
/// -------
/// - None
pub fn run(source: &str) {
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    let statements = parser.parse();
    if parser.had_error {
        return;
    }
    let mut env = Environment::new();
    for stmt in &statements {
        execute(stmt, &mut env);
    }
}

/// About
/// -----
/// - Scans, parses, and evaluates a single expression string into a `Value`
/// - Useful for tests that check a single expression's return value
/// 
/// Parameters
/// ----------
/// - source (&str)
///     - The Lox expression source code
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Value
///     - The evaluated `Value` resulting from `source`
fn interpret(source: &str) -> Value {
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    let expr = parser.parse_expression();
    parser.consume(TokenType::TokenEof, "Expected end of expression.");
    let env = Environment::new();
    evaluate(&expr, &env)
}

/// About
/// -----
/// - Helper function that scans and parses a source string into a `Vec<Stmt>`
/// 
/// Parameters
/// ----------
/// - source (&str)
///     - The Lox source code
/// 
/// Panics
/// ------
/// - None
/// 
/// Returns
/// -------
/// - Vec<Stmt>
///     - The parsed statements
pub fn parse_source(source: &str) -> Vec<Stmt> {
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    parser.parse()
}

/// About
/// -----
/// - Helper function for unit testing to easily create a Token
/// 
/// Parameters
/// ----------
/// - token_type (TokenType)
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
// END Homework 08 / Assignment 05 Functions
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
        let expr = parser.parse_expression();
        let env = Environment::new();
        assert_eq!(evaluate(&expr, &env), Value::ValNumber(42.0));
    }

    #[test]
    fn test_addition_normal() {
        assert_eq!(interpret("1 + 2"), Value::ValNumber(3.0));
    }

    #[test]
    fn test_addition_zeros() {
        assert_eq!(interpret("0 + 0"), Value::ValNumber(0.0));
    }

    #[test]
    fn test_addition_long_chain_ones() {
        // "1 + 1 + 1 + ... + 1" with 1,000 terms
        let source = vec!["1"; 1_000].join(" + ");
        assert_eq!(interpret(&source), Value::ValNumber(1_000.0));
    }

    #[test]
    fn test_subtraction_normal() {
        assert_eq!(interpret("2 - 1"), Value::ValNumber(1.0));
    }

    #[test]
    fn test_subtraction_zeros() {
        assert_eq!(interpret("0 - 0"), Value::ValNumber(0.0));
    }

    #[test]
    fn test_subtraction_long_chain_ones() {
        // "1 - 1 - 1 - ... - 1" with 1,000 terms
        let source = vec!["1"; 1_000].join(" - ");
        assert_eq!(interpret(&source), Value::ValNumber(-998.0));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(interpret("1 + 2 * 3"), Value::ValNumber(7.0));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(interpret("(1 + 2) * 3"), Value::ValNumber(9.0));
    }

    #[test]
    fn test_left_associative() {
        assert_eq!(interpret("10 - 2 - 3"), Value::ValNumber(5.0));
    }

    #[test]
    fn test_unary_negation() {
        assert_eq!(interpret("-5"), Value::ValNumber(-5.0));
    }

    #[test]
    fn test_complex() {
        assert_eq!(interpret("(8 - 3) * (2 + 2)"), Value::ValNumber(20.0));
    }

    #[test]
    fn test_single_number() {
        assert_eq!(interpret("42"), Value::ValNumber(42.0));
    }

    #[test]
    fn test_multiplication_normal() {
        assert_eq!(interpret("3 * 4"), Value::ValNumber(12.0));
    }

    #[test]
    fn test_multiplication_zeros() {
        assert_eq!(interpret("3 * 0"), Value::ValNumber(0.0));
    }

    #[test]
    fn test_multiplication_long_chain_ones() {
        // "1 * 1 * 1 * ... * 1" with 1,000 terms
        let source = vec!["1"; 1_000].join(" * ");
        assert_eq!(interpret(&source), Value::ValNumber(1.0));
    }

    #[test]
    fn test_division_normal() {
        assert_eq!(interpret("2 / 1"), Value::ValNumber(2.0));
    }

    #[test]
    fn test_division_zeros() {
        match interpret("10 / 0") {
            Value::ValNumber(n) => {
                assert!(n.is_infinite());
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_negation() {
        assert_eq!(interpret("-5"), Value::ValNumber(-5.0));
    }

    #[test]
    fn test_nested_parens() {
        assert_eq!(interpret("((1 + 2))"), Value::ValNumber(3.0));
    }

    #[test]
    fn test_complex_expression() {
        // (8 - 3) * (2 + 2) = 5 * 4 = 20
        assert_eq!(interpret("(8 - 3) * (2 + 2)"), Value::ValNumber(20.0));
    }

    #[test]
    fn test_left_associative_division() {
        // 24 / 4 / 2 = (24 / 4) / 2 = 3
        assert_eq!(interpret("24 / 4 / 2"), Value::ValNumber(3.0));
    }

    #[test]
    fn test_all_operators() {
        // 2 + 3 * 5 - 4 / 2 = 2 + 15 - 2 = 15
        assert_eq!(interpret("2 + 3 * 5 - 4 / 2"), Value::ValNumber(15.0));
    }

    #[test]
    fn test_negation_in_expression() {
        assert_eq!(interpret("-5 + 3"), Value::ValNumber(-2.0));
    }

    #[test]
    fn test_negation_after_operator() {
        assert_eq!(interpret("2 * -3"), Value::ValNumber(-6.0));
    }

    #[test]
    fn test_double_negation() {
        // Double negation (--5) should evaluate to 5.0
        assert_eq!(interpret("--5"), Value::ValNumber(5.0));
    }

    #[test]
    fn test_triple_negation() {
        // Triple negation (---5) should evaluate to -5.0
        assert_eq!(interpret("---5"), Value::ValNumber(-5.0));
    }

    #[test]
    fn test_deeply_nested_parentheses() {
        // Deeply nested parentheses (((42)))
        assert_eq!(interpret("(((42)))"), Value::ValNumber(42.0));
        // Deeply nested arithmetic (((1 + 2) * (3 + 4))) = 3 * 7 = 21.0
        assert_eq!(interpret("(((1 + 2) * (3 + 4)))"), Value::ValNumber(21.0));
    }

    #[test]
    fn test_division_and_precedence() {
        // Basic division
        assert_eq!(interpret("10 / 2"), Value::ValNumber(5.0));
        // Floating point result
        assert_eq!(interpret("5 / 2"), Value::ValNumber(2.5));
        // Mixed precedence with division: 10 + 20 / 4 - 2 = 10 + 5 - 2 = 13.0
        assert_eq!(interpret("10 + 20 / 4 - 2"), Value::ValNumber(13.0));
    }

    #[test]
    fn test_left_associativity_mixed() {
        // Left associativity for subtraction and division: 100 / 10 / 2 = 5.0
        assert_eq!(interpret("100 / 10 / 2"), Value::ValNumber(5.0));
        // 20 - 10 - 5 - 2 = ((20 - 10) - 5) - 2 = 3.0
        assert_eq!(interpret("20 - 10 - 5 - 2"), Value::ValNumber(3.0));
    }

    #[test]
    fn test_negation_with_parentheses_and_operators() {
        // -(5 + 3) * -2 = -8 * -2 = 16.0
        assert_eq!(interpret("-(5 + 3) * -2"), Value::ValNumber(16.0));
        // -(-5) = 5.0
        assert_eq!(interpret("-(-5)"), Value::ValNumber(5.0));
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
        assert!(
            parser.had_error,
            "Expected parser to flag an error for missing ')'"
        );
    }

    #[test]
    fn test_error_unexpected_trailing_tokens() {
        // Extra token at the end: 1 + 2 3;
        let tokens = vec![
            make_test_token(TokenType::TokenNumber, "1"),
            make_test_token(TokenType::TokenPlus, "+"),
            make_test_token(TokenType::TokenNumber, "2"),
            make_test_token(TokenType::TokenNumber, "3"), // Unexpected
            make_test_token(TokenType::TokenSemicolon, ";"),
            make_test_token(TokenType::TokenEof, ""),
        ];
        let mut parser = Parser::init_parser(tokens);
        let _ = parser.parse();
        assert!(
            parser.had_error,
            "Expected parser to flag an error for trailing tokens"
        );
    }

    #[test]
    fn test_error_empty_expression() {
        let tokens = vec![make_test_token(TokenType::TokenEof, "")];
        let mut parser = Parser::init_parser(tokens);
        let _ = parser.parse_expression();
        assert!(
            parser.had_error,
            "Expected parser to flag an error for an empty expression"
        );
    }

    #[test]
    fn test_division_by_zero() {
        match interpret("42.0 / 0.0") {
            Value::ValNumber(result) => {
                assert!(
                    result.is_infinite(),
                    "Division by zero should result in infinity"
                );
                assert!(result > 0.0);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_complex_precedence_and_unary() {
        // -2 * (3 + -5) / 2 = -2 * (-2) / 2 = 4 / 2 = 2.0
        assert_eq!(interpret("-2 * (3 + -5) / 2"), Value::ValNumber(2.0));
    }

    #[test]
    fn test_true() {
        assert_eq!(interpret("true"), Value::ValBool(true));
    }

    #[test]
    fn test_false() {
        assert_eq!(interpret("false"), Value::ValBool(false));
    }

    #[test]
    fn test_nil() {
        assert_eq!(interpret("nil"), Value::ValNil);
    }

    #[test]
    fn test_not_true() {
        assert_eq!(interpret("!true"), Value::ValBool(false));
    }

    #[test]
    fn test_not_false() {
        assert_eq!(interpret("!false"), Value::ValBool(true));
    }

    #[test]
    fn test_not_nil() {
        assert_eq!(interpret("!nil"), Value::ValBool(true));
    }

    #[test]
    fn test_not_number() {
        assert_eq!(interpret("!3"), Value::ValBool(false));
    }

    #[test]
    fn test_equal_numbers() {
        assert_eq!(interpret("3 == 3"), Value::ValBool(true));
    }

    #[test]
    fn test_not_equal_numbers() {
        assert_eq!(interpret("3 != 4"), Value::ValBool(true));
    }

    #[test]
    fn test_greater() {
        assert_eq!(interpret("5 > 3"), Value::ValBool(true));
    }

    #[test]
    fn test_less_equal() {
        assert_eq!(interpret("3 <= 3"), Value::ValBool(true));
    }

    #[test]
    fn test_nil_equals_nil() {
        assert_eq!(interpret("nil == nil"), Value::ValBool(true));
    }

    #[test]
    fn test_mixed_type_not_equal() {
        assert_eq!(interpret("3 == true"), Value::ValBool(false));
    }

    #[test]
    fn test_arithmetic_then_compare() {
        assert_eq!(interpret("2 + 3 > 4"), Value::ValBool(true));
    }

    // --- Assignment 05 Statement & Global Variable Tests ---

    #[test]
    fn test_var_declaration() {
        let mut env = Environment::new();
        let stmts = parse_source("var x = 42;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("x"), Value::ValNumber(42.0));
    }

    #[test]
    fn test_var_default_nil() {
        let mut env = Environment::new();
        let stmts = parse_source("var y;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("y"), Value::ValNil);
    }

    #[test]
    fn test_var_in_expression() {
        let mut env = Environment::new();
        let stmts = parse_source("var x = 10; var y = x + 5;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("y"), Value::ValNumber(15.0));
    }

    #[test]
    fn test_multiple_variables() {
        let mut env = Environment::new();
        let stmts = parse_source("var a = 3; var b = 4;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("a"), Value::ValNumber(3.0));
        assert_eq!(env.get("b"), Value::ValNumber(4.0));
    }

    #[test]
    fn test_print_does_not_crash() {
        let mut env = Environment::new();
        let stmts = parse_source("print 1 + 2;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        // print produces output but doesn't store a value
    }

    #[test]
    fn test_expression_statement() {
        let mut env = Environment::new();
        let stmts = parse_source("1 + 2;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        // expression statement evaluates and discards
    }

    #[test]
    fn test_var_with_bool() {
        let mut env = Environment::new();
        let stmts = parse_source("var flag = true;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("flag"), Value::ValBool(true));
    }

    #[test]
    fn test_var_with_comparison() {
        let mut env = Environment::new();
        let stmts = parse_source("var x = 10; var big = x > 5;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
        assert_eq!(env.get("big"), Value::ValBool(true));
    }

    #[test]
    #[should_panic(expected = "Undefined variable")]
    fn test_undefined_variable() {
        let mut env = Environment::new();
        let stmts = parse_source("print z;");
        for stmt in &stmts {
            execute(stmt, &mut env);
        }
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
    println!("\x1b[35m==================== Arithmetic Tests ====================\x1b[0m");
    println!("1 + 2 * 3 = {}", interpret("1 + 2 * 3"));
    println!("(1 + 2) * 3 = {}", interpret("(1 + 2) * 3"));

    println!("\n\x1b[35m============= Booleans, Nil, & Negation =============\x1b[0m");
    println!("true = {}", interpret("true"));
    println!("false = {}", interpret("false"));
    println!("nil = {}", interpret("nil"));
    println!("!false = {}", interpret("!false"));
    println!("!nil = {}", interpret("!nil"));

    println!("\n\x1b[35m=============== Comparisons & Equalities ===============\x1b[0m");
    println!("1 < 2 = {}", interpret("1 < 2"));
    println!("3 <= 3 = {}", interpret("3 <= 3"));
    println!("5 > 3 = {}", interpret("5 > 3"));
    println!("3 == 3 = {}", interpret("3 == 3"));
    println!("3 != 4 = {}", interpret("3 != 4"));
    println!("2 + 3 > 4 = {}", interpret("2 + 3 > 4"));

    println!("\n\x1b[35m============= Statements & Global Variables =============\x1b[0m");
    run("var x = 10; var y = 20; print x + y;");

    println!("\n\x1b[35m==================== Error Handling ====================\x1b[0m");
    let source = "(1 + 2;";
    println!("Parsing invalid expression: \"{}\"", source);
    let tokens = scan_all(source);
    let mut parser = Parser::init_parser(tokens);
    let _stmts = parser.parse();
    if parser.had_error {
        println!("\x1b[32m-> Parser successfully flagged `had_error = true` for the syntax error!\x1b[0m");
    } else {
        println!("\x1b[31m-> Parser failed to flag `had_error = false` for the syntax error!\x1b[0m");
    }
}

// ================================================================================================
// END Main Function
// ================================================================================================