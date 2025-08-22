use std::collections::HashMap;

use compiler_11::{
    lexer::token::TokenType,
    parser::Parser,
    ast::{FunctionDef, Variable},
};

pub struct File {
    name: String,
    functions: HashMap<String, FunctionDef>,
    variables: HashMap<String, Variable>,
}

impl File {
    pub fn parse(source: String) -> File {
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
            functions,
            variables,
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
        let file = File::parse(code.to_string());
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