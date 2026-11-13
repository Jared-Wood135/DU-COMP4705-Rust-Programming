//! FILE OVERVIEW:
//! - File for COMP3705 Assignment 05
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
//! - Lab 08 / Assignment 05 Functions
//!     - repl
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// io crate
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};

// Lox Interpreter
use assignment05_global_vars::expr::{Environment, Value, evaluate, execute};
use assignment05_global_vars::parser::{Parser, run, scan_all};
use assignment05_global_vars::scanner::TokenType;

// ----- Global Variables -------------------------------------------------------------------------
// None

// ================================================================================================
// END File Overview, Imports, Global Variables
// START Lab 08 / Assignment 05 Functions
// ================================================================================================

/// About
/// -----
/// - Print a prompt (e.g., >) and wait for the user to type a line.
///     1. Read the line of input.
///     2. Scan, parse, and evaluate/execute the input (handling both statements ending with `;` and expressions without `;`).
///     3. Maintain a persistent `Environment` across REPL lines so variables defined on one line can be used on the next.
///     4. Repeat until the user types quit or presses Ctrl-D (end of input).
///     5. If the user enters something that causes a parse error or runtime error, print an error message and continue (not crash).
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
/// - Lox Interpreter REPL
fn repl() {
    let mut input = String::new();
    let mut env = Environment::new();
    let mut last_expression = String::from("None");
    let mut last_result = Value::ValNil;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        // Ctrl-D (end of input)
        if bytes_read == 0 {
            println!();
            break;
        }

        // Make line a mutable, owned String so multiline appends work
        let mut line = input.trim().to_string();

        if line.is_empty() {
            continue;
        }

        if line == "quit" {
            break;
        }

        // Help command
        if line == "help" {
            println!("Available Commands & Features:");
            println!("  - Any valid Lox statement (e.g., var x = 10;, print x + 5;)");
            println!("  - Any valid Lox expression (e.g., 1 + 2 * 3, x + 5, true == true)");
            println!("  - last  : Prints the previous expression and its evaluated result");
            println!("  - help  : Displays this help menu");
            println!("  - quit  : Exits the REPL");
            println!("  - Multiline: Automatically continues if parentheses are unclosed.");
            continue;
        }

        // History (`last` command)
        if line == "last" {
            println!("Last Expression: {}", last_expression);
            println!("Result: {}", last_result);
            continue;
        }

        // Multiline input for unclosed parentheses
        while line.matches('(').count() > line.matches(')').count() {
            print!(". ");
            io::stdout().flush().unwrap();
            let mut next_line = String::new();
            let next_bytes = io::stdin().read_line(&mut next_line).unwrap();
            if next_bytes == 0 {
                break;
            }
            line.push(' ');
            line.push_str(next_line.trim());
        }

        let tokens = scan_all(&line);
        let mut parser = Parser::init_parser(tokens);

        if line.ends_with(';') {
            // Treat as a statement (or series of statements)
            let statements = parser.parse();
            if parser.had_error {
                continue;
            }

            let exec_result = panic::catch_unwind(AssertUnwindSafe(|| {
                for stmt in &statements {
                    execute(stmt, &mut env);
                }
            }));

            if let Err(err) = exec_result {
                if let Some(msg) = err.downcast_ref::<&str>() {
                    eprintln!("Runtime Error: {}", msg);
                } else if let Some(msg) = err.downcast_ref::<String>() {
                    eprintln!("Runtime Error: {}", msg);
                } else {
                    eprintln!("Runtime Error occurred.");
                }
            }
        } else {
            // Treat as an expression without ';' — evaluate and print the result
            let expr = parser.parse_expression();
            if !parser.had_error {
                parser.consume(
                    TokenType::TokenEof,
                    "Expected ';' or end of expression.",
                );
            }
            if parser.had_error {
                continue;
            }

            let eval_result = panic::catch_unwind(AssertUnwindSafe(|| evaluate(&expr, &env)));
            match eval_result {
                Ok(result) => {
                    println!("{}", result);
                    last_expression = line.to_string();
                    last_result = result;
                }
                Err(err) => {
                    if let Some(msg) = err.downcast_ref::<&str>() {
                        eprintln!("Runtime Error: {}", msg);
                    } else if let Some(msg) = err.downcast_ref::<String>() {
                        eprintln!("Runtime Error: {}", msg);
                    } else {
                        eprintln!("Runtime Error occurred.");
                    }
                }
            }
        }
    }
}

// ================================================================================================
// END Lab 08 / Assignment 05 Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Runs a Lox source file if provided as an argument, or launches the REPL by default
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Run file if filename argument is provided
        let filename = &args[1];
        match std::fs::read_to_string(filename) {
            Ok(contents) => run(&contents),
            Err(e) => eprintln!("Could not read file '{}': {}", filename, e),
        }
    } else {
        // Run REPL by default when no arguments are given
        repl();
    }
}

// ================================================================================================
// END Main Function
// ================================================================================================