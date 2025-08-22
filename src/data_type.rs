use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DataType {
    Int,
    String,
    None,
    Bool,
}


pub fn type_from(value: String) -> DataType {
    return match value.as_str() {
        "int" => DataType::Int,
        "string" => DataType::String,
        "bool" => DataType::Bool,
        _ => panic!("Unknown type {}", value),
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::String => write!(f, "string"),
            DataType::Bool => write!(f, "bool"),
            DataType::None => write!(f, "none"),
        }
    }
}
