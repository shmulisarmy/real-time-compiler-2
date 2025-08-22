use crate::{
    data_type::DataType,
    lexer::token::Token,
};

use super::structure::{
    Expression,
    FunctionCall,
    FunctionDef,
    OperatorUse,
    ValidInFunctionBody,
    Variable,
};

// ---- Comparison utilities and traits ----
#[derive(Debug, PartialEq)]
pub enum ComparisonError {
    MismatchedTypes,
    MismatchedValues(String),
    MismatchedLengths(usize, usize),
    MismatchedVariant,
    Custom(String),
}

impl std::fmt::Display for ComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonError::MismatchedTypes => write!(f, "types do not match"),
            ComparisonError::MismatchedValues(details) => write!(f, "{}", details),
            ComparisonError::MismatchedLengths(expected, actual) => {
                write!(f, "length mismatch: expected {}, got {}", expected, actual)
            }
            ComparisonError::MismatchedVariant => write!(f, "enum variants do not match"),
            ComparisonError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ComparisonError {}

pub trait AstComparable: std::fmt::Debug {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>>;
}

fn compare_optional<T: AstComparable>(
    a: &Option<T>,
    b: &Option<T>,
    context: &str,
) -> Result<(), Vec<ComparisonError>> {
    match (a, b) {
        (Some(a_val), Some(b_val)) => a_val.compare(b_val).map_err(|mut errs| {
            errs.push(ComparisonError::Custom(format!("In {}: ", context)));
            errs
        }),
        (None, None) => Ok(()),
        _ => Err(vec![ComparisonError::MismatchedValues(format!(
            "One value is Some while the other is None in {}",
            context
        ))]),
    }
}

fn compare_vec<T: AstComparable>(
    a: &[T],
    b: &[T],
    context: &str,
) -> Result<(), Vec<ComparisonError>> {
    if a.len() != b.len() {
        return Err(vec![ComparisonError::MismatchedLengths(a.len(), b.len())]);
    }

    let mut errors = Vec::new();
    for (i, (a_item, b_item)) in a.iter().zip(b.iter()).enumerate() {
        if let Err(errs) = a_item.compare(b_item) {
            errors.extend(
                errs.into_iter().map(|e| {
                    ComparisonError::Custom(format!("At index {} in {}: {}", i, context, e))
                }),
            );
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

// ---- Primitive/leaf impls ----
impl AstComparable for String {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        if self == other { Ok(()) } else {
            Err(vec![ComparisonError::MismatchedValues(format!(
                "Expected '{}', but got '{}'",
                self, other
            ))])
        }
    }
}

impl AstComparable for DataType {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        if self == other { Ok(()) } else {
            Err(vec![ComparisonError::MismatchedValues(format!(
                "Expected type '{:?}', but got '{:?}'",
                self, other
            ))])
        }
    }
}

// ---- AST node comparison impls ----
impl<'a> AstComparable for FunctionCall<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        let mut errors = Vec::new();
        if self.name != other.name {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Function name mismatch: '{}' != '{}'",
                self.name, other.name
            )));
        }
        if let Err(errs) = compare_vec(&self.args, &other.args, "function arguments") {
            errors.extend(errs);
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl<'a> PartialEq for FunctionCall<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }

impl<'a> AstComparable for Variable<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        let mut errors = Vec::new();
        if self.name != other.name {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Variable name mismatch: '{}' != '{}'",
                self.name, other.name
            )));
        }
        if self.type_ != other.type_ {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Type mismatch for variable '{}': {:?} != {:?}",
                self.name, self.type_, other.type_
            )));
        }
        if let Err(errs) = compare_optional(&self.value, &other.value, "variable value") {
            errors.extend(errs);
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl<'a> PartialEq for Variable<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }

impl<'a> AstComparable for OperatorUse<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        let mut errors = Vec::new();
        if self.operator != other.operator {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Operator mismatch: '{}' != '{}'",
                self.operator, other.operator
            )));
        }
        if let Err(errs) = self.left.compare(&other.left) {
            errors.extend(errs.into_iter().map(|e| ComparisonError::Custom(format!("In left operand: {}", e))));
        }
        if let Err(errs) = self.right.compare(&other.right) {
            errors.extend(errs.into_iter().map(|e| ComparisonError::Custom(format!("In right operand: {}", e))));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl<'a> PartialEq for OperatorUse<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }

impl<'a> AstComparable for Expression<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        match (self, other) {
            (Expression::OperatorUse(a), Expression::OperatorUse(b)) => a.compare(b),
            (Expression::Token(a), Expression::Token(b)) => {
                if a.type_ != b.type_ || a.value != b.value {
                    Err(vec![ComparisonError::MismatchedValues(format!(
                        "Token mismatch: {:?} != {:?}", a, b
                    ))])
                } else { Ok(()) }
            }
            (Expression::FunctionCall(a), Expression::FunctionCall(b)) => a.compare(b),
            (Expression::VarReference(a), Expression::VarReference(b)) => {
                if a.name == b.name { Ok(()) } else { Err(vec![ComparisonError::MismatchedValues(format!("Var reference mismatch: '{}' != '{}'", a.name, b.name))]) }
            }
            _ => Err(vec![ComparisonError::MismatchedVariant]),
        }
    }
}

impl<'a> PartialEq for Expression<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }

impl<'a> AstComparable for ValidInFunctionBody<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        match (self, other) {
            (ValidInFunctionBody::Variable(a), ValidInFunctionBody::Variable(b)) => a.compare(b),
            (ValidInFunctionBody::Expression(a), ValidInFunctionBody::Expression(b)) => a.compare(b),
            (ValidInFunctionBody::Return(a), ValidInFunctionBody::Return(b)) => a.compare(b),
            _ => Err(vec![ComparisonError::MismatchedVariant]),
        }
    }
}

impl<'a> PartialEq for ValidInFunctionBody<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }

impl<'a> AstComparable for FunctionDef<'a> {
    fn compare(&self, other: &Self) -> Result<(), Vec<ComparisonError>> {
        let mut errors = Vec::new();
        if self.name != other.name {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Function name mismatch: '{}' != '{}'",
                self.name, other.name
            )));
        }
        if self.return_type != other.return_type {
            errors.push(ComparisonError::MismatchedValues(format!(
                "Return type mismatch: '{:?}' != '{:?}'",
                self.return_type, other.return_type
            )));
        }
        if let Err(errs) = compare_vec(&self.args, &other.args, "function arguments") {
            errors.extend(errs);
        }
        if let Err(errs) = compare_vec(&self.body, &other.body, "function body") {
            errors.extend(errs);
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl<'a> PartialEq for FunctionDef<'a> { fn eq(&self, other: &Self) -> bool { self.compare(other).is_ok() } }
