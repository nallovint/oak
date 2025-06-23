// Interpreter / AST Visitor
use std::collections::HashMap;

use super::parser::{
    Assign, BinOp, Comment, EvalMathExp, FunctionCall, Number, StringLiteral, Value, Var, Visitor,
};
use super::math::{get_math_functions, get_math_constants};

pub struct Interpreter {
    variables: HashMap<String, f64>,
    math_functions: HashMap<String, fn(f64) -> f64>,
    math_constants: HashMap<String, f64>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            math_functions: get_math_functions(),
            math_constants: get_math_constants(),
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
        // First check if it's a math constant
        if let Some(&constant_value) = self.math_constants.get(&node.name) {
            println!("Constante matemática '{}' = {}", node.name, constant_value);
            return Value::Number(constant_value);
        }
        
        // Then check if it's a variable
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
        
        // Check if it's a math function
        if let Some(&math_func) = self.math_functions.get(&node.name) {
            if node.args.len() != 1 {
                println!("Error: función '{}' requiere exactamente 1 argumento", node.name);
                return Value::None;
            }
            
            let arg = node.args[0].accept(self);
            if let Value::Number(x) = arg {
                let result = math_func(x);
                println!("Resultado de {}: {}", node.name, result);
                return Value::Number(result);
            } else {
                println!("Error: argumento de '{}' debe ser un número", node.name);
                return Value::None;
            }
        }
        
        // Handle other function calls (existing logic)
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
