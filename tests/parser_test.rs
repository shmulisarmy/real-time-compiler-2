use compiler_11::{
    ast::{
        AstComparable, Expression, FunctionCall, FunctionDef, OperatorUse,
        ValidInFunctionBody, Variable,
    },
    data_type::DataType,
    lexer::token::{Token, TokenType},
    parser::Parser,
};

// Helper macro to assert equality with detailed error reporting
macro_rules! assert_ast_eq {
    ($left:expr, $right:expr) => {
        match $left.compare(&$right) {
            Ok(_) => {}
            Err(errors) => {
                let mut msg = String::from("AST comparison failed. Differences found:\n");
                for error in errors {
                    msg.push_str(&format!("- {}\n", error));
                }
                panic!("{}", msg);
            }
        }
    };
}

// Helper function to create a simple token
fn token(ty: TokenType, value: &str) -> Token {
    Token {
        type_: ty,
        value: value.to_string(),
    }
}

// Helper function to create a simple expression
fn expr_token(ty: TokenType, value: &str) -> Expression {
    Expression::Token(token(ty, value))
}

#[test]
fn test_parse_variable_declaration() {
    let mut p = Parser::new("var num int = 10".to_string());
    p.tokenizer.expect(TokenType::Keyword);
    let var = p.parse_var();
    assert_ast_eq!(var.name, "num".to_string());
    assert_ast_eq!(var.type_, DataType::Int);
    if let Some(Expression::Token(token)) = var.value {
        assert_ast_eq!(token.value, "10".to_string());
        assert_eq!(token.value, "10");
    } else {
        panic!("Expected a value");
    }
}

#[test]
fn test_parse_function() {
    let code = "my_func(a int, b int,): int { return a + b }";
    let mut p = Parser::new(code.to_string());
    let function = p.parse_function();

    // Define the expected AST structure
    let expected = FunctionDef {
        name: "my_func".to_string(),
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
        body: vec![ValidInFunctionBody::Return(Expression::OperatorUse(
            OperatorUse {
                operator: "+".to_string(),
                left: Box::new(expr_token(TokenType::Identifier, "a")),
                right: Box::new(expr_token(TokenType::Identifier, "b")),
            },
        ))],
    };

    // Compare the actual and expected ASTs
    assert_ast_eq!(&function, &expected);

    // Define the expected function structure
    let expected = FunctionDef {
        name: "my_func".to_string(),
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
        body: vec![ValidInFunctionBody::Return(Expression::OperatorUse(
            OperatorUse {
                operator: "+".to_string(),
                left: Box::new(expr_token(TokenType::Identifier, "a")),
                right: Box::new(expr_token(TokenType::Identifier, "b")),
            },
        ))],
    };

    // Compare the actual and expected ASTs
    assert_ast_eq!(&function, &expected);
}

#[test]
fn test_parse_function_header() {
    let code = "my_func(a int, b int,): int";
    let mut p = Parser::new(code.to_string());
    let (name, args, return_type) = p.parse_function_header();

    // Verify the parsed header
    assert_eq!(name, "my_func");
    assert_eq!(args.len(), 2);

    // Create expected variables for comparison
    let expected_args = vec![
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
    ];

    // Compare each argument
    for (actual, expected) in args.iter().zip(expected_args.iter()) {
        assert_ast_eq!(actual, expected);
    }

    assert_eq!(return_type, DataType::Int);
}

#[test]
fn test_parse_complex_expression() {
    let code = "var result int = (a + b) * (c - d) / e";
    let mut p = Parser::new(code.to_string());
    p.tokenizer.expect(TokenType::Keyword);
    let var = p.parse_var();

    // Verify the variable structure
    assert_eq!(var.name, "result");
    assert_eq!(var.type_, DataType::Int);

    // Define the expected AST structure
    let expected = Expression::OperatorUse(OperatorUse {
        operator: "/".to_string(),
        left: Box::new(Expression::OperatorUse(OperatorUse {
            operator: "*".to_string(),
            left: Box::new(Expression::OperatorUse(OperatorUse {
                operator: "+".to_string(),
                left: Box::new(expr_token(TokenType::Identifier, "a")),
                right: Box::new(expr_token(TokenType::Identifier, "b")),
            })),
            right: Box::new(Expression::OperatorUse(OperatorUse {
                operator: "-".to_string(),
                left: Box::new(expr_token(TokenType::Identifier, "c")),
                right: Box::new(expr_token(TokenType::Identifier, "d")),
            })),
        })),
        right: Box::new(expr_token(TokenType::Identifier, "e")),
    });

    // Compare the actual and expected ASTs
    assert_ast_eq!(var.value.unwrap(), expected);
}

#[test]
fn test_parse_var_with_value() {
    let code = "var result bool = a == b";
    let mut p = Parser::new(code.to_string());
    p.tokenizer.expect(TokenType::Keyword);
    let var = p.parse_var();

    // Define the expected AST structure based on actual parser behavior
    let expected_value = Expression::OperatorUse(OperatorUse {
        operator: "==".to_string(),
        left: Box::new(expr_token(TokenType::Identifier, "a")),
        right: Box::new(expr_token(TokenType::Identifier, "b")),
    });

    assert_ast_eq!(var.value.unwrap(), expected_value);
}

#[test]
fn test_parse_function_call_expression() {
    let code = "result = max(a + b, c * d,)";
    let mut p = Parser::new(code.to_string());
    let parsed_expression = p.parse_expression(0);

    // Define the expected AST structure based on actual parser behavior
    let expected_expression: Expression = Expression::OperatorUse(OperatorUse {
        operator: "=".to_string(),
        left: Box::new(expr_token(TokenType::Identifier, "result")),
        right: Box::new(Expression::FunctionCall(FunctionCall {
            name: "max".to_string(),
            args: vec![
                Expression::OperatorUse(OperatorUse {
                    operator: "+".to_string(),
                    left: Box::new(expr_token(TokenType::Identifier, "a")),
                    right: Box::new(expr_token(TokenType::Identifier, "b")),
                }),
                Expression::OperatorUse(OperatorUse {
                    operator: "*".to_string(),
                    left: Box::new(expr_token(TokenType::Identifier, "c")),
                    right: Box::new(expr_token(TokenType::Identifier, "d")),
                }),
            ],
        })),
    });

    assert_ast_eq!(parsed_expression, expected_expression);
}

#[test]
fn test_parse_var() {
    let code = "var result int = a + b";
    let mut p = Parser::new(code.to_string());
    p.tokenizer.expect(TokenType::Keyword);
    let parsed_var = p.parse_var();

    // Define the expected AST structure based on actual parser behavior
    let expected_var = Variable {
        name: "result".to_string(),
        type_: DataType::Int,
        value: Some(Expression::OperatorUse(OperatorUse {
            operator: "+".to_string(),
            left: Box::new(expr_token(TokenType::Identifier, "a")),
            right: Box::new(expr_token(TokenType::Identifier, "b")),
        })),
    };

    assert_ast_eq!(parsed_var, expected_var);
}

#[test]
fn test_parse_function2() {
    let code = "func main() { send_message() }";
    let mut p = Parser::new(code.to_string());
    p.tokenizer.expect(TokenType::Keyword);
    let parsed_function = p.parse_function();

    // Define the expected AST structure based on actual parser behavior
    let expected_function = FunctionDef {
        name: "main".to_string(),
        args: vec![],
        return_type: DataType::None,
        body: vec![ValidInFunctionBody::Expression(Expression::FunctionCall(FunctionCall {
            name: "send_message".to_string(),
            args: vec![],
        }))],
    };

    assert_ast_eq!(parsed_function, expected_function);
}
