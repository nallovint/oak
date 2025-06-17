// Permits nodes to return values from function output
#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    None,
}

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value;
}

// Evaluates math expressions in code
pub(crate) struct EvalMathExp {
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

// Parses binary operations
pub(crate) struct BinOp {
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

// Evaluates and parse numerical types
pub(crate) struct Number {
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

// Parses variables
pub(crate) struct Var {
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

// Evaluates variables and functions assignation
pub(crate) struct Assign {
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

// Parses strings
pub(crate) struct StringLiteral {
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

// Parses function calls
pub(crate) struct FunctionCall {
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

// Non-evaluative comment parsing
pub(crate) struct Comment {
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

// Visitor trait pattern for each node
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
