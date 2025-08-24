use crate::{ast::StructDef, lexer::token::TokenType};
use colored::*;
use std::fmt;

use super::structure::{Expression, FunctionCall, FunctionDef, OperatorUse, ValidInFunctionBody, Variable};

fn format_type(s: &str) -> String { s.custom_color((80, 205, 150)).to_string() }
fn format_keyword(s: &str) -> String { s.custom_color((20, 0, 205)).bold().to_string() }
fn format_identifier(s: &str) -> String { s.custom_color((204, 204, 0)).to_string() }
fn format_number(s: &str) -> String { s.white().to_string() }
fn format_operator(s: &str) -> String { s.white().to_string() }
fn format_string(s: &str) -> String { s.custom_color((255, 195, 50)).to_string() }

fn indent(f: &mut std::fmt::Formatter, depth: usize) -> fmt::Result {
    write!(f, "{:indent$}", "", indent = depth * 2)
}

impl<'a> fmt::Display for FunctionCall<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", format_identifier(&self.name))?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 { write!(f, "{}", ", ".white())?; }
            write!(f, "{}", arg)?;
        }
        write!(f, "{}", ")".white())
    }
}

impl<'a> fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = &self.value {
            write!(f, "{} {} {} = {}",
                format_keyword("var"),
                format_identifier(&self.name),
                format_type(&self.type_.to_string()),
                expr)?;
        } else {
            write!(f, "{} {} {}", format_keyword("var"), format_identifier(&self.name), format_type(&self.type_.to_string()))?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for OperatorUse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, format_operator(&self.operator), self.right)
    }
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::OperatorUse(op) => write!(f, "{}", op),
            Expression::Token(t) => match t.type_ {
                TokenType::Number => write!(f, "{}", format_number(&t.value)),
                TokenType::String => write!(f, "\"{}\"", format_string(&t.value)),
                TokenType::Keyword => write!(f, "{}", format_keyword(&t.value)),
                _ => write!(f, "{}", format_identifier(&t.value)),
            },
            Expression::FunctionCall(func) => write!(f, "{}", func),
            Expression::VarReference(var_ref) => write!(f, "{}", format_identifier(&var_ref.name)),
            Expression::Array(_) => write!(f, "{}", "array"),
            Expression::Object(_) => write!(f, "{}", "object"),
            Expression::Subscript(_) => write!(f, "{}", "subscript"),
        }
    }
}

impl<'a> fmt::Display for ValidInFunctionBody<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidInFunctionBody::Variable(var) => write!(f, "{}", var),
            ValidInFunctionBody::Expression(expr) => write!(f, "{}", expr),
            ValidInFunctionBody::Return(expr) => write!(f, "{} {}", format_keyword("return"), expr),
        }
    }
}

impl<'a> fmt::Display for FunctionDef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Function signature
        write!(
            f,
            "{} {} (",
            format_keyword("func"),
            format_identifier(&self.name)
        )?;

        // Format arguments with proper coloring
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 { write!(f, "{}", ", ".white())?; }
            write!(f, "{}: {}", format_identifier(&arg.name), format_type(&arg.type_.to_string()))?;
        }

        // Return type and opening brace
        write!(f, ") {} {} {{\n", "->".white(), format_type(&self.return_type.to_string()))?;

        // Function body with proper indentation
        for stmt in &self.body {
            write!(f, "    {}\n", stmt)?;
        }

        // Closing brace
        write!(f, "}}")
    }
}




impl<'a> fmt::Display for StructDef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {{\n", format_keyword("struct"), format_identifier(&self.name))?;

        for field in &self.fields {
            write!(f, "    {}\n", field)?;
        }

        write!(f, "}}")
    }
}
