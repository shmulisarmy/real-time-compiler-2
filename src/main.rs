use colored::*;
use compiler_11::{
    lexer::token::TokenType,
    parser::Parser,
};

mod in_function_scope_validation;
mod scope_placement_info;
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
        var res int = add(1, n)

        Person{
            name string
            age int
        }
        
        func add(a int, b int): int {
            var result int = 7 + 5
            var z array = ["hello", "world"]
            var d int = a + b
            var person object = {name: "John", age: 30}
            return result
        }
        Car{ }
    "#;


    let file = File::parse(code);   
    // Parse and print the variable declaration
    println!("Variable declarations:");
    file.variables.values().for_each(|var| {
        println!("{}", var);
        println!();
    });

    // Parse and print the function definition
    println!("Function definitions:");
    file.functions.values().for_each(|func| {
        println!("{}", func);
        println!();
    });

    file.structs.values().for_each(|struct_| {
        println!("Struct definition:");
        println!("{}", struct_);
        println!();
    });
    file.validate_global_variable_types();
    file.validate_functions();
}
