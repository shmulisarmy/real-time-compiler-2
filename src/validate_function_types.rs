use compiler_11::{ast::{Expression, FunctionCall, FunctionDef, OperatorUse, ValidInFunctionBody}, data_type::DataType, lexer::token::TokenType};

use crate::{file::File, get_type::HasType};



impl<'compilation_unit> File<'compilation_unit> {
    
    fn validate_function_types(&self, function: &FunctionDef<'compilation_unit>) {
        for function_body_piece in function.body.iter() {
            match function_body_piece {
                ValidInFunctionBody::Variable(variable) => {
                    match &variable.value {
                        Some(value) => {
                            if value.get_type(self) != variable.type_ {
                                panic!("Type mismatch: expected {} but got {} on variable {}", variable.type_, value.get_type(self), variable.name);
                            }
                        }
                        None => {}
                    }
                }
                ValidInFunctionBody::Expression(expression) => {
                    expression.get_type(self); //this will recursively validate the expressions type based off the types used within
                }
                ValidInFunctionBody::Return(expression) => {
                    let expression_type = expression.get_type(self);
                    if expression_type != function.return_type {
                        panic!("in function {} type of return statement {} does not match return type {}", function.name, expression_type, function.return_type);
                    }
                }
            }
        }
    }

    pub fn validate_functions(&self) {
        for function in self.functions.values() {
            self.validate_function_types(function);
        }
    }
}