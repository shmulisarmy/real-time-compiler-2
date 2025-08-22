use crate::{
    data_type::DataType,
    ast::{Expression, OperatorUse},
    lexer::token::TokenType,
};



impl<'a>  OperatorUse<'a> {
    pub fn get_type(&self) -> DataType {
        let lefts_type = self.left.get_type();
        let rights_type = self.right.get_type();
        if lefts_type != rights_type {
            panic!("Type mismatch: {} and {}", lefts_type, rights_type);
        }
        lefts_type
    }
}




impl<'a>  Expression<'a> {
    pub fn get_type(&self) -> DataType {
        match self {
            Expression::OperatorUse(op) => op.get_type(),
            Expression::Token(token) => match token.type_ {
                TokenType::Number => DataType::Int,
                TokenType::String => DataType::String,
                _ => panic!("Unknown token type: {}", token.type_)
            },
            // Expression::FunctionCall(call) => call.get_type(),
            // Expression::VarReference(reference) => reference.referring_to.type_,
            _ => panic!("Unknown expression type: {}", self),
        }
    }
}