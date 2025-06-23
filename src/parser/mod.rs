// Parser + AST Definitions
use regex::Error as RegexError;
use std::{fs::File, io::Read, result::Result};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    None,
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] RegexError),
}

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value;
}

pub struct EvalMathExp {
    pub expr: String,
}

impl EvalMathExp {
    pub fn parse(expr_tokens: Vec<&str>) -> Self {
        Self {
            expr: expr_tokens.join(" "),
        }
    }
}

impl Node for EvalMathExp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_eval_math_exp(self)
    }
}

pub struct BinOp {
    pub left: Box<dyn Node>,
    pub op: String,
    pub right: Box<dyn Node>,
}

impl BinOp {
    pub fn parse(left: Box<dyn Node>, op: String, right: Box<dyn Node>) -> Self {
        Self { left, op, right }
    }
}

impl Node for BinOp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_bin_op(self)
    }
}

pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn parse(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Node for Number {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_number(self)
    }
}

pub struct Var {
    pub name: String,
}

impl Var {
    pub fn parse(name: String) -> Self {
        Self { name }
    }
}

impl Node for Var {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_var(self)
    }
}

pub struct Assign {
    pub name: String,
    pub expr: Box<dyn Node>,
}

impl Assign {
    pub fn parse(name: String, expr: Box<dyn Node>) -> Self {
        Self { name, expr }
    }
}

impl Node for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_assign(self)
    }
}

pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_string_literal(self)
    }
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<dyn Node>>,
}

impl FunctionCall {
    pub fn parse(name: String, args: Vec<Box<dyn Node>>) -> Self {
        Self { name, args }
    }
}

impl Node for FunctionCall {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_function_call(self)
    }
}

pub struct Comment {
    pub value: String,
}

impl Comment {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for Comment {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_comment(self)
    }
}

pub trait Visitor {
    fn visit_eval_math_exp(&mut self, node: &EvalMathExp) -> Value;
    fn visit_bin_op(&mut self, node: &BinOp) -> Value;
    fn visit_number(&mut self, node: &Number) -> Value;
    fn visit_var(&mut self, node: &Var) -> Value;
    fn visit_assign(&mut self, node: &Assign) -> Value;
    fn visit_string_literal(&mut self, node: &StringLiteral) -> Value;
    fn visit_function_call(&mut self, node: &FunctionCall) -> Value;
    fn visit_comment(&mut self, node: &Comment) -> Value;
}

pub fn parse_script(source: String) -> Result<(), ScriptError> {
    use crate::tokenizer::tokenize;

    let mut file = File::open(source)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let tokens = tokenize(&content);

    for token in tokens {
        println!("Parsed token: {:?}", token);
    }

    Ok(())
}
