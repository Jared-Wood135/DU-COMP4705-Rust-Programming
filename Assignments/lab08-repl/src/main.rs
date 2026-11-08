//! FILE OVERVIEW:
//! - File for COMP3705 Lab 08
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
//! - Lab 08 Functions
//!     - repl
//! - Main Function

// ----- Imports ----------------------------------------------------------------------------------
// io crate
use std::io::{self, Write};

// Lox Interpreter
mod expr;
mod scanner;
mod parser;
use expr::{Value, evaluate};
use parser::{Parser, scan_all};

// ----- Global Variables -------------------------------------------------------------------------


// ================================================================================================
// END File Overview, Imports, Global Variables
// START Lab 08 Functions
// ================================================================================================

/// About
/// -----
/// - Print a prompt (e.g., >) and wait for the user to type a line.
///     1. Read the line of input.
///     2. Scan, parse, and evaluate the input.
///     3. Print the result.
///     4. Repeat until the user types quit or presses Ctrl-D (end of input).
///     5. If the user enters something that causes a parse error or runtime error, the REPL should print an error message and continue (not crash).
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
    let mut last_expression = String::from("None");
    let mut last_result = Value::ValNil;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .unwrap();

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
            println!("  - Any valid Lox expression (e.g., 1 + 2 * 3, true == true, nil)");
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

        // scan, parse, evaluate, and print the result
        // 1. Pass &line (borrowed str) to scan_all
        let tokens = scan_all(&line);
        let mut parser = Parser::init_parser(tokens);
        
        // 2. Parse the tokens into an expression AST
        let expr = parser.parse();

        // 3. Check if any syntax/parse errors occurred
        if parser.had_error {
            continue;
        }

        // 4. Evaluate the expression and print the resulting Value
        let result = evaluate(&expr);
        println!("{}", result);

        // Update history tracking variables upon success
        last_expression = line.to_string();
        last_result = result;
    }
}

// ================================================================================================
// END Lab 08 Functions
// START Main Function
// ================================================================================================

/// About
/// -----
/// - Simply just runs the REPL and shows the proper functionality
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Run file if filename argument is provided
        let filename = &args[1];
        match std::fs::read_to_string(filename) {
            Ok(contents) => {
                let tokens = scan_all(&contents);
                let mut parser = Parser::init_parser(tokens);
                let expr = parser.parse();
                if !parser.had_error {
                    let result = evaluate(&expr);
                    println!("{}", result);
                }
            }
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