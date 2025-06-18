#[cfg(test)]
#[allow(warnings)]
#[test]
fn test_binary_operation() {
    use crate::{
        interpreter::Interpreter,
        parser::{Assign, BinOp, Node, Number, Value, Var},
    };

    let expr = BinOp::parse(
        Box::new(Number::parse("3")),
        "+".to_string(),
        Box::new(Number::parse("4")),
    );

    let assignment = Assign::parse("x".to_string(), Box::new(expr));
    let mut interpreter = Interpreter::new();

    assignment.accept(&mut interpreter);

    let var = Var::parse("x".to_string());
    let result = var.accept(&mut interpreter);

    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_runtime_script_parsing() {
    use crate::parser::parse_script;

    let script_source: String = "./test.oak".to_string();

    if let Err(_) = parse_script(script_source) {
        println!("Failed to assert the result of file parsing was ok!");
        std::process::exit(1);
    } else {
        println!("File parsing result was ok!");
        std::process::exit(0);
    }
}
