use colored::*;
use compiler_11::{
    lexer::token::TokenType,
    parser::Parser,
};


mod file;
use crate::file::File;

// Enable colored output
use std::io::Write;
use std::sync::Once;

static INIT: Once = Once::new();

fn init_colors() {
    INIT.call_once(|| {
        colored::control::set_override(true);
    });
}

fn main() {
    color_backtrace::install();
    // Initialize colored output
    init_colors();
    // Example 1: Variable declaration
    let code = r#"
        var num int = b * 9 + a(a, 3*0,)
        
        func add(a int, b int,): int {
            var result int = a + b * 7 + "hello"
            return result
        }
    "#;

    let mut p = Parser::new(code.to_string());

    p.tokenizer.eat_lines();
    // Parse and print the variable declaration
    p.tokenizer.expect(TokenType::Keyword);
    let var = p.parse_var();
    println!("Variable declaration:");
    println!("  {}", var);
    println!();

    p.tokenizer.eat_lines();

    // Parse and print the function definition
    p.tokenizer.expect(TokenType::Keyword);
    let func = p.parse_function();
    println!("Function definition:");
    println!("{}", func);
}
