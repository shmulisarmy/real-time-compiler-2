use std::option;

use compiler_11::{ast::{Expression, FunctionCall, OperatorUse}, data_type::DataType, lexer::token::TokenType};

use crate::{file::File, in_function_scope_validation::find_var_type_from_local_scope, scope_placement_info::ScopePlacementInfo};

 



pub trait HasType<'compilation_unit> {
    fn get_type(&self, file: &File<'compilation_unit>, scope_placement_info: &'compilation_unit Option<ScopePlacementInfo<'compilation_unit>>) -> DataType;
}



impl<'compilation_unit>HasType<'compilation_unit> for OperatorUse<'compilation_unit>  {
    fn get_type(&self, file: &File<'compilation_unit>, scope_placement_info: &'compilation_unit Option<ScopePlacementInfo<'compilation_unit>>) -> DataType {
        let lefts_type = self.left.get_type(file, scope_placement_info);
        let rights_type = self.right.get_type(file, scope_placement_info);
        if lefts_type != rights_type {
            panic!("Type mismatch: {} and {}", lefts_type, rights_type);
        }
        lefts_type
    }
}




impl<'compilation_unit> HasType<'compilation_unit> for FunctionCall<'compilation_unit> {
    fn get_type(&self, file: &File<'compilation_unit>, scope_placement_info: &'compilation_unit Option<ScopePlacementInfo<'compilation_unit>>) -> DataType {
        let function = file.functions.get(&self.name).unwrap();
        if self.args.len() != function.args.len() {
            panic!("Argument count mismatch: {} and {} when trying to call function {}", self.args.len(), function.args.len(), self.name);
        }
        for (i, arg) in self.args.iter().enumerate() {
            let func_arg = &function.args[i];
            if arg.get_type(file, scope_placement_info) != func_arg.type_ {
                panic!("Type mismatch: expected {} but got {} on arg {} when trying to call function {}", func_arg.type_, arg.get_type(file, scope_placement_info), i+1, self.name);
            }
        }
        function.return_type.clone() //@optimize
    }
}

impl<'compilation_unit> HasType<'compilation_unit> for Expression<'compilation_unit> {
    fn get_type(&self, file: &File<'compilation_unit>, scope_placement_info: &'compilation_unit Option<ScopePlacementInfo<'compilation_unit>>) -> DataType {
        match self {
            Expression::OperatorUse(op) => op.get_type(file, scope_placement_info),
            Expression::Token(token) => match token.type_ {
                TokenType::Number => DataType::Int,
                TokenType::String => DataType::String,
                _ => panic!("Unknown token type: {}", token.type_)
            },
            Expression::FunctionCall(call) => call.get_type(file, scope_placement_info),
            Expression::VarReference(reference) => {
                if scope_placement_info.is_some() {
                    let scope_placement_info = scope_placement_info.as_ref().unwrap();
                    let variable_type = find_var_type_from_local_scope(&reference.name, scope_placement_info);
                    if variable_type.is_some() {
                        return variable_type.unwrap();
                    }
                }
                let variable = file.variables.get(&reference.name)
                    .unwrap_or_else(|| panic!("Variable {} not found", reference.name));
                variable.type_.clone()
            },
            // _ => panic!("Unknown expression type: {}", self),
        }
    }
}