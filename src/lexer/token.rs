use crate::ast::{AstComparable, ComparisonError};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    Number,
    String,
    Punctuation,
    Operator,
    Keyword,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::Number => write!(f, "number"),
            TokenType::String => write!(f, "string"),
            TokenType::Punctuation => write!(f, "punctuation"),
            TokenType::Operator => write!(f, "operator"),
            TokenType::Keyword => write!(f, "keyword"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AstComparable for Token {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        let mut errors = Vec::new();

        if self.type_ != other.type_ {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Token type mismatch: expected {:?}, got {:?}",
                self.type_, other.type_
            )));
        }

        if self.value != other.value {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Token value mismatch: expected '{}', got '{}'",
                self.value, other.value
            )));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl AstComparable for TokenType {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        if self == other {
            Ok(())
        } else {
            Err(vec![ComparisonError::MismatchedValues(format!(
                "Expected token type '{:?}', but got '{:?}'",
                self, other
            ))])
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
}
