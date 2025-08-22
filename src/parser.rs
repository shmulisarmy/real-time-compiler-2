use crate::{
    ast::{structure::{Array, Object, StructDef, StructScopeItem, Subscript}, Expression, FunctionCall, FunctionDef, OperatorUse, ValidInFunctionBody, Variable},
    data_type::{type_from, DataType},
    lexer::{
        token::{self, TokenType},
        tokenizer::Tokenizer,
    },
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static OPERATOR_PRECEDENCE: Lazy<HashMap<&'static str, u32>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    hm.insert("=", 1);
    hm.insert("+", 2);
    hm.insert("-", 2);
    hm.insert("*", 3);
    hm.insert("/", 3);
    hm.insert("+=", 2);
    hm.insert("-=", 2);
    hm.insert("*=", 3);
    hm.insert("/=", 3);
    hm.insert("|", 4);
    hm.insert("==", 5);
    hm.insert("!=", 5);
    hm.insert(">=", 6);
    hm.insert("<=", 6);
    hm.insert(">", 6);
    hm.insert("<", 6);
    hm.insert("&&", 7);
    hm.insert("||", 8);
    hm
});

pub struct Parser<'a> {
    pub tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            tokenizer: Tokenizer::new(source),
        }
    }

    pub fn parse_var(&mut self) -> Variable<'a> {
        let name = self.tokenizer.expect(TokenType::Identifier);
        let type_ = self.parse_type();
        if self.tokenizer.optionally_expect_string("=") {
            let value = self.parse_expression(0);
            return Variable {
                name: name.value,
                type_,
                value: Some(value),
            };
        }
        return Variable {
            name: name.value,
            type_,
            value: None,
        };
    }


    pub fn parse_object_field(&mut self) -> Variable<'a> {
        let name = self.tokenizer.expect(TokenType::Identifier);
        self.tokenizer.expect_punctuation(':');
        let value = self.parse_expression(0);
        return Variable {
                name: name.value,
                type_: DataType::None,
                value: Some(value), 
            };
    }

    fn parse_expression_piece(&mut self) -> Expression<'a> {
        let position_at_start = self.tokenizer.index;

        let next_token = self.tokenizer.next();
        if next_token.is_none() {
            panic!("Unexpected end of file");
        }
        let next_token = next_token.unwrap();

        if next_token.type_ == TokenType::Punctuation{
            match next_token.value.as_str() {
                 "[" => {
                    self.tokenizer.index = position_at_start;
                    return Expression::Array(self.parse_array());
                }
                 "{" => {
                    self.tokenizer.index = position_at_start;
                    return Expression::Object(Object { name: ("anonymous".to_string()), 
                    fields: self.collect_custom_list(|parser| parser.parse_object_field(), '{', '}') });
                }
                 "(" => {
                    let expr = self.parse_expression(0);
                    self.tokenizer.expect_punctuation(')');
                    return expr;
                }
                "," => {
                     self.tokenizer.show_user_error(position_at_start, self.tokenizer.index, "did you mean to put another expression piece before the comma?".to_string());
                }
                _ => self.tokenizer.show_user_error(position_at_start, self.tokenizer.index, "don't know how to deal with the this punctuation char in this context".to_string()),
            }
        }

        if next_token.type_ == TokenType::Identifier {
            if let Some(peek) = self.tokenizer.peek() && peek.type_ == TokenType::Punctuation{
                    match peek.value.as_str() {
                        "(" => {
                            self.tokenizer.index = position_at_start;
                            let func_call = self.parse_function_call();
                            return Expression::FunctionCall(func_call);
                        }
                        "{" => {
                            self.tokenizer.index = position_at_start;
                            let struct_call = self.parse_object();
                            return Expression::Object(struct_call);
                        }
                        "[" => {
                            self.tokenizer.index = position_at_start;
                            let subscript = self.parse_subscript();
                            return Expression::Subscript(subscript);
                        }
                        _ => {}
                    }
            }
            return Expression::VarReference(crate::ast::structure::VarReference { name: next_token.value.clone(), referring_to: None });
        }

        Expression::Token(next_token)
    }


    fn parse_subscript(&mut self) -> Subscript<'a> {
        let name = self.tokenizer.expect(TokenType::Identifier);
        self.tokenizer.expect_punctuation('[');
        let arg = self.parse_expression(0);
        self.tokenizer.expect_punctuation(']');
        return Subscript {
            name: name.value,
            arg: Box::new(arg),
        };
    }

    fn collect_expression_list(
        &mut self,
        start_punctuation: char,
        end_punctuation: char,
    ) -> Vec<Expression<'a>> {
        let mut expression_list = Vec::new();
        self.tokenizer.expect_punctuation(start_punctuation);
        while !self
            .tokenizer
            .optionally_expect_punctuation(end_punctuation)
        {
            expression_list.push(self.parse_expression(0));
            println!("self.tokenizer.index: {}", self.tokenizer.index);
            if !self.tokenizer.optionally_expect_punctuation(',') {
                self.tokenizer.expect_punctuation(end_punctuation);
                break;
            }
        }
        expression_list
    }

    fn collect_custom_list<T, F: Fn(&mut Parser<'a>) -> T>(
        &mut self,
        parser_method: F,
        start_punctuation: char,
        end_punctuation: char,
    ) -> Vec<T> {
        let mut expression_list = Vec::new();
        self.tokenizer.expect_punctuation(start_punctuation);
        while !self
            .tokenizer
            .optionally_expect_punctuation(end_punctuation)
        {
            expression_list.push(parser_method(self));
            println!("self.tokenizer.index: {}", self.tokenizer.index);
            if !self.tokenizer.optionally_expect_punctuation(',') {
                self.tokenizer.expect_punctuation(end_punctuation);
                break;
            }
        }
        expression_list
    }

    fn parse_array(&mut self) -> Array<'a> {
        let elements = self.collect_expression_list('[', ']');
        return Array { elements };
    }

    fn collect_custom_list_without_comma<T, F: Fn(&mut Parser<'a>) -> T>(
        &mut self,
        parser_method: F,
        start_punctuation: char,
        end_punctuation: char,
    ) -> Vec<T> {
        let mut expression_list = Vec::new();
        self.tokenizer.expect_punctuation(start_punctuation);
        self.tokenizer.eat_lines();
        while !self
            .tokenizer
            .optionally_expect_punctuation(end_punctuation)
        {
            expression_list.push(parser_method(self));
            println!("self.tokenizer.index: {}", self.tokenizer.index);
            self.tokenizer.eat_lines();
        }
        expression_list
    }

    fn parse_function_call(&mut self) -> FunctionCall<'a> {
        let name = self.tokenizer.expect(TokenType::Identifier);
        let args = self.collect_expression_list('(', ')');
        return FunctionCall {
            name: name.value,
            args,
        };
    }

    pub fn parse_function_header(&mut self) -> (String, Vec<Variable<'a>>, DataType) {
        let name = self.tokenizer.expect(TokenType::Identifier);
        let args = self.collect_custom_list(|parser| parser.parse_var(), '(', ')');
        if self.tokenizer.optionally_expect_punctuation(':') {
            let return_type = self.parse_type();
            return (name.value, args, return_type);
        }
        let return_type = DataType::None;
        return (name.value, args, return_type);
    }

    fn parse_valid_in_function_body(&mut self) -> ValidInFunctionBody<'a> {
        if self.tokenizer.optionally_expect_keyword_of("var") {
            return ValidInFunctionBody::Variable(self.parse_var());
        }
        if self.tokenizer.optionally_expect_keyword_of("return") {
            return ValidInFunctionBody::Return(self.parse_expression(0));
        }
        return ValidInFunctionBody::Expression(self.parse_expression(0));
    }

    pub fn parse_function(&mut self) -> FunctionDef<'a> {
        let (name, args, return_type) = self.parse_function_header();
        let body = self.collect_custom_list_without_comma(
            |parser| parser.parse_valid_in_function_body(),
            '{',
            '}',
        );
        return FunctionDef {
            name,
            args,
            return_type,
            body,
        };
    }


    fn parse_field_or_method(&mut self) -> StructScopeItem<'a> {
        let start_pos = self.tokenizer.index;
        if self.tokenizer.optionally_expect_keyword_of("func") {
            return StructScopeItem::Method(self.parse_function());
        }
        if self.tokenizer.optionally_expect_type(TokenType::Identifier) && self.tokenizer.optionally_expect_punctuation('(') {
            self.tokenizer.index = start_pos;
            return StructScopeItem::Method(self.parse_function());
        }
        self.tokenizer.index = start_pos;
        return StructScopeItem::Field(self.parse_var());
    }

    pub fn parse_struct(&mut self) -> StructDef<'a> {
        let name_token = self.tokenizer.expect(TokenType::Identifier);
        let scope_items = self.collect_custom_list_without_comma(|parser| parser.parse_field_or_method(), '{', '}');
        let mut fields = vec![];
        let mut methods = vec![];
        for item in scope_items {
            match item {
                StructScopeItem::Field(field) => fields.push(field),
                StructScopeItem::Method(method) => methods.push(method),
            }
        }
        return StructDef {
            name: name_token.value,
            fields,
            methods,
        };
    }
    pub fn parse_object(&mut self) -> Object<'a> {
        let name_token = self.tokenizer.expect(TokenType::Identifier);
        let fields = self.collect_custom_list(|parser| parser.parse_object_field(), '{', '}');
        return Object {
            name: name_token.value,
            fields,
        };
    }
    pub fn parse_expression(&mut self, left_pull: u32) -> Expression<'a> {
        println!("[parse_expression] Starting with left_pull: {}", left_pull);
        let mut left: Expression = self.parse_expression_piece();
        println!("[parse_expression] Initial left: {:?}", left);

        while self.tokenizer.in_range() {
            let possibly_greater_precedence_operand = self.tokenizer.peek();
            if possibly_greater_precedence_operand.is_none() {
                break;
            }
            let possibly_greater_precedence_operand = possibly_greater_precedence_operand.unwrap();
            if possibly_greater_precedence_operand.type_ != TokenType::Operator {
                break;
            }
            if let Some(&precedence) =
                OPERATOR_PRECEDENCE.get(possibly_greater_precedence_operand.value.as_str())
            {
                if precedence > left_pull {
                    println!(
                        "[parse_expression] Found operator '{}' with precedence {} (needs > {})",
                        possibly_greater_precedence_operand.value, precedence, left_pull
                    );
                    self.tokenizer.next(); // Consume the operator

                    println!(
                        "[parse_expression] Parsing right hand side with precedence {}",
                        precedence
                    );
                    let right = self.parse_expression(precedence);

                    println!(
                        "[parse_expression] Creating OperatorUse: {} between {:?} and {:?}",
                        possibly_greater_precedence_operand.value, left, right
                    );
                    left = Expression::OperatorUse(OperatorUse {
                        operator: possibly_greater_precedence_operand.value.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    });
                    println!("[parse_expression] New left: {:?}", left);
                    continue;
                } else {
                    break;
                }
            }
        }

        println!("[parse_expression] Final expression: {:?}", left);
        return left;
    }
    fn parse_type(&mut self) -> DataType {
        let token = self.tokenizer.expect(TokenType::Identifier);
        type_from(token.value)
    }
}


