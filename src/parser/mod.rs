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
