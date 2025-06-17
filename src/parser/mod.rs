trait Node {}

// Evaluates math expressions in code
struct EvalMathExp {
    expr: String,
}

impl EvalMathExp {
    fn parse(expr_tokens: Vec<&str>) -> Self {
        Self {
            expr: expr_tokens.join(" "),
        }
    }
}

impl Node for EvalMathExp {}

// Parses binary operations
struct BinOp {
    left: Box<dyn Node>,
    op: String,
    right: Box<dyn Node>,
}

impl BinOp {
    fn parse(left: Box<dyn Node>, op: String, right: Box<dyn Node>) -> Self {
        Self { left, op, right }
    }
}

impl Node for BinOp {}

// Evaluates and parse numerical types
struct Number {
    value: f64,
}

impl Number {
    fn parse(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Node for Number {}

// Parses variables
struct Var {
    name: String,
}

impl Var {
    fn parse(name: String) -> Self {
        Self { name }
    }
}

impl Node for Var {}

// Evaluates variables and functions assignation
struct Assign {
    name: String,
    expr: Box<dyn Node>,
}

impl Assign {
    fn parse(name: String, expr: Box<dyn Node>) -> Self {
        Self { name, expr }
    }
}

impl Node for Assign {}

// Parses strings
struct StringLiteral {
    value: String,
}

impl StringLiteral {
    fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteral {}

// Parses function calls
struct FunctionCall {
    name: String,
    args: Vec<Box<dyn Node>>,
}

impl FunctionCall {
    fn parse(name: String, args: Vec<Box<dyn Node>>) -> Self {
        Self { name, args }
    }
}

impl Node for FunctionCall {}

// Non-evaluative comment parsing
struct Comment {
    value: String,
}

impl Comment {
    fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for Comment {}
