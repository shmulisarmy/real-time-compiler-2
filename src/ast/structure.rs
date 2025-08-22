use crate::{
    data_type::DataType,
    lexer::token::Token,
};

// ---- AST node types ----
#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub type_: DataType,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct OperatorUse {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    OperatorUse(OperatorUse),
    Token(Token),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub enum ValidInFunctionBody {
    Variable(Variable),
    Expression(Expression),
    Return(Expression),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub args: Vec<Variable>,
    pub return_type: DataType,
    pub body: Vec<ValidInFunctionBody>,
}
