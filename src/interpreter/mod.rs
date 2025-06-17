use std::collections::HashMap;

use super::parser::{
    Assign, BinOp, Comment, EvalMathExp, FunctionCall, Number, StringLiteral, Value, Var, Visitor,
};

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl Visitor for Interpreter {
    fn visit_eval_math_exp(&mut self, node: &EvalMathExp) -> Value {
        println!("Evaluando expresión matemática: {}", node.expr);
        Value::None
    }

    fn visit_bin_op(&mut self, node: &BinOp) -> Value {
        let left = node.left.accept(self);
        let right = node.right.accept(self);

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match node.op.as_str() {
                "+" => Value::Number(l + r),
                "-" => Value::Number(l - r),
                "*" => Value::Number(l * r),
                "/" => Value::Number(l / r),
                _ => {
                    println!("Operación desconocida: {}", node.op);
                    Value::None
                }
            },
            _ => {
                println!("Error de tipo en operación binaria");
                Value::None
            }
        }
    }

    fn visit_number(&mut self, node: &Number) -> Value {
        Value::Number(node.value)
    }

    fn visit_var(&mut self, node: &Var) -> Value {
        match self.variables.get(&node.name) {
            Some(val) => {
                println!("Variable '{}' = {}", node.name, val);
                Value::Number(*val)
            }
            None => {
                println!("Variable '{}' no definida", node.name);
                Value::None
            }
        }
    }

    fn visit_assign(&mut self, node: &Assign) -> Value {
        let val = node.expr.accept(self);
        if let Value::Number(num) = val {
            self.variables.insert(node.name.clone(), num);
            println!("Asignando a '{}' el valor {}", node.name, num);
            Value::Number(num)
        } else {
            println!("Asignación fallida para '{}'", node.name);
            Value::None
        }
    }

    fn visit_string_literal(&mut self, node: &StringLiteral) -> Value {
        println!("Cadena: \"{}\"", node.value);
        Value::String(node.value.clone())
    }

    fn visit_function_call(&mut self, node: &FunctionCall) -> Value {
        println!(
            "Llamada a función '{}', args: {}",
            node.name,
            node.args.len()
        );
        for arg in &node.args {
            arg.accept(self);
        }
        Value::None
    }

    fn visit_comment(&mut self, node: &Comment) -> Value {
        println!("Comentario: {}", node.value);
        Value::None
    }
}
