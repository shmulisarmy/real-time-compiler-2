use std::collections::HashMap;

use compiler_11::{
    lexer::token::TokenType,
    parser::Parser,
    ast::{FunctionDef, Variable, Expression},
    data_type::DataType,
    
};
use crate::get_type::HasType;



pub struct File<'a> {
    pub name: String,
    source: &'a str,
    pub functions: HashMap<String, FunctionDef<'a>>,
    pub variables: HashMap<String, Variable<'a>>,
}


impl<'a> File<'a> {
    pub fn parse(source: &'a str) -> File<'a> {
        
        let mut parser = Parser::new(source);
        let mut functions = HashMap::new();
        let mut variables = HashMap::new();
        parser.tokenizer.eat_lines();
        let token_start_pos = parser.tokenizer.index;
        while let Some(token) = parser.tokenizer.next() {
            match token.type_ {
                TokenType::Keyword => {
                    match token.value.as_str() {
                        "func" => {
                            let function = parser.parse_function();
                            functions.insert(function.name.clone(), function);
                        }
                        "var" => {
                            let variable = parser.parse_var();
                            variables.insert(variable.name.clone(), variable);
                        }
                        _ => parser.tokenizer.show_user_error(token_start_pos, token_start_pos+token.value.len(), "not implemented".to_string())
                    }
                }
                _ => parser.tokenizer.show_user_error(token_start_pos, token_start_pos+token.value.len(), "not implemented".to_string())
            }
            parser.tokenizer.eat_lines();
        }
        File {
            name: "main".to_string(),
            source,
            functions,
            variables,
        }   
    }



    pub fn type_inference(&self) {
        for variable in self.variables.values() {
            match &variable.value {
                Some(value) => assert_eq!(variable.type_, value.get_type(self), "Variable {} has type {} but value {}", variable.name, variable.type_, value.get_type(self)),
                None => {assert_ne!(variable.type_, DataType::None, "Variable {} has no type or default value to infer type", variable.name)}
            }
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use compiler_11::data_type::DataType;
    use compiler_11::ast::{Expression, FunctionCall, OperatorUse, ValidInFunctionBody};
    use compiler_11::lexer::token::Token;

    #[test]
    fn test_parse_file() {
        let code = "func add(a int, b int,): int { return a + b } \n var result int = add(1, 2,)";
        let file = File::parse(code);
        let parsed_add_function = file.functions.get("add").unwrap();

        let expected_add_function = FunctionDef {
            name: "add".to_string(),
            args: vec![
                Variable {
                    name: "a".to_string(),
                    type_: DataType::Int,
                    value: None,
                },
                Variable {
                    name: "b".to_string(),
                    type_: DataType::Int,
                    value: None,
                },
            ],
            return_type: DataType::Int,
            body: vec![
                ValidInFunctionBody::Return(
                    Expression::OperatorUse(
                        OperatorUse{
                            left: Box::new(Expression::Token(Token {
                                type_: TokenType::Identifier,
                                value: "a".to_string(),
                            })),
                            operator: "+".to_string(),
                            right: Box::new(Expression::Token(Token {
                                type_: TokenType::Identifier,
                                value: "b".to_string(),
                            }))
                        }
                    ),
                ),
            ],
        };


        let expected_result_variable = Variable {
            name: "result".to_string(),
            type_: DataType::Int,
            value: Some(Expression::FunctionCall(FunctionCall {
                name: "add".to_string(),
                args: vec![
                    Expression::Token(Token {
                        type_: TokenType::Number,
                        value: "1".to_string(),
                    }),
                    Expression::Token(Token {
                        type_: TokenType::Number,
                        value: "2".to_string(),
                    }),
                ],
            })),
        };
        assert_eq!(parsed_add_function, &expected_add_function);
        assert_eq!(file.variables.get("result").unwrap(), &expected_result_variable);
    }
}