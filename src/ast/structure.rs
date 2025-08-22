use std::option;

use crate::{
    data_type::DataType,
    lexer::token::{Token, TokenType},
};

// ---- AST node types ----
#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub name: String,
    pub args: Vec<Expression<'a>>,
}

#[derive(Debug)]
pub struct Variable<'a> {
    pub name: String,
    pub type_: DataType,
    pub value: Option<Expression<'a>>,
}

#[derive(Debug)]
pub struct OperatorUse<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: String,
    pub right: Box<Expression<'a>>,
}





#[derive(Debug)]
pub struct VarReference<'a> {
    pub name: String,
    pub referring_to: Option<&'a Variable<'a>>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    OperatorUse(OperatorUse<'a>),
    Token(Token),
    FunctionCall(FunctionCall<'a>),
    VarReference(VarReference<'a>),
}




#[derive(Debug)]
pub enum ValidInFunctionBody<'a> {
    Variable(Variable<'a>),
    Expression(Expression<'a>),
    Return(Expression<'a>),
}

#[derive(Debug)]
pub struct FunctionDef<'a> {
    pub name: String,
    pub args: Vec<Variable<'a>>,
    pub return_type: DataType,
    pub body: Vec<ValidInFunctionBody<'a>>,
}




