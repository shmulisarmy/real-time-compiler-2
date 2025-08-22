use compiler_11::{
    ast::{
        Expression, FunctionCall, FunctionDef, OperatorUse,
        ValidInFunctionBody, Variable,
    },
    data_type::DataType,
    lexer::{token::Token, token::TokenType},
};


#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;
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