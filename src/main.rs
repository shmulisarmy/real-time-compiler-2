use colored::*;
use compiler_11::{
    lexer::token::TokenType,
    parser::Parser,
};
mod validate_function_types;
mod get_type;
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
        var n int = 1
        var num int = 3 - n
        var res string = add(1, 2,)
        var result string = "hello"
        
        func add(a int, b int,): string {
            var result int = 7 + 5
            return result
        }
    "#;


    let file = File::parse(code);   
    // Parse and print the variable declaration
    file.variables.values().for_each(|var| {
        println!("Variable declaration:");
        println!("  {}", var);
        println!();
    });

    // Parse and print the function definition
    file.functions.values().for_each(|func| {
        println!("Function definition:");
        println!("  {}", func);
        println!();
    });

    file.validate_global_variable_types();
    file.validate_functions();
}
